use std::sync::mpsc;

use anyhow::anyhow;
use clap::Parser;
use lipsum::lipsum;
use notify::Watcher;
use rand::Rng;
use wasmtime::{
    Engine, Store,
    component::{Component, HasSelf, Linker, bindgen},
};
use wasmtime_wasi::{
    ResourceTable,
    p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView},
};

mod cli;

bindgen!({
    path: "../wit"
});

struct HostState {
    wasi: WasiCtx,
    resources: ResourceTable,
    print_counter: u64,
}

impl HostState {
    pub fn new() -> Self {
        Self {
            wasi: WasiCtxBuilder::new().build(),
            resources: ResourceTable::new(),
            print_counter: 0,
        }
    }
}

impl WasiView for HostState {
    fn ctx(&mut self) -> &mut wasmtime_wasi::p2::WasiCtx {
        &mut self.wasi
    }
}

impl IoView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resources
    }
}

impl HostExtensionImports for HostState {
    fn print(&mut self, msg: String) {
        self.print_counter += 1;
        println!("{msg}");
    }
}

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(&args.path, notify::RecursiveMode::Recursive)?;
    let mut rng = rand::rng();

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);

    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    HostExtension::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;

    let mut state = Some(HostState::new());

    loop {
        let mut store = Store::new(&engine, state.take().ok_or(anyhow!("Missing host state!"))?);
        let component = Component::from_file(&engine, &args.path)?;
        let bindings = HostExtension::instantiate(&mut store, &component, &linker)?;

        loop {
            if let Ok(Ok(notify::Event {
                kind: notify::EventKind::Create(..) | notify::EventKind::Modify(..),
                ..
            })) = rx.try_recv()
            {
                println!("-- RELOADING PLUGIN: `{}`", args.path.display());
                break;
            }

            let arg = lipsum(rng.random_range(1..6));
            let res = bindings.call_run(&mut store, &arg)?;

            println!("{res:?}");
            println!("Print counter: {}", store.data().print_counter);

            std::thread::sleep(std::time::Duration::from_millis(args.sleep));
        }

        state = Some(store.into_data());
    }
}

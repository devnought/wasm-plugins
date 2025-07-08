# wasm-plugins
Simple exmaple of [wasmtime](https://github.com/bytecodealliance/wasmtime) embeded within a
host program, executing `component` plugins within a loop, and reloading the plugin if they
change on disk.

## host
Host program providing the execution environment for the component plugins:
```
cargo run -- --help
```
```
Usage: wasm-plugins.exe [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>    Wasm component plugin path
  -s, --sleep <SLEEP>  Sleep interval in milliseconds between plugin invocations [default: 1000]
  -h, --help           Print help
```

Based off the [wit](wit/host-extension.wit) definition, the host provides a single `print()`
function, and expects the component plugin to define a `run()` function.

## plugin-rust
A component plugin written in Rust. To build:
```
cargo build --package plugin-rust --target wasm32-wasip2 --release
```

## plugin-csharp
A component plugin written in C#. This project currently depends on the .NET 10 SDK. To build:
```
cd plugin-csharp
dotnet build -c Release
```

## Running the plugins
Within an infinite loop, the host program will generate a list of between 1 and 5 (inclusive)
Lorem Ipsum words, and pass the string to the plugin.

Each plugin invokes the host `print()` function to print a single message, splits the string
on whitespace characters, and returns the collection of individual strings.

The `print()` function increments a counter each time it is called to 
demonstrate host application state management, which is preserved
if a plugin is recompiled and reloaded at runtime.

Once one of the plugins are built, simply pass a path to the generated wasm file:
### Rust
```
cargo run -- --path target/wasm32-wasip2/release/plugin_rust.wasm

Message from Rust
["Lorem", "ipsum", "dolor", "sit", "amet."]
Print counter: 1
Message from Rust
["Lorem", "ipsum", "dolor."]
Print counter: 2
Message from Rust
["Lorem", "ipsum", "dolor", "sit."]
Print counter: 3
```
### C#
```
cargo run -- --path ./plugin-csharp/bin/Release/net10.0/wasi-wasm/native/plugin-csharp.wasm

Message from C#
["Lorem", "ipsum", "dolor."]
Print counter: 1
Message from C#
["Lorem", "ipsum."]
Print counter: 2
Message from C#
["Lorem", "ipsum", "dolor", "sit."]
Print counter: 3
```

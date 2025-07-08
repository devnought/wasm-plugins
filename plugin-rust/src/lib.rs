wit_bindgen::generate!(in "../wit");

struct Component;

impl Guest for Component {
    fn run(msg: String) -> Vec<String> {
        print("Message from Rust");
        msg.split(' ').map(String::from).collect()
    }
}

export!(Component);

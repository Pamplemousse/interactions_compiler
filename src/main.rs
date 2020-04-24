use std::io;


fn main() {
    let stdin = io::stdin();

    let source: serde_json::Value =
        serde_json::from_reader(stdin).expect("JSON was not well-formatted");

    println!("json input: {}", source);
}

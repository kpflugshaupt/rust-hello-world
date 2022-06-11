use std::env;

fn get_name() -> Option<String> {
    env::args().skip(1).next()
}

fn main() {
    let dflt = "my dear";
    let name_to_greet = get_name().unwrap_or_else(|| dflt.to_string());
    println!("Hello, {name_to_greet}!");
}

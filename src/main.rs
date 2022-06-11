use std::env;

fn get_name() -> String {
    env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "world".to_string())
}

fn main() {
    let name_to_greet = get_name();
    println!("Hello, {name_to_greet}!");
}

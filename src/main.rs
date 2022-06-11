fn main() {
    let name = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "world".to_string());

    println!("Hello, {name}!");
}

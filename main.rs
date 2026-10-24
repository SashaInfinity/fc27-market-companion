use std::fs;
fn main() {
    let content = fs::read_to_string("config.json").unwrap_or_default();
    println!("Loaded {} bytes", content.len());
}

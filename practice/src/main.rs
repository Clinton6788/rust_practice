mod ownership;
mod iters;
mod order_processor;
mod data_structures;
use ownership::ownership_pipeline;
use std::io;

fn main() {
    let name: String = cli_interface();
    println!("Returned: {name}");
    
    iters::iter_pipeline();
    ownership_pipeline();
    data_structures::data_structures_pipeline();
    order_processor::pattern_and_destructing();

}

fn cli_interface() -> String {
    let mut input = String::new();

    println!("Enter your name:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    clean_string(&mut input);
    println!("Hello, {}", input);

    return input;
}

// Removes leading/trailing whitespace, converts to lowercase
fn clean_string(raw: &mut String) {
    let cleaned = raw.trim().to_lowercase();
    raw.clear();
    raw.push_str(&cleaned);
}
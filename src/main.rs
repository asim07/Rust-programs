use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    println!("Enter some input:");
    stdin
        .lock()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_input = input.trim();

    println!("Trimmed input: {}", trimmed_input);
}

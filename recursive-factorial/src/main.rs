use recursive_factorial::{fact, parse_input};
use std::env;

fn main() {
    let input = env::args();
    match parse_input(input) {
        Ok(num) => println!("Factorial of {} is {}", num, fact(num)),
        Err(e) => {
            println!("ERROR: Can't parse input");
            println!("{e}");
            println!("\nUSAGE: fact int");
        }
    }
}

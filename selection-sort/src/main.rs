use selection_sort::*;
use std::env;
use std::process::exit;

fn main() {
    let input = parse_input(env::args());

    match input {
        Ok(mut list) => {
            let sorted_list = selection_sort(&mut list);
            match sorted_list {
                Ok(_) => println!("{:?}", list),
                Err(e) => {
                    println!("ERROR Unable to sort: {e}");
                    exit(1)
                }
            }
        }
        Err(e) => {
            println!("ERROR unable to parse input: {e}");
            println!("\nPlease use comma separated list of integers\n\nUSAGE: selection-sort int1,int2,int3,...");
            exit(1)
        }
    }
}

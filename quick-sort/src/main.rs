use quick_sort::*;
use std::env;
use std::process::exit;

fn main() {
    let input = parse_input(env::args());
    let mut steps = 0;

    match input {
        Ok(list) => {
            let sorted_list = qsort(&list, &mut steps);
            println!("Items count: {}", sorted_list.len());
            println!("List is ordered in {steps} steps");
            println!("{:?}", sorted_list);
        }
        Err(e) => {
            println!("ERROR unable to parse input: {e}");
            println!("\nPlease use comma separated list of integers\n\nUSAGE: quick-sort int1,int2,int3,...");
            exit(1)
        }
    }
}

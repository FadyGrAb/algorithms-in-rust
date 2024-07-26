use std::env;
use std::env::Args;
use std::process::exit;

fn selection_sort(items: &mut Vec<i32>) -> Result<(), String> {
    let len = items.len();
    if len == 0 {
        return Err("You didn't provide a list to sort".to_string());
    }

    let mut steps: usize = 0;

    for i in 0..len {
        let mut smallest_index = i;
        steps += 1;
        for j in (i + 1)..len {
            steps += 1;
            if items[j] < items[smallest_index] {
                smallest_index = j;
            }
        }
        items.swap(i, smallest_index)
    }
    println!("Items count: {}", len);
    println!("List is ordered in {} steps", steps);
    Ok(())
}

fn parse_input(input: Args) -> Result<Vec<i32>, String> {
    let list = input
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .trim_matches(',')
        .replace(" ", "");

    if list.is_empty() {
        return Err("Empty input".to_string());
    }

    println!("Input list {list}");

    list.split(',')
        .map(|item| {
            item.trim()
                .parse::<i32>()
                .map_err(|e| format!("{e}: {item}"))
        })
        .collect()
}

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

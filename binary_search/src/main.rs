use std::env;
use text_colorizer::*;

#[derive(Debug)]
struct Arg {
    list: Vec<i32>,
    target: i32,
}

fn main() {
    let args: Vec<i32> = env::args()
        .skip(1)
        .map(|a| a.parse::<i32>().expect("cant parse"))
        .collect();

    if args.len() < 2 {
        eprintln!("{} binary_search <target> <list>", "Usage".bold().green());
        std::process::exit(1);
    };

    let arguments = Arg {
        list: args[1..].into(),
        target: args[0],
    };

    match binary_search(&arguments) {
        Some(x) => println!("{}: Found {} at index {} in {} steps","RESULT".bold().green(), arguments.target, x.0, x.1),
        None => println!("Not found"),
        }

}

fn binary_search(args: &Arg) -> Option<(usize, i32)> {
    println!("Target: {}\nList: {:?}\nTotal Length: {}", args.target, args.list, args.list.len());
    println!("[] are the current boundaries\n<> is the mid point");
    println!("{:=<20}", "=");
    let mut low = 0;
    let mut high = args.list.len() - 1;
    let mut steps = 0;

while low <= high {
    steps += 1;
    let mid: usize = (low + high) / 2;
    println!("{}",format!("[STEP {}]", steps).bold().yellow());
    for (idx, num) in args.list.iter().enumerate() {
        match idx {
            x if ((x == low) || (x == high)) && (x != mid) => print!("[{num}] "),
            x if ((x == low) || (x == high)) && (x == mid) => print!("<[{num}]> "),
            x if x == mid => print!("<{num}> "),
            _ => print!("{num} "),
        }
    }
    println!("\n{:-<50}", "-");
    match mid {
        x if args.list[x] == args.target => {
                return Some((x, steps))
            },
            x if args.list[x] < args.target => low = x + 1,
            _ => if mid.checked_sub(1).is_none() {
                break;
            } else {
                high = mid - 1
            },
        };
    };

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arg1 = Arg {
            target: 20,
            list: vec![20, 23, 25, 30, 45, 50, 66],
        };

        let arg2 = Arg {
            target: 66,
            list: vec![20, 23, 25, 30, 45, 50, 66],
        };

        let arg3 = Arg {
            target: 25,
            list: vec![20, 23, 25, 30, 45, 50, 66],
        };

        let arg4 = Arg {
            target: 45,
            list: vec![20, 23, 25, 30, 45, 50, 66],
        };

        let arg5 = Arg {
            target: 10,
            list: vec![20, 23, 25, 30, 45, 50, 66],
        };
        let arg6 = Arg {
            target: 69,
            list: vec![20, 23, 25, 30, 45, 50, 66],
        };

        assert_eq!(binary_search(&arg1).unwrap().0, 0);
        assert_eq!(binary_search(&arg2).unwrap().0, 6);
        assert_eq!(binary_search(&arg3).unwrap().0, 2);
        assert_eq!(binary_search(&arg4).unwrap().0, 4);
        assert_eq!(binary_search(&arg5), None);
        assert_eq!(binary_search(&arg6), None);
    }
}

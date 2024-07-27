//! # Factorial using recursion
//! This is a demonstration of recursion using Rust. It's a simple CLI tool that takes one Integer and prints out its factorial.
//!
//! ## Examples
//! ```bash
//! $ recursive-factorial 6
//!
//! Factorial of 6 is 720
//! ```

/// # The main factorial function
/// This function takes one `u64` and returns its factorial using recursion.
/// - The base case is when the `number == 0` (!0 = 1).
/// - The recursive case is `number * fact(number - 1)`.
///
/// ## Examples:
/// ```
/// use recursive_factorial::fact;
///
/// let num = 5_u64;
/// assert_eq!(fact(num), 120_u64)
/// ```
pub fn fact(num: u64) -> u64 {
    if num == 0 {
        1
    } else {
        num * fact(num - 1)
    }
}

/// # Parsing input from stdin
/// This function takes in an `Iterator` and handles input errors.
/// It makes sure that the input can be parsed to `u64` to serve as an input for the `fact` function.
///
/// ## Examples
/// ```
/// use recursive_factorial::parse_input;
///
/// let args = vec!["name", "6"]
///             .into_iter()
///             .map(|item| item.to_string());
///
/// assert_eq!(parse_input(args), Ok(6_u64))
/// ```
pub fn parse_input<T>(input: T) -> Result<u64, String>
where
    T: Iterator<Item = String>,
{
    let num = input.skip(1).collect::<Vec<String>>();

    if num.is_empty() {
        return Err("Empty Input".to_string());
    } else if num.len() > 1 {
        return Err("Too many args".to_string());
    }

    num[0]
        .parse::<u64>()
        .map_err(|e| format!("{} :{}", e, num[0]))
}

#[cfg(test)]
mod fact_tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let arg1 = ["name", "10"];
        let arg2 = ["name", "10", "11"];
        let arg3 = ["name", "10.0"];
        let arg4 = ["name", "-10"];

        assert_eq!(
            parse_input(arg1.into_iter().map(|item| item.to_string())),
            Ok(10u64)
        );
        assert!(parse_input(arg2.into_iter().map(|item| item.to_string())).is_err());
        assert!(parse_input(arg3.into_iter().map(|item| item.to_string())).is_err());
        assert!(parse_input(arg4.into_iter().map(|item| item.to_string())).is_err());
    }

    #[test]
    fn test_fact() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(2), 2);
        assert_eq!(fact(5), 120);
    }
}

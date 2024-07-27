//! # Implementation of the quick sort algorithm
//! This is a simple implementation in Rust for the quick sort algorithm.
//! It receives a list of comma separated integer list, tries to parse it and sort it.
//! The CLI tries to clean up the input like removing extra white spaces, extra commas and
//! outputs error messages if it receives anything but integers.
//!
//! ## Example
//!
//! ```sh
//! $ quick-sort 4, 5,1, 3,10,8, 9,
//!
//! Input list 4,5,1,3,10,8,9
//! Items count: 7
//! [1, 3, 4, 5, 8, 9, 10]
//! ```

use rand::seq::SliceRandom;
use rand::thread_rng;

///# The sorting function
/// This function takes in a reference to vector of `i32` and returns a sorted `Vec<i32>` using the **quick sort** algorithm.  
/// The pivot is selected randomly using the `rand` crate. It also takes in a `mut usize` to calculate the number of steps taken to sort the list.
///
/// ## Examples
///
/// ```
/// use quick_sort::qsort;
///
/// let list = vec![3, 4, 1, 5, 7];
/// let mut steps: usize = 0;
/// let list_sorted = qsort(&list, &mut steps);
/// println!("Number of steps: {steps}");
/// assert_eq!(list_sorted, vec![1, 3, 4, 5, 7])
/// ```
pub fn qsort(array: &Vec<i32>, steps: &mut usize) -> Vec<i32> {
    let mut rng = thread_rng();

    *steps += 1;

    if array.len() < 2 {
        array.clone()
    } else {
        let pivot = array.choose(&mut rng).unwrap();

        let less = array
            .iter()
            .filter(|item| *item < pivot)
            .cloned()
            .collect::<Vec<i32>>();

        let greater = array
            .iter()
            .filter(|item| *item > pivot)
            .cloned()
            .collect::<Vec<i32>>();

        [qsort(&less, steps), vec![*pivot], qsort(&greater, steps)].concat()
    }
}

/// # Parsing input from stdin args
/// This a generic function that receives a type that implements **Iterator<Item = String>** trait
/// and try to parse a **`Vec<i32>`** from it.
///
/// ## Examples:
/// ```
/// use quick_sort::parse_input;
///
/// let args = vec!["name", "1,2,3,4"]
///                 .into_iter()
///                 .map(|item| item.to_string())
///                 .collect::<Vec<String>>();
/// assert_eq!(parse_input(args.into_iter()), Ok(vec![1, 2, 3, 4]))
/// ```
pub fn parse_input<T>(input: T) -> Result<Vec<i32>, String>
where
    T: Iterator<Item = String>,
{
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

#[cfg(test)]
mod quick_sort_sort_tests {
    use super::parse_input;
    use super::qsort;

    #[test]
    fn test_quick_sort() {
        let list = vec![3, 4, 1, 5, 7];
        let mut steps = 0;
        let list_sorted = qsort(&list, &mut steps);
        assert_eq!(list_sorted, vec![1, 3, 4, 5, 7]);
    }

    #[test]
    fn test_pars_input() {
        let args1 = vec!["name", "5,4,2,1,3"]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        let args2 = vec!["name", "5,", "4,", "2,", "1,", "3"]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        let args3 = vec!["name", "5,4,", "2,", "1,", "3"]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        let args4 = vec!["name", "5,4,", "2,", "1,", "3,"]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        let args5 = vec!["name", "5,4,", "text,", "1,", "3"]
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        for case in vec![args1, args2, args3, args4] {
            assert_eq!(parse_input(case.into_iter()), Ok(vec![5, 4, 2, 1, 3]))
        }

        assert!(parse_input(args5.into_iter()).is_err())
    }
}

//! # Implementation of the selection sort algorithm
//! This is a simple implementation in Rust for the selection sort algorithm

///# The sorting function
/// This function takes in a mutable vector of i32 and sort it in place using the **selection sort** algorithm.
///
/// ## Examples
///
/// ```
/// use selection_sort::selection_sort;
///
/// let mut list = vec![3, 4, 1, 5, 7];
/// let _ = selection_sort(&mut list);
/// assert_eq!(list, vec![1, 3, 4, 5, 7])
/// ```
pub fn selection_sort(items: &mut Vec<i32>) -> Result<(), String> {
    // Main selection sort algorithm implementation
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

/// # Parsing input from stdin args
/// This a generic function that receives a type that implements **Iterator<Item = String>** trait
/// and try to parse a **`Vec<i32>`** from it.
///
/// ## Examples:
/// ```
/// use selection_sort::parse_input;
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
mod selection_sort_tests {
    use super::parse_input;
    use super::selection_sort;

    #[test]
    fn test_selection_sort() {
        let mut list = vec![3, 4, 1, 5, 7];
        let _ = selection_sort(&mut list);
        assert_eq!(list, vec![1, 3, 4, 5, 7]);
        assert!(selection_sort(&mut vec![]).is_err())
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

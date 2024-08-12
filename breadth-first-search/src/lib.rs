//! # Breadth-First Search
//! A module to demonstrate Breadth-first Search (BFS) in Rust
//!

use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

pub struct Args {
    pub maze_size: usize,
    pub block_prob: f32,
}

pub fn parse_input() -> Result<Args, String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 2 {
        return Err("You must enter both the maze size and the block probability".to_string());
    }

    Ok(Args {
        maze_size: args[0]
            .parse::<usize>()
            .map_err(|err| err.to_string())
            .unwrap(),
        block_prob: args[1]
            .parse::<f32>()
            .map_err(|err| err.to_string())
            .unwrap(),
    })
}

/// # Generates a square maze
/// Generates a square maze of size x size of random zeros and ones.  
/// 0: means an open path.  
/// 1: means a blocked path.
/// You can set the probability of cell to be opened or blocked with `block_prob` which is an `f32` between (0.0, 1.0) inclusive.  
/// `0.0` value means all the maze will be opened and a `1.0` means that all the maze will be blocked.
/// ## Examples
/// ```
/// use breadth_first_search::generate_maze;
///
/// let maze = generate_maze(3, 0.5);
///
/// assert_eq!(maze.len(), 3);
/// assert_eq!(maze[0].len(), 3);
///
/// ```
pub fn generate_maze(size: usize, block_prob: f32) -> Vec<Vec<u8>> {
    let mut maze = vec![vec![0u8; size]; size];

    // Generate random bits
    for row in 0..size {
        for col in 0..size {
            maze[row][col] = get_random_bit(block_prob);
        }
    }

    // Set the top left and bottom right bits to 0 (open)
    maze[0][0] = 0;
    maze[size - 1][size - 1] = 0;

    maze
}

fn get_random_bit(block_prob: f32) -> u8 {
    let mut rng = thread_rng();

    [(0, 1.0 - block_prob), (1, block_prob)]
        .choose_weighted(&mut rng, |item| item.1)
        .unwrap()
        .0
}

pub fn bfs(maze: &Vec<Vec<u8>>) -> Option<Vec<((usize, usize), char)>> {
    let size = maze.len();

    let directions = [([0, 1], '-'), ([0, -1], '-'), ([-1, 0], '|'), ([1, 0], '|')];

    // check queue with first element is the top left point
    let mut queue = VecDeque::new();
    queue.push_back([0, 0]);

    // the visited list. the top left point is always the starting point so it's always visited.
    let mut visited = vec![vec![false; size]; size];
    visited[0][0] = true;

    // parent points to track the path
    let mut parents: Vec<Vec<Option<((usize, usize), char)>>> = vec![vec![None; size]; size];

    while let Some([row, col]) = queue.pop_front() {
        // if the end of the maze is reached
        if row == size - 1 && col == size - 1 {
            let mut current_point = (row, col);
            let mut path = vec![];

            while let Some(p) = parents[current_point.0][current_point.1] {
                path.push(p.to_owned());
                current_point = p.0;
            }
            path.last_mut().unwrap().1 = '*';
            path.reverse();
            path.push(((size - 1, size - 1), '*'));
            return Some(path);
        }
        for ([delta_row, delta_col], symbol) in &directions {
            let next_row = row as i32 + delta_row;
            let next_col = col as i32 + delta_col;

            if (next_row >= 0 && next_row < size as i32)
                && (next_col >= 0 && next_col < size as i32)
            {
                let (next_row, next_col) = (next_row as usize, next_col as usize);
                if maze[next_row][next_col] == 0 && !visited[next_row][next_col] {
                    visited[next_row][next_col] = true;
                    parents[next_row][next_col] = Some(((row, col), *symbol));
                    queue.push_back([next_row, next_col])
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod bfs_tests {
    use super::*;

    #[test]
    fn test_generate_maze() {
        let mut maze = generate_maze(3, 1.0);

        assert_eq!(maze.len(), maze[0].len()); // Square maze
        assert_eq!(maze.len(), 3); // rows == size (3)
        assert_eq!(maze[0][0], 0); // top left is 0
        assert_eq!(maze[2][2], 0); // bottom right is 0

        // Change back top left and bottom right cells to 1.
        maze[0][0] = 1;
        maze[2][2] = 1;

        assert!(maze.into_iter().flatten().all(|item| item == 1)) // All maze should be ones as block probability is 1.
    }
}

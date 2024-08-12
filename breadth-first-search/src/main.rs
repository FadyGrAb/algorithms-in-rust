use bfs_maze::*;

fn main() {
    let input = parse_input();
    match input {
        Ok(args) => {
            let maze = generate_maze(args.maze_size, args.block_prob);
            println!(
                "Generating a maze of {} x {} size and with {:.0}% probability of blocked cells",
                args.maze_size,
                args.maze_size,
                args.block_prob * 100.0
            );
            for row in &maze {
                for point in row {
                    print!("{} ", point)
                }
                println!()
            }

            println!("\nTrying to find the shortest path to the end ... \n");

            match bfs(&maze) {
                Some(path) => {
                    for x in 0..args.maze_size {
                        for y in 0..args.maze_size {
                            let matched_points = path
                                .iter()
                                .filter(|item| item.0 .0 == x && item.0 .1 == y)
                                .collect::<Vec<_>>();
                            if matched_points.is_empty() {
                                print!("{} ", maze[x][y])
                            } else {
                                print!("{} ", matched_points[0].1)
                            }
                        }
                        println!()
                    }
                    println!("You can reach the end in {} moves", path.len() - 1)
                }
                None => println!("There is no path from start to end!"),
            }
        }
        Err(err) => {
            println!("An Error has happened:\n{err}");
            println!("\nUSAGE: bfs-maze <maze size> <block probability>");
            println!("EXAMPLE: bfs-maze 10 0.2")
        }
    }
}

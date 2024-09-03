## Breadth first search
Breadth first search is a graph algorithm that is used to find the shortest path or search for a node in a Graph.
### Rust implementation
This algorithm is demonstrated as a CLI tool that accepts two arguments, the maze size and probability of 1s. This CLI generates a maze of `0`s and `1`s where all cells contains `0`, are ok to step into, while cells containing `1` are blocked. You can control how many cells are blocked by the `blocked prob` param. Meaning a blocked prob of 0.25, means that 25% of the maze's cells will be blocked. The tool them uses *BFS* to try to find the shortest path from the top left corner to the bottom right corner and plots it.
#### Example
If you have compiled the Rust code, then you can type in the following in the `./target` directory whether in `debug` or `release` according to your compilation.
```bash
bfs-maze 10 0.2
```
In this example, the tool will generate a 10x10 maze with 20% of blocked cells.  
Or you can use `cargo` directly as follows in the project's root:
```bash
cargo run -- 10 0.2
```
The output will vary at each run but it will resemble the following:
```bash
Generating a maze of 10 x 10 size and with 20% probability of blocked cells
0 1 1 0 0 1 0 0 1 0 
0 0 0 0 0 0 0 0 0 0 
1 0 0 0 1 0 0 0 0 1 
1 0 0 0 1 1 0 1 0 0 
1 0 0 0 0 0 1 0 1 0 
0 0 0 0 1 0 0 0 0 1 
0 0 0 0 0 0 0 0 0 0 
1 0 0 1 1 0 0 0 0 0 
0 1 0 0 0 0 1 0 0 0 
0 0 0 1 1 0 0 0 0 0 

Trying to find the shortest path to the end ... 

* 1 1 0 0 1 0 0 1 0 
- - - | 0 0 0 0 0 0 
1 0 0 | 1 0 0 0 0 1 
1 0 0 | 1 1 0 1 0 0 
1 0 0 - - | 1 0 1 0 
0 0 0 0 1 - - - | 1 
0 0 0 0 0 0 0 0 - | 
1 0 0 1 1 0 0 0 0 | 
0 1 0 0 0 0 1 0 0 | 
0 0 0 1 1 0 0 0 0 * 
You can reach the end in 18 moves
```
#### Tests and Docs
You can use `cargo` to run the tests and compile the docs as follows:
```bash
cargo test
cargo doc
```

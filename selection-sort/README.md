## Selection Sort
Selection sort is a sorting algorithm with $O(n^2)$ operations. It's slower than *Quick Sort* algorithm but simpler.
### Rust implementation
This algorithm is demonstrated as a CLI tool that accepts a comma separated list of integers and it sorts it. The logic is in the `lib.rs` while the `main.rs` is the "driver". The CLI tool tries to correctly parse the input and ignores extra spaces.
#### Example
If you have compiled the Rust code, then you can type in the following in the `./target` directory whether in `debug` or `release` according to your compilation.
```bash
selection-sort 12, 13,15,1, 3,4,  1,6
```
In this example, `12, 13,15,1, 3,4,  1,6` is the list to be ordered (note the inconsistent spaces)  
Or you can use `cargo` directly as follows in the project's root:
```bash
cargo run -- 12, 13,15,1, 3,4,  1,6
```
The output in both cases is:
```bash
Input list 12,13,15,1,3,4,1,6
Items count: 8
List is ordered in 36 steps
[1, 1, 3, 4, 6, 12, 13, 15]
```
#### Tests and Docs
You can use `cargo` to run the tests and compile the docs as follows:
```bash
cargo test
cargo doc
```

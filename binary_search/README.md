## Binary Search
Binary Search is a fast search algorithm with $O(\log n)$ operations where $n$ is the number of the list elements. The algorithms works only on *sorted* lists.
### Rust implementation
This algorithm is demonstrated as a CLI tool that accepts series of inputs and considers the first one as the *Target* and the other, as the *Ordered List*. The args are parsed without the need for a separator (i.e. no need for commas or hyphens, spaces will suffice).
#### Example
If you have compiled the Rust code, then you can type in the following in the `./target` directory whether in `debug` or `release` according to your compilation.
```bash
binary_search 10 1 2 3 5 8 10 11 15 39
```
In this example, `10` is the *target* and `1 2 3 5 8 10 11 15 39` is the *ordered list*.  
Or you can use `cargo` directly as follows in the project's root:
```bash
cargo run -- 10 1 2 3 5 8 10 11 15 39
```
The output in both cases is:
```bash
Target: 10
List: [1, 2, 3, 5, 8, 10, 11, 15, 39]
Total Length: 9
[] are the current boundaries
<> is the mid point
====================
[STEP 1]
[1] 2 3 5 <8> 10 11 15 [39] 
--------------------------------------------------
[STEP 2]
1 2 3 5 8 [10] <11> 15 [39] 
--------------------------------------------------
[STEP 3]
1 2 3 5 8 <[10]> 11 15 39 
--------------------------------------------------
RESULT: Found 10 at index 5 in 3 steps
```
It visually demonstrates each step until the target is found. 

#### Test
You can use `cargo` to run the tests as follows:
```bash
cargo test
```

## Recursion
This isn't an algorithm per-se, but this is a CS general concept. It's used in the *Quick Sort* algorithm in this repo.
### Rust implementation
To demonstrate *recursion*, this implementation uses the mathematical *factorial*. It's a CLI tool that accepts an integer and calculates its factorial. The logic is in the `lib.rs` while the `main.rs` is the "driver".
#### Example
If you have compiled the Rust code, then you can type in the following in the `./target` directory whether in `debug` or `release` according to your compilation.
```bash
recursive-factorial 6
```
In this example, `6` is number to calculate the factorial for.  
Or you can use `cargo` directly as follows in the project's root:
```bash
cargo run -- 6
```
The output in both cases is:
```bash
Factorial of 6 is 720
```
Factorial of `6` is `1 x 2 x 3 x 4 x 5 x 6` = `720`
#### Tests and Docs
You can use `cargo` to run the tests and compile the docs as follows:
```bash
cargo test
cargo doc
```

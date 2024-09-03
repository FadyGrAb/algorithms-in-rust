## Traveling Salesman
Traveling Salesman is a classic problem where you have a "salesman" that needs to travel through a fixed number of cities and he must choose the path shortest path between all cities. The start and end don't matter as long as he/she visits all the cities once. This is the greedy approach and it has $O(n!)$ complexity where $n$ is the number of cities.
### Rust implementation
The problem is demonstrated by hardcoding the cities and there coordinates and the output is the shortest path. This is the "Greedy" approach and the implementation calculates each and every path available.
#### Example
If you have compiled the Rust code, then you can type in the following in the `./target` directory whether in `debug` or `release` according to your compilation.
```bash
travelling-salesman
```
Or you can use `cargo` directly as follows in the project's root:
```bash
cargo run
```
The output in both cases is:
```bash
>>>>>>> [[Egypt], [Sudan], [Lybia], [UAE], [Iran]] 78.42458097267891
>>>>>>> [[Egypt], [Sudan], [Lybia], [Iran], [UAE]] 78.65483931059939
>>>>>>> [[Egypt], [Sudan], [UAE], [Lybia], [Iran]] 113.55330889773981
.
.
.
>>>>>>> [[Iran], [UAE], [Lybia], [Egypt], [Sudan]] 73.29316643552079
>>>>>>> [[Iran], [UAE], [Lybia], [Sudan], [Egypt]] 78.42458097267892
SELECTED: [[Iran], [UAE], [Sudan], [Egypt], [Lybia]] 62.44110298280657 in 120 steps
```

# Advent Of Code 2024 in Rust

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

My solutions to the [Advent of Code 2024](https://adventofcode.com/2024) challenges written in Rust.

Each day has a folder with the solutions for that day.

## Benchmarking

Benchmarked on a Victus with 20 × 12th Gen Intel® Core™ i7-12700H and 32 GB of RAM.  
Bench marking is done by iterating the released rust binary 100 times, removing outliers, and getting the sorted
median.

The code for benchmarking is in [**benchmark.py**](./benchmark.py). Saves the results and execution times as bar-graph
and box-plot.  
Results - [**results.txt**](./results.txt)
Bar Graph - [**executionTimesBar.png**](./executionTimesBar.png)  
Box Plot - [**executionTimesBox.png**](./executionTimesBox.png).

|                               |  Part 1   |  Part 2   |
|:-----------------------------:|:---------:|:---------:|
|  [Day 1](./day1/src/main.rs)  | 136.231µs | 146.004µs |
|  [Day 2](./day2/src/main.rs)  | 235.518µs | 326.648µs |
|  [Day 3](./day3/src/main.rs)  | 456.810µs |  6.428ms  |
|  [Day 4](./day4/src/main.rs)  | 481.475µs | 209.835µs |
|  [Day 5](./day5/src/main.rs)  |  7.345ms  | 11.890ms  |
|  [Day 6](./day6/src/main.rs)  | 238.012µs | 16.555ms  |
|  [Day 7](./day7/src/main.rs)  |  1.538ms  |  5.414ms  |
|  [Day 8](./day8/src/main.rs)  | 79.195µs  | 119.705µs |
|  [Day 9](./day9/src/main.rs)  |  1.374ms  | 27.478ms  |
| [Day 10](./day10/src/main.rs) |  1.081ms  | 669.132µs |
| [Day 11](./day11/src/main.rs) | 444.993µs | 12.272ms  |
| [Day 12](./day12/src/main.rs) |  1.527ms  |  3.429ms  |
| [Day 13](./day13/src/main.rs) | 391.282µs | 383.308µs |
| [Day 14](./day14/src/main.rs) | 204.585µs | 28.841ms  |
| [Day 15](./day15/src/main.rs) | 771.976µs |  1.092ms  |
| [Day 16](./day16/src/main.rs) |  1.092ms  |  5.887ms  |
| [Day 17](./day17/src/main.rs) |  9.330µs  | 15.935µs  |
| [Day 18](./day18/src/main.rs) |  2.607ms  | 19.335ms  |
| [Day 19](./day19/src/main.rs) |  1.509ms  |  1.417ms  |
| [Day 20](./day20/src/main.rs) |  1.628ms  | 10.205ms  |
| [Day 21](./day21/src/main.rs) | 21.091µs  | 137.173µs |
| [Day 22](./day22/src/main.rs) | 14.531ms  | 41.554ms  |
| [Day 23](./day23/src/main.rs) |  1.734ms  |  1.648ms  |
| [Day 24](./day24/src/main.rs) | 164.410µs | 400.493µs |
| [Day 25](./day25/src/main.rs) |  1.174ms  | 62.000ns  |

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
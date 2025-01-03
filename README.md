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
|  [Day 1](./day1/src/main.rs)  | 81.541µs  | 69.906µs  |
|  [Day 2](./day2/src/main.rs)  | 136.327µs | 183.412µs |
|  [Day 3](./day3/src/main.rs)  | 201.526µs |  4.412ms  |
|  [Day 4](./day4/src/main.rs)  | 365.498µs | 107.420µs |
|  [Day 5](./day5/src/main.rs)  |  3.748ms  |  7.403ms  |
|  [Day 6](./day6/src/main.rs)  | 74.768µs  | 10.974ms  |
|  [Day 7](./day7/src/main.rs)  | 918.669µs |  3.324ms  |
|  [Day 8](./day8/src/main.rs)  | 33.014µs  | 53.580µs  |
|  [Day 9](./day9/src/main.rs)  | 571.484µs | 12.746ms  |
| [Day 10](./day10/src/main.rs) | 864.687µs | 552.174µs |
| [Day 11](./day11/src/main.rs) | 385.989µs |  5.571ms  |
| [Day 12](./day12/src/main.rs) |  1.232ms  |  1.688ms  |
| [Day 13](./day13/src/main.rs) | 174.226µs | 332.396µs |
| [Day 14](./day14/src/main.rs) | 185.673µs | 14.634ms  |
| [Day 15](./day15/src/main.rs) | 243.289µs | 364.136µs |
| [Day 16](./day16/src/main.rs) | 564.812µs |  5.921ms  |
| [Day 17](./day17/src/main.rs) |  3.950µs  | 16.535µs  |
| [Day 18](./day18/src/main.rs) |  1.812ms  |  8.533ms  |
| [Day 19](./day19/src/main.rs) |  1.025ms  |  1.228ms  |
| [Day 20](./day20/src/main.rs) | 659.363µs |  4.657ms  |
| [Day 21](./day21/src/main.rs) |  5.279µs  | 42.184µs  |
| [Day 22](./day22/src/main.rs) |  1.898ms  |  9.776ms  |
| [Day 23](./day23/src/main.rs) |  1.464ms  |  1.091ms  |
| [Day 24](./day24/src/main.rs) | 50.569µs  | 127.661µs |
| [Day 25](./day25/src/main.rs) | 346.784µs |  0.000ns  |

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
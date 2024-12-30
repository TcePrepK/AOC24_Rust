# Advent Of Code 2024 in Rust

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

My solutions to the [Advent of Code 2024](https://adventofcode.com/2024) challenges written in Rust.

Each day has a folder with the solutions for that day.

## Benchmarks

Benchmarked on a Victus with 20 × 12th Gen Intel® Core™ i7-12700H and 32 GB of RAM.  
Bench marking is done by iterating the released rust binary 100 times, removing outliers, and getting the sorted
median.  
The code for benchmarking is in [**benchmark.py**](./benchmark.py), also can see execution times as box-plot in
[**ExecutionTimes.png**](./ExecutionTimes.png).

`???` For not solved days!  
`---` For not benchmarkable days!

|                               |  Part 1   |  Part 2   |
|:-----------------------------:|:---------:|:---------:|
|  [Day 1](./day1/src/main.rs)  | 146.453µs | 143.836µs |
|  [Day 2](./day2/src/main.rs)  | 248.920µs | 360.482µs |
|  [Day 3](./day3/src/main.rs)  | 479.261µs |  6.497ms  |
|  [Day 4](./day4/src/main.rs)  | 559.377µs | 237.285µs |
|  [Day 5](./day5/src/main.rs)  |  6.227ms  |  8.751ms  |
|  [Day 6](./day6/src/main.rs)  | 120.531µs | 53.806ms  |
|  [Day 7](./day7/src/main.rs)  |  1.198ms  |  4.591ms  |
|  [Day 8](./day8/src/main.rs)  | 52.981µs  | 78.335µs  |
|  [Day 9](./day9/src/main.rs)  | 908.505µs | 20.230ms  |
| [Day 10](./day10/src/main.rs) | 799.292µs | 462.095µs |
| [Day 11](./day11/src/main.rs) | 378.809µs | 65.883ms  |
| [Day 12](./day12/src/main.rs) |  1.237ms  |  2.941ms  |
| [Day 13](./day13/src/main.rs) | 297.899µs | 288.994µs |
| [Day 14](./day14/src/main.rs) | 152.921µs |    ---    |
| [Day 15](./day15/src/main.rs) | 495.024µs | 721.059µs |
| [Day 16](./day16/src/main.rs) |  3.503ms  | 28.930ms  |
| [Day 17](./day17/src/main.rs) |  4.689µs  | 10.741µs  |
| [Day 18](./day18/src/main.rs) |  1.895ms  | 17.676ms  |
| [Day 19](./day19/src/main.rs) | 48.682ms  | 46.596ms  |
| [Day 20](./day20/src/main.rs) | 760.122µs | 47.011ms  |
| [Day 21](./day21/src/main.rs) | 13.387µs  | 87.894µs  |
| [Day 22](./day22/src/main.rs) |  8.825ms  | 24.201ms  |
| [Day 23](./day23/src/main.rs) |  1.155ms  |  1.393ms  |
| [Day 24](./day24/src/main.rs) | 114.883µs | 274.001µs |
| [Day 25](./day25/src/main.rs) |  1.026ms  | 73.000ns  |

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
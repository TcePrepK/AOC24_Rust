use std::fs;
use std::ops::{Add, Mul, Rem};

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i32 = 21;
const ANSWER_TWO: i32 = 0;

fn test_examples() -> [bool; 2] {
    let example = read_file("src/example");

    let results = [first_part(&example), 0];

    if results[0] > 0 && results[0] != ANSWER_ONE {
        println!("Part One Wrong");
    }

    if results[1] > 0 && results[1] != ANSWER_TWO {
        println!("Part Two Wrong");
    }

    [results[0] == ANSWER_ONE, results[1] == ANSWER_TWO]
}

fn test_inputs(example_solutions: [bool; 2]) {
    let input = read_file("src/input");

    if example_solutions[0] {
        let start_time = std::time::Instant::now();
        let result = first_part(&input);
        let total_time = start_time.elapsed();
        println!("Part 1 result: {}, took: {:?}", result, total_time);
    }
    if example_solutions[1] {
        let start_time = std::time::Instant::now();
        let result = second_part(&input);
        let total_time = start_time.elapsed();
        println!("Part 2 result: {}, took: {:?}", result, total_time);
    }
}

fn main() {
    let example_solutions = test_examples();
    test_inputs(example_solutions);
}

/* ------------------- Helpers ------------------- */

#[derive(Debug, Clone, Copy)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl<T: Rem<Output = T>> Rem<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn rem(self, v: Self) -> Self::Output {
        Self {
            x: self.x % v.x,
            y: self.y % v.y,
        }
    }
}

macro_rules! vec2_mul {
    ( $lhs:ty , $rhs:ty ) => {
        impl Mul<$rhs> for $lhs {
            type Output = Vec2<$rhs>;
            fn mul(self, rhs: $rhs) -> Vec2<$rhs> {
                Vec2 {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }
    };
}

vec2_mul!(Vec2<i32>, i32);
vec2_mul!(&Vec2<i32>, i32);

#[derive(Debug)]
struct Robot {
    position: Vec2<i32>,
    velocity: Vec2<i32>,
}

/// The input consists of several lines of what could be parsed to a Robot
/// Reads and parses each line to a robot struct with position/velocity defined.
fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = vec![];

    for line in input.lines() {
        let data: Vec<Vec<i32>> = line
            .split_whitespace()
            .map(|part| {
                part.split_at(2)
                    .1
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let robot = Robot {
            position: Vec2 {
                x: data[0][0],
                y: data[0][1],
            },
            velocity: Vec2 {
                x: data[1][0],
                y: data[1][1],
            },
        };

        robots.push(robot);
    }

    robots
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i32 {
    let robots = parse_input(input);

    let width = 101;
    let height = 103;
    let steps = 100;
    let resolution = Vec2 {
        x: width,
        y: height,
    };

    let mut quarters: [i32; 4] = [0, 0, 0, 0];
    for robot in robots {
        // Move the robots by some amount of steps.
        let next_pos = robot.position + robot.velocity * steps;
        let limited_pos = (next_pos % resolution + resolution) % resolution;

        // If is in the middle of quarters, doesn't affect the result.
        if limited_pos.x == width / 2 || limited_pos.y == height / 2 {
            continue;
        }

        let qx = limited_pos.x < width / 2;
        let qy = limited_pos.y < height / 2;
        let idx = if qx { 0 } else { 2 } + if qy { 0 } else { 1 };
        quarters[idx] += 1;
    }

    let safety_factor = quarters[0] * quarters[1] * quarters[2] * quarters[3];
    safety_factor
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i32 {
    let robots = parse_input(input);

    let width: i32 = 101;
    let height: i32 = 103;
    let resolution = Vec2 {
        x: width,
        y: height,
    };

    // Check and store the steps that could be a tree.
    let mut test: Vec<(i32, i32)> = vec![];
    for steps in 1000..10000 {
        let mut quarters: [i32; 4] = [0, 0, 0, 0];
        for robot in &robots {
            // Move the robots by some amount of steps.
            let next_pos = robot.position + robot.velocity * steps;
            let limited_pos = (next_pos % resolution + resolution) % resolution;

            // If is in the middle of quarters, doesn't affect the result.
            if limited_pos.x == width / 2 || limited_pos.y == height / 2 {
                continue;
            }

            let qx = limited_pos.x < width / 2;
            let qy = limited_pos.y < height / 2;
            let idx = if qx { 0 } else { 2 } + if qy { 0 } else { 1 };
            quarters[idx] += 1;
        }

        let safety_factor = quarters[0] * quarters[1] * quarters[2] * quarters[3];
        test.push((steps, safety_factor));
    }

    // Sort by the safety factor to get the minimum.
    test.sort_by(|a, b| a.1.cmp(&b.1));
    let minimal_step = test[0].0;

    // Run it again, this time with the minimal step.
    let mut drawing: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];
    for robot in &robots {
        let next_pos = robot.position + robot.velocity * minimal_step;
        let limited_pos = (next_pos % resolution + resolution) % resolution;

        drawing[limited_pos.y as usize][limited_pos.x as usize] = '#';
    }

    // Render the end result to see if it is a tree.
    for y in 0..height {
        for x in 0..width {
            print!("{}", drawing[y as usize][x as usize]);
        }
        println!();
    }

    minimal_step
}

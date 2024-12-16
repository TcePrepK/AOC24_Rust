use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i64 = 480;
const ANSWER_TWO: i64 = 875318608908;

fn test_examples() -> [bool; 2] {
    let example = read_file("src/example");

    let results = [first_part(&example), second_part(&example)];

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

fn parse_to_matrix(input: &str) -> Vec<[f32; 6]> {
    let mut matrices: Vec<[f32; 6]> = vec![];
    for data in input.split("\n\n") {
        let processed: Vec<f32> = data
            .lines()
            .map(|line| {
                line.split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|s| s.split_at(2).1)
                    .map(|s| s.parse::<f32>().unwrap())
                    .collect::<Vec<f32>>()
            })
            .flatten()
            .collect::<Vec<f32>>();
        matrices.push(processed.try_into().unwrap());
    }

    matrices
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i64 {
    let matrices = parse_to_matrix(input);

    let mut result: i64 = 0;
    for matrix in matrices.iter() {
        let x = matrix[0];
        let y = matrix[2];
        let z = matrix[1];
        let w = matrix[3];
        let a = matrix[4];
        let b = matrix[5];

        let det = w * x - y * z;
        if det == 0.0 {
            continue;
        }

        let b_press = (b * x - a * z) / det;
        let a_press = (a - b_press * y) / x;

        if b_press.fract() > 0.01 || a_press.fract() > 0.01 {
            continue;
        }

        result += a_press as i64 * 3 + b_press as i64;
    }

    result
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i64 {
    let matrices = parse_to_matrix(input);

    let mut result: i64 = 0;
    for matrix in matrices.iter() {
        let x: f64 = matrix[0] as f64;
        let y: f64 = matrix[2] as f64;
        let z: f64 = matrix[1] as f64;
        let w: f64 = matrix[3] as f64;
        let a: f64 = 10000000000000.0 + matrix[4] as f64;
        let b: f64 = 10000000000000.0 + matrix[5] as f64;

        let det = w * x - y * z;
        if det == 0.0 {
            continue;
        }

        let b_press = (b * x - a * z) / det;
        let a_press = (a - b_press * y) / x;

        if b_press.fract() > 0.01 || a_press.fract() > 0.01 {
            continue;
        }

        result += a_press as i64 * 3 + b_press as i64;
    }

    result
}
use std::collections::HashMap;
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i32 = 11;
const ANSWER_TWO: i32 = 31;

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

/* ------------------- Solutions ------------------- */

fn first_part(input: &str) -> i32 {
    let mut first_col: Vec<i32> = Vec::new();
    let mut second_col: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap());
        first_col.push(split.next().unwrap());
        second_col.push(split.next().unwrap());
    }

    first_col.sort();
    second_col.sort();

    let mut diff: i32 = 0;
    for i in 0..first_col.len() {
        diff += (first_col[i] - second_col[i]).abs();
    }

    diff
}

fn second_part(input: &str) -> i32 {
    let mut first_col: Vec<i32> = Vec::new();
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut split = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap());
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        first_col.push(left);
        *hash_map.entry(right).or_insert(0) += 1;
    }

    let mut result: i32 = 0;
    for entry in first_col {
        if !hash_map.contains_key(&entry) {
            continue;
        };
        result += entry * hash_map[&entry];
    }

    result
}

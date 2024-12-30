use std::fs;

fn read_file(path: &str) -> String {
    if fs::exists(path).expect("Could not check file") {
        fs::read_to_string(path).expect("Could not read file")
    } else {
        String::new()
    }
}

const ANSWER_ONE: u32 = 3;
const ANSWER_TWO: i32 = 0;

fn test_examples() -> (bool, bool) {
    let mut example_1 = read_file("src/example_1");
    let mut example_2 = read_file("src/example_2");
    if example_1.is_empty() && !example_2.is_empty() {
        panic!("Example 1 is empty, but example 2 is not");
    } else if !example_1.is_empty() && example_2.is_empty() {
        panic!("Example 2 is empty, but example 1 is not");
    } else if example_1.is_empty() && example_2.is_empty() {
        example_1 = read_file("src/example");
        example_2 = example_1.clone();
    }

    let results = (first_part(&example_1), second_part(&example_2));

    if results.0 != 0 && results.0 != ANSWER_ONE {
        println!("Part One Wrong");
    }

    if results.1 != 0 && results.1 != ANSWER_TWO {
        println!("Part Two Wrong");
    }

    (results.0 == ANSWER_ONE, results.1 == ANSWER_TWO)
}

fn test_inputs(example_solutions: (bool, bool)) {
    let input = read_file("src/input");

    if example_solutions.0 {
        let start_time = std::time::Instant::now();
        let result = first_part(&input);
        let total_time = start_time.elapsed();
        println!("Part 1 result: {}, took: {:?}", result, total_time);
    }
    if example_solutions.1 {
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

fn parse_input(input: &str) -> (Vec<[u32; 5]>, Vec<[u32; 5]>) {
    let mut lock_pins = vec![];
    let mut key_heights = vec![];

    let mut current_grid = [0; 5];
    let mut in_grid = false;
    let mut top_grid = false;

    let mut iter = input.lines();
    loop {
        let line_opt = iter.next();
        if line_opt.is_none() || line_opt.unwrap() == "" {
            if top_grid {
                lock_pins.push(current_grid);
            } else {
                key_heights.push(current_grid);
            }

            in_grid = false;
            current_grid = [0; 5];

            if line_opt.is_none() {
                break;
            }
            continue;
        }

        let line = line_opt.unwrap();
        if !in_grid {
            if line == "#####" {
                in_grid = true;
                top_grid = true;
            } else {
                in_grid = true;
                top_grid = false;
            }
        }

        let chars = line.chars().collect::<Vec<char>>();
        for i in 0..5 {
            if chars[i] == '#' {
                current_grid[i] += 1;
            }
        }
    }

    (lock_pins, key_heights)
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u32 {
    let (lock_pins, key_heights) = parse_input(input);

    let mut unique_pairs = 0;
    for lock in lock_pins {
        for key in key_heights.iter() {
            if key.iter().enumerate().all(|(i, k)| k + lock[i] <= 7) {
                unique_pairs += 1;
            }
        }
    }

    unique_pairs
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i32 {
    0
}

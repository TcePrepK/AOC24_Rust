use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i32 = 2;
const ANSWER_TWO: i32 = 4;

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
    let reports: Vec<&str> = input.lines().collect();

    let mut safe_reports: i32 = 0;
    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut safe: bool = true;
        for i in 1..levels.len() - 1 {
            let prev_dif = levels[i] - levels[i - 1];
            let next_dif = levels[i + 1] - levels[i];

            if prev_dif * next_dif <= 0 || prev_dif.abs() > 3 || next_dif.abs() > 3 {
                safe = false;
                break;
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn second_part(input: &str) -> i32 {
    let reports: Vec<&str> = input.lines().collect();

    let mut safe_reports: i32 = 0;
    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut safe: bool = false;
        for i in 0..levels.len() {
            let mut clone_levels = levels.clone();
            clone_levels.remove(i);

            let mut sub_safe: bool = true;
            for i in 1..clone_levels.len() - 1 {
                let prev_dif = clone_levels[i] - clone_levels[i - 1];
                let next_dif = clone_levels[i + 1] - clone_levels[i];

                if prev_dif * next_dif <= 0 || prev_dif.abs() > 3 || next_dif.abs() > 3 {
                    sub_safe = false;
                    break;
                }
            }

            if sub_safe {
                safe = true;
                break;
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

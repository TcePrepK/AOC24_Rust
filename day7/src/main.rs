use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i64 = 3749;
const ANSWER_TWO: i64 = 11387;

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

#[allow(unused_variables)]
fn first_part(input: &str) -> i64 {
    let possible_operations = ['+', '*'];

    let mut result: i64 = 0;
    for equation in input.lines() {
        let data: Vec<&str> = equation.split(": ").collect::<Vec<&str>>();

        let answer: i64 = data[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = data[1]
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut num_checks: Vec<i64> = vec![];
        num_checks.push(answer);

        let mut last_index = numbers.len() - 1;
        while last_index > 0 {
            let operation_num = numbers[last_index];

            let mut next_checks = vec![];
            for check_num in num_checks.iter() {
                if check_num % operation_num == 0 {
                    next_checks.push(check_num / operation_num);
                }
                next_checks.push(check_num - operation_num);
            }

            num_checks = next_checks;
            last_index -= 1;
        }

        if num_checks.contains(numbers.get(0).unwrap()) {
            result += answer;
        }
    }

    result
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i64 {
    let possible_operations = ['+', '*'];

    let mut result: i64 = 0;
    for equation in input.lines() {
        let data: Vec<&str> = equation.split(": ").collect::<Vec<&str>>();

        let answer: i64 = data[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = data[1]
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut num_checks: Vec<i64> = vec![];
        num_checks.push(answer);

        let mut last_index = numbers.len() - 1;
        while last_index > 0 {
            let operation_num = numbers[last_index];

            let mut next_checks = vec![];
            for check_num in num_checks.iter() {
                if check_num % operation_num == 0 {
                    next_checks.push(check_num / operation_num);
                }

                let mut check_str = check_num.to_string();
                let operation_str = operation_num.to_string();
                let digit_count = operation_str.len();
                if check_str.ends_with(operation_str.as_str()) {
                    let _ = check_str.split_off(check_str.len() - digit_count);
                    if check_str.len() > 0 {
                        next_checks.push(check_str.parse::<i64>().unwrap());
                    }
                }

                let sum_result = check_num - operation_num;
                if sum_result < 0 {
                    continue;
                }
                next_checks.push(check_num - operation_num);
            }

            num_checks = next_checks;
            last_index -= 1;
        }

        if num_checks.contains(numbers.get(0).unwrap()) {
            result += answer;
        }
    }

    result
}

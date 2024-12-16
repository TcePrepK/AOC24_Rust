use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: usize = 55312;
const ANSWER_TWO: usize = 65601038650482;

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

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn digit_count(mut number: u64) -> usize {
    let mut result: usize = 0;
    while number > 0 {
        number /= 10;
        result += 1;
    }

    result
}

fn add_number(numbers: &mut Vec<(u64, usize)>, number: u64, count: usize) {
    if let Some(prev) = numbers.iter_mut().find(|x| x.0 == number) {
        prev.1 += count;
    } else {
        numbers.push((number, count));
    }
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> usize {
    let numbers: Vec<u64> = parse_input(input);

    let mut bundled_numbers: Vec<(u64, usize)> = vec![];
    for number in numbers.iter() {
        add_number(&mut bundled_numbers, *number, 1);
    }

    for i in 0..25 {
        let mut next_numbers: Vec<(u64, usize)> = vec![];
        for (number, count) in bundled_numbers.iter() {
            let digit_count = digit_count(*number);

            if *number == 0 {
                add_number(&mut next_numbers, 1, *count);
            } else if digit_count % 2 == 0 {
                let scale = 10_i64.pow(digit_count as u32 / 2) as u64;
                add_number(&mut next_numbers, *number / scale, *count);
                add_number(&mut next_numbers, *number % scale, *count);
            } else {
                add_number(&mut next_numbers, *number * 2024, *count);
            }
        }
        bundled_numbers = next_numbers;
    }

    bundled_numbers.iter().map(|x| x.1).sum()
}

#[allow(unused_variables)]
fn second_part(input: &str) -> usize {
    let numbers: Vec<u64> = parse_input(input);

    let mut bundled_numbers: Vec<(u64, usize)> = vec![];
    for number in numbers.iter() {
        add_number(&mut bundled_numbers, *number, 1);
    }

    for i in 0..75 {
        let mut next_numbers: Vec<(u64, usize)> = vec![];
        for (number, count) in bundled_numbers.iter() {
            let digit_count = digit_count(*number);

            if *number == 0 {
                add_number(&mut next_numbers, 1, *count);
            } else if digit_count % 2 == 0 {
                let scale = 10_i64.pow(digit_count as u32 / 2) as u64;
                add_number(&mut next_numbers, *number / scale, *count);
                add_number(&mut next_numbers, *number % scale, *count);
            } else {
                add_number(&mut next_numbers, *number * 2024, *count);
            }
        }
        bundled_numbers = next_numbers;
    }

    bundled_numbers.iter().map(|x| x.1).sum::<usize>()
}

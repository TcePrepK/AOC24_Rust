use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

#[allow(unused_variables)]
fn main() {
    let example = read_file("src/example");
    let input = read_file("src/input");
    let used_string = input;

    println!("First Part: {:?}", first_part(&used_string));
    println!("Second Part: {:?}", second_part(&used_string));
}

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

    bundled_numbers.iter().map(|x| x.1).sum()
}

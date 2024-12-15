use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i64 = 1928;
const ANSWER_TWO: i64 = 2858;

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
        println!("Part One: {:?}", first_part(&input));
    }
    if example_solutions[1] {
        println!("Part Two: {:?}", second_part(&input));
    }
}

fn main() {
    let example_solutions = test_examples();
    test_inputs(example_solutions);
}

/* ------------------- Helpers ------------------- */

fn parse_input(input: &str) -> Vec<i32> {
    let numbers = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    let mut idx: usize = 0;

    let mut result: Vec<i32> = vec![];
    for (i, num) in numbers.iter().enumerate() {
        let id: i32;
        if i % 2 == 0 {
            id = idx as i32;
            idx += 1;
        } else {
            id = -1;
        }

        for _ in 0..*num {
            result.push(id);
        }
    }

    result
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i64 {
    let mut parsed = parse_input(input);

    let mut last_index: usize = parsed.len() - 1;
    let mut first_index: usize = 0;
    while first_index < last_index {
        if parsed[first_index] != -1 {
            first_index += 1;
            continue;
        }

        if parsed[last_index] == -1 {
            last_index -= 1;
            continue;
        }

        parsed[first_index] = parsed[last_index];
        parsed[last_index] = -1;

        last_index -= 1;
        first_index += 1;
    }

    let mut result: i64 = 0;
    for (i, c) in parsed.iter().enumerate() {
        if *c == -1 {
            break;
        }

        result += (i as i64) * (*c as i64);
    }

    result
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i64 {
    let mut empty_slots: Vec<(usize, usize)> = vec![];
    let mut files: Vec<(usize, usize)> = vec![];

    let chars = input.chars().collect::<Vec<char>>();
    let mut last_index = 0;
    for (i, char) in chars.iter().enumerate() {
        let len = char.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push((last_index, len));
        } else {
            empty_slots.push((last_index, len));
        }
        last_index += len;
    }

    let mut end_files: Vec<(usize, usize, usize)> = vec![];
    while !files.is_empty() {
        let (start, length) = files.pop().unwrap();
        let index = files.len();

        let mut empty_index = empty_slots.len();
        for i in 0..index {
            if empty_slots[i].1 >= length {
                empty_index = i;
                break;
            }
        }

        if empty_index >= index {
            end_files.push((start, length, index));
            continue;
        }

        let empty_data = &mut empty_slots[empty_index];

        end_files.push((empty_data.0, length, index));
        empty_data.0 += length;
        if empty_data.1 <= length {
            empty_data.1 = 0;
            continue;
        }
        empty_data.1 -= length;

        // 8666607636406 >>
        // 8380334014484
        // 6478232739671
        // 5299816551211 <<
    }

    let mut result: i64 = 0;
    for (start, length, index) in end_files {
        for i in start..start + length {
            result += (i as i64) * (index as i64);
        }
    }

    result
}

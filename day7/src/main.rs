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

#[allow(unused_variables)]
fn first_part(input: &str) -> i64 {
    let possible_operations = ['+', '*'];

    let mut result: i64 = 0;
    for equation in input.lines() {
        let data: Vec<&str> = equation.split(": ").collect::<Vec<&str>>();

        let answer: i64 = data[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = data[1].split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();

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
        let numbers: Vec<i64> = data[1].split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();

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
                if sum_result < 0 { continue; }
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
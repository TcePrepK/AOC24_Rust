use regex::Regex;
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i32 = 161;
const ANSWER_TWO: i32 = 48;

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
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let nums = regex
        .captures_iter(&input)
        .map(|cap| {
            vec![
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            ]
        })
        .collect::<Vec<Vec<i32>>>();

    nums.iter().map(|nums| nums[0] * nums[1]).sum()
}

fn second_part(input: &str) -> i32 {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut enabled_mules: Vec<String> = Vec::new();

    let mut construct = String::from(input);
    let mut doing = true;
    let mut finished = false;
    while !finished {
        let mul_opt = mul_regex
            .find(&construct)
            .map(|mat| (mat.start(), mat.end(), mat.as_str().to_string()));
        let do_opt = if !doing {
            do_regex
                .find(&construct)
                .map(|mat| (mat.start(), mat.end()))
        } else {
            None
        };
        let dont_opt = dont_regex
            .find(&construct)
            .map(|mat| (mat.start(), mat.end()));

        if mul_opt.is_none() || (!doing && do_opt.is_none()) {
            finished = true;
            continue;
        }

        if let Some((_, end)) = do_opt {
            construct.replace_range(0..end, &*".".repeat(end));
            doing = true;
            continue;
        }

        let (mul_start, mul_end, mul_str) = mul_opt.unwrap();
        if let Some((dont_start, dont_end)) = dont_opt {
            if dont_start < mul_start {
                construct.replace_range(0..dont_end, &*".".repeat(dont_end));
                doing = false;
                continue;
            }
        }

        enabled_mules.push(mul_str);
        construct.replace_range(0..mul_end, &*".".repeat(mul_end));
    }

    let mut result = 0;
    for mul in enabled_mules {
        let capture = mul_regex.captures(&mul).unwrap();

        let left = capture[1].parse::<i32>().unwrap();
        let right = capture[2].parse::<i32>().unwrap();
        result += left * right;
    }

    result
}

use std::collections::HashMap;
use std::fs;

fn read_file(path: &str) -> String {
    if fs::exists(path).expect("Could not check file") {
        fs::read_to_string(path).expect("Could not read file")
    } else {
        String::new()
    }
}

const ANSWER_ONE: u64 = 6;
const ANSWER_TWO: u64 = 16;

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

/// Parses the input and returns a vector of available patterns and a vector of designs
fn parse_input(input: &str) -> (Vec<&str>, usize, Vec<Vec<char>>) {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let patterns = data[0].split(", ").collect::<Vec<&str>>();
    let max_pattern_len = patterns.iter().map(|x| x.len()).max().unwrap();
    let designs = data[1]
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    (patterns, max_pattern_len, designs)
}

/// Checks a design and returns how many different ways it can be made
fn check_design(
    design: &Vec<char>,
    patterns: &Vec<&str>,
    max_pattern_len: usize,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    // If the design is empty, we return 1
    if design.is_empty() {
        return 1;
    }

    let mut design_counter = 0;
    for i in 0..max_pattern_len {
        if i >= design.len() {
            break;
        }

        // Calculate the base, and if it is not in the pattern list, we skip this iteration
        let base = design[0..i + 1].to_vec();
        let base_string = (&base).into_iter().collect::<String>();
        if !patterns.contains(&base_string.as_str()) {
            continue;
        }

        // We calculate the remaining part as we will need it later
        let remaining = design[i + 1..].to_vec();
        let remaining_string = (&remaining).into_iter().collect::<String>();

        // Get the remaining count from the cache if it exists and increase design counter,
        // If it doesn't exist, we call check_design for the remaining part
        if let Some(remaining_count) = cache.get(&remaining_string) {
            design_counter += remaining_count;
        } else {
            design_counter += check_design(&remaining, patterns, max_pattern_len, cache);
        }
    }

    // We add the base part to the cache and return the total
    cache.insert(design.into_iter().collect::<String>(), design_counter);
    design_counter
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u64 {
    let (patterns, max_pattern_len, designs) = parse_input(input);
    let mut cache: HashMap<String, u64> = HashMap::new();

    // Iterate through the designs and call the check_design function to get the number of possible designs
    // If more than 0, it is a possible design
    let mut possible_designs: u64 = 0;
    for design in designs.iter() {
        possible_designs += check_design(design, &patterns, max_pattern_len, &mut cache).min(1);
    }

    possible_designs
}

#[allow(unused_variables)]
fn second_part(input: &str) -> u64 {
    let (patterns, max_pattern_len, designs) = parse_input(input);
    let mut cache: HashMap<String, u64> = HashMap::new();

    // Iterate through the designs and call the check_design function to get the number of possible designs
    let mut design_counter = 0;
    for design in designs.iter() {
        design_counter += check_design(design, &patterns, max_pattern_len, &mut cache);
    }

    design_counter
}

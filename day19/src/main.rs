use ahash::{AHashMap, AHashSet};
use utils::test_solutions;

const ANSWER_ONE: u64 = 6;
const ANSWER_TWO: u64 = 16;

fn main() {
    test_solutions(&first_part, ANSWER_ONE, &second_part, ANSWER_TWO);
}

/* ------------------- Helpers ------------------- */

/// Parses the input and returns a vector of available patterns and a vector of designs
fn parse_input(input: &str) -> (AHashSet<Vec<char>>, usize, Vec<Vec<char>>) {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let patterns = data[0]
        .split(", ")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<AHashSet<Vec<char>>>();
    let max_pattern_len = patterns.iter().map(|x| x.len()).max().unwrap();
    let designs = data[1]
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    (patterns, max_pattern_len, designs)
}

/// Checks a design and returns how many different ways it can be made
fn check_design(
    cache: &mut AHashMap<Vec<char>, u64>,
    patterns: &AHashSet<Vec<char>>,
    design: &Vec<char>,
    max_pattern_len: usize,
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
        if !patterns.contains(&base) {
            continue;
        }

        // We calculate the remaining part as we will need it later
        let remaining = design[i + 1..].to_vec();

        // Get the remaining count from the cache if it exists and increase design counter,
        // If it doesn't exist, we call check_design for the remaining part
        if let Some(remaining_count) = cache.get(&remaining) {
            design_counter += remaining_count;
        } else {
            design_counter += check_design(cache, patterns, &remaining, max_pattern_len);
        }
    }

    // We add the base part to the cache and return the total
    cache.insert(design.clone(), design_counter);
    design_counter
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u64 {
    let (patterns, max_pattern_len, designs) = parse_input(input);
    let mut cache = AHashMap::new();

    // Iterate through the designs and call the check_design function to get the number of possible designs
    // If more than 0, it is a possible design
    let mut possible_designs: u64 = 0;
    for design in designs.iter() {
        possible_designs += check_design(&mut cache, &patterns, design, max_pattern_len).min(1);
    }

    possible_designs
}

#[allow(unused_variables)]
fn second_part(input: &str) -> u64 {
    let (patterns, max_pattern_len, designs) = parse_input(input);
    let mut cache = AHashMap::new();

    // Iterate through the designs and call the check_design function to get the number of possible designs
    let mut design_counter = 0;
    for design in designs.iter() {
        design_counter += check_design(&mut cache, &patterns, design, max_pattern_len);
    }

    design_counter
}

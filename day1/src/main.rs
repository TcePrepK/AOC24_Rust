use std::collections::HashMap;
use utils::test_solutions;

fn main() {
    test_solutions(1, &first_part, Some(11), &second_part, Some(31));
}

/* ------------------- Helpers ------------------- */

/* ------------------- Solutions ------------------- */

fn first_part(input: &str) -> i32 {
    let mut first_col: Vec<i32> = Vec::new();
    let mut second_col: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap());
        first_col.push(split.next().unwrap());
        second_col.push(split.next().unwrap());
    }

    first_col.sort();
    second_col.sort();

    let mut diff: i32 = 0;
    for i in 0..first_col.len() {
        diff += (first_col[i] - second_col[i]).abs();
    }

    diff
}

fn second_part(input: &str) -> i32 {
    let mut first_col: Vec<i32> = Vec::new();
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut split = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap());
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        first_col.push(left);
        *hash_map.entry(right).or_insert(0) += 1;
    }

    let mut result: i32 = 0;
    for entry in first_col {
        if !hash_map.contains_key(&entry) {
            continue;
        };
        result += entry * hash_map[&entry];
    }

    result
}

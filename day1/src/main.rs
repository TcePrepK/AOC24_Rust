use std::collections::HashMap;
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

#[allow(unused_variables)]
fn main() {
    let example = read_file("src/example");
    let input = read_file("src/input");
    let used_string = input;

    println!("First Part: {:?}", first_part(used_string.clone()));
    println!("Second Part: {:?}", second_part(used_string.clone()));
}

fn first_part(input: String) -> i32 {
    let mut first_col: Vec<i32> = Vec::new();
    let mut second_col: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace().map(|num| num.parse::<i32>().unwrap());
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

fn second_part(input: String) -> i32 {
    let mut first_col: Vec<i32> = Vec::new();
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let mut split = line.split_whitespace().map(|num| num.parse::<i32>().unwrap());
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        first_col.push(left);
        *hash_map.entry(right).or_insert(0) += 1;
    }

    let mut result: i32 = 0;
    for entry in first_col {
        if !hash_map.contains_key(&entry) { continue; };
        result += entry * hash_map[&entry];
    }

    result
}
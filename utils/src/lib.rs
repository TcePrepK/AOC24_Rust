mod grid;
mod point;

use std::fmt::Debug;
use std::fs;

pub use grid::*;
pub use point::*;

/* --------------------------- Parsers --------------------------- */

#[inline]
pub fn read_file(path: &str) -> String {
    if fs::exists(path).expect("Could not check file") {
        fs::read_to_string(path).expect("Could not read file")
    } else {
        String::new()
    }
}

#[inline]
pub fn test_solutions<T: PartialEq + Debug, K: PartialEq + Debug>(
    day: u8,
    function_one: &dyn Fn(&str) -> T,
    expected_value_one: Option<T>,
    function_two: &dyn Fn(&str) -> K,
    expected_value_two: Option<K>,
) {
    let mut example_1 = read_file(&format!("day{day}/src/example_1"));
    let mut example_2 = read_file(&format!("day{day}/src/example_2"));
    if example_1.is_empty() && !example_2.is_empty() {
        panic!("Example 1 is empty, but example 2 is not");
    } else if !example_1.is_empty() && example_2.is_empty() {
        panic!("Example 2 is empty, but example 1 is not");
    } else if example_1.is_empty() && example_2.is_empty() {
        example_1 = read_file(&format!("day{day}/src/example"));
        example_2 = example_1.clone();
    }

    if example_1.is_empty() || example_2.is_empty() {
        panic!("Example is empty");
    }

    let input = read_file(&format!("day{day}/src/input"));
    if input.is_empty() {
        panic!("Input is empty");
    }

    if !expected_value_one.is_some_and(|v| function_one(&example_1) != v) {
        let start_time = std::time::Instant::now();
        let result = function_one(&input);
        let total_time = start_time.elapsed();
        println!("Part-1 ( {:?} ) - {:?} ", result, total_time);
    } else {
        println!("Part One Wrong");
    }

    if !expected_value_two.is_some_and(|v| function_two(&example_2) != v) {
        let start_time = std::time::Instant::now();
        let result = function_two(&input);
        let total_time = start_time.elapsed();
        println!("Part-2 ( {:?} ) - {:?}", result, total_time);
    } else {
        println!("Part Two Wrong");
    }
}

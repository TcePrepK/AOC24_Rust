use std::fmt::Display;
use std::fs;

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[inline]
pub fn read_file(path: &str) -> String {
    if fs::exists(path).expect("Could not check file") {
        fs::read_to_string(path).expect("Could not read file")
    } else {
        String::new()
    }
}

pub fn test_solutions<T: PartialEq + Display, K: PartialEq + Display>(
    function_one: &dyn Fn(&str) -> T,
    expected_value_one: T,
    function_two: &dyn Fn(&str) -> K,
    expected_value_two: K,
) {
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

    if example_1.is_empty() || example_2.is_empty() {
        panic!("Example is empty");
    }

    let input = read_file("src/input");
    if input.is_empty() {
        panic!("Input is empty");
    }

    if function_one(&example_1) == expected_value_one {
        let start_time = std::time::Instant::now();
        let result = function_one(&input);
        let total_time = start_time.elapsed();
        println!("Part 1 result: {}, took: {:?}", result, total_time);
    } else {
        println!("Part One Wrong");
    }

    if function_two(&example_2) == expected_value_two {
        let start_time = std::time::Instant::now();
        let result = function_two(&input);
        let total_time = start_time.elapsed();
        println!("Part 2 result: {}, took: {:?}", result, total_time);
    } else {
        println!("Part Two Wrong");
    }
}

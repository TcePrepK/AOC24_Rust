use std::fmt::Debug;
use std::hint::black_box;
use std::time::Instant;
use std::{env, fs};

#[inline]
pub fn read_file(path: &str) -> String {
    if fs::exists(path).expect("Could not check file") {
        fs::read_to_string(path).expect("Could not read file")
    } else {
        String::new()
    }
}

/// Gets the input from the given day. If no day is given, it is a benchmark call.
/// In the case of benchmark call, we will be using the relative path!
#[inline]
pub fn get_input(opt_day: Option<u8>) -> String {
    let result = if opt_day.is_some() {
        read_file(&format!("day{}/src/input", opt_day.unwrap()))
    } else {
        read_file(&"./src/input".to_string())
    };

    if result.is_empty() {
        panic!("Input is empty");
    }

    result
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

    let env_values = env::args().collect::<Vec<String>>();
    let benchmark = env_values
        .get(1)
        .is_some_and(|x| x.as_str().starts_with("--benchmark="));
    let iteration_count = env_values
        .get(1)
        .map(|x| x.as_str().split_once("=").unwrap().1)
        .unwrap_or("100")
        .parse::<u32>()
        .unwrap_or(100);

    if !expected_value_one.is_some_and(|v| function_one(&example_1) != v) {
        let initial_start_time = Instant::now();
        let result = function_one(&input);
        let initial_time = initial_start_time.elapsed();

        if benchmark {
            let bench_start_time = Instant::now();
            for _ in 0..iteration_count {
                let result = function_one(&input);
                black_box(result);
            }
            let bench_time = bench_start_time.elapsed() / iteration_count;
            println!("Part-1 ( {:?} ) - {:?} ", result, bench_time);
        } else {
            println!("Part-1 ( {:?} ) - {:?} ", result, initial_time);
        }
    } else {
        println!("Part One Wrong");
    }

    if !expected_value_two.is_some_and(|v| function_two(&example_2) != v) {
        let initial_start_time = Instant::now();
        let result = function_two(&input);
        let initial_time = initial_start_time.elapsed();

        if benchmark {
            let bench_start_time = Instant::now();
            for _ in 0..iteration_count {
                let result = function_two(&input);
                black_box(result);
            }
            let bench_time = bench_start_time.elapsed() / iteration_count;
            println!("Part-2 ( {:?} ) - {:?}", result, bench_time);
        } else {
            println!("Part-2 ( {:?} ) - {:?}", result, initial_time);
        }
    } else {
        println!("Part Two Wrong");
    }
}

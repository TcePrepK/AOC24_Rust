use criterion::{black_box, Criterion};
use std::fmt::Debug;
use std::fs;
use std::time::{Duration, Instant};

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
        read_file("./src/input")
    };

    if result.is_empty() {
        panic!("Input is empty");
    }

    result
}

#[inline]
pub fn get_example(opt_day: Option<u8>) -> String {
    let result = if opt_day.is_some() {
        read_file(&format!("day{}/src/example", opt_day.unwrap()))
    } else {
        read_file("./src/example")
    };

    if result.is_empty() {
        panic!("Example is empty");
    }

    result
}

#[inline]
pub fn test_example_string<T: PartialEq + Debug + Copy>(
    day: u8,
    function_one: &dyn Fn(&str) -> T,
    expected_value: Option<T>,
) -> (T, Option<T>, bool) {
    let mut input = get_example(Some(day));
    if input.is_empty() {
        input = get_example(None);
    }

    let result = function_one(&input);
    let is_correct = !expected_value.is_some_and(|v| result != v);
    (result, expected_value, is_correct)
}

#[inline]
pub fn test_example_bytes<T: PartialEq + Debug + Copy>(
    day: u8,
    function_one: &dyn Fn(&mut [u8]) -> T,
    expected_value: Option<T>,
) -> (T, Option<T>, bool) {
    let mut input = get_example(Some(day));
    if input.is_empty() {
        input = get_example(None);
    }

    let result = unsafe { function_one(input.as_bytes_mut()) };
    let is_correct = !expected_value.is_some_and(|v| result != v);
    (result, expected_value, is_correct)
}

fn time_functions<I1, I2, T: PartialEq + Debug, K: PartialEq + Debug>(
    input_one: I1,
    input_two: I2,
    example_result_one: (T, Option<T>, bool),
    example_result_two: (K, Option<K>, bool),
    function_one: &dyn Fn(I1) -> T,
    function_two: &dyn Fn(I2) -> K,
) {
    if example_result_one.2 {
        let initial_start_time = Instant::now();
        let result = function_one(input_one);
        let initial_time = initial_start_time.elapsed();
        println!(" 1 -> ( {:?} ) - {:?} (✅ )", result, initial_time);
    } else {
        println!(
            " 1 -> Got: {:?}, Expected: {:?} (❌ )",
            example_result_one.0,
            example_result_one.1.unwrap()
        );
    }

    if example_result_two.2 {
        let initial_start_time = Instant::now();
        let result = function_two(input_two);
        let initial_time = initial_start_time.elapsed();
        println!(" 2 -> ( {:?} ) - {:?} (✅ )", result, initial_time);
    } else {
        println!(
            " 2 -> Got: {:?}, Expected: {:?} (❌ )",
            example_result_two.0,
            example_result_two.1.unwrap()
        );
    }
}

#[inline]
pub fn test_both_parts<T: PartialEq + Debug, K: PartialEq + Debug>(
    day: u8,
    example_result_one: (T, Option<T>, bool),
    example_result_two: (K, Option<K>, bool),
    function_one: &dyn Fn(&str) -> T,
    function_two: &dyn Fn(&str) -> K,
) {
    println!("Day {}:", day);

    let input = read_file(&format!("day{day}/src/input"));
    if input.is_empty() {
        panic!("Input is empty");
    }

    time_functions(
        input.as_str(),
        input.as_str(),
        example_result_one,
        example_result_two,
        function_one,
        function_two,
    );
}

#[inline]
pub fn test_both_parts_bytes<T: PartialEq + Debug, K: PartialEq + Debug>(
    day: u8,
    example_result_one: (T, Option<T>, bool),
    example_result_two: (K, Option<K>, bool),
    function_one: &dyn Fn(&mut [u8]) -> T,
    function_two: &dyn Fn(&mut [u8]) -> K,
) {
    println!("Day {}:", day);

    let input = read_file(&format!("day{day}/src/input"));
    if input.is_empty() {
        panic!("Input is empty");
    }

    time_functions(
        unsafe { input.clone().as_bytes_mut() },
        unsafe { input.clone().as_bytes_mut() },
        example_result_one,
        example_result_two,
        function_one,
        function_two,
    );
}

#[inline]
pub fn run_both_benchmarks<T: PartialEq + Debug, K: PartialEq + Debug>(
    day: u8,
    c: &mut Criterion,
    part_one: &dyn Fn(&str) -> T,
    part_two: &dyn Fn(&str) -> K,
) {
    let mut group = c.benchmark_group(format!("Day{day}"));
    group.measurement_time(Duration::new(10, 0));

    let input = get_input(None);
    group.bench_function("1", |b| b.iter(|| part_one(black_box(&input))));
    group.bench_function("2", |b| b.iter(|| part_two(black_box(&input))));
}

#[inline]
pub fn run_both_benchmarks_bytes<T: PartialEq + Debug, K: PartialEq + Debug>(
    day: u8,
    c: &mut Criterion,
    part_one: &dyn Fn(&mut [u8]) -> T,
    part_two: &dyn Fn(&mut [u8]) -> K,
) {
    let mut group = c.benchmark_group(format!("Day{day}"));
    group.measurement_time(Duration::new(10, 0));

    let mut input_one = get_input(None);
    let mut input_two = get_input(None);
    let bytes_one = unsafe { input_one.as_bytes_mut() };
    let bytes_two = unsafe { input_two.as_bytes_mut() };
    group.bench_function("1", |b| b.iter(|| part_one(black_box(bytes_one))));
    group.bench_function("2", |b| b.iter(|| part_two(black_box(bytes_two))));
}

#[deprecated]
pub fn test_solutions<T: PartialEq + Debug, K: PartialEq + Debug>(
    day: u8,
    function_one: &dyn Fn(&str) -> T,
    expected_value_one: Option<T>,
    function_two: &dyn Fn(&str) -> K,
    expected_value_two: Option<K>,
) {
    println!("Day {}:", day);

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
        let initial_start_time = Instant::now();
        let result = function_one(&input);
        let initial_time = initial_start_time.elapsed();
        println!(" 1 -> ( {:?} ) - {:?} ", result, initial_time);
    } else {
        println!(" 1 -> Example is wrong");
    }

    if !expected_value_two.is_some_and(|v| function_two(&example_2) != v) {
        let initial_start_time = Instant::now();
        let result = function_two(&input);
        let initial_time = initial_start_time.elapsed();
        println!(" 2 -> ( {:?} ) - {:?} ", result, initial_time);
    } else {
        println!(" 2 -> Example is wrong");
    }
}

use std::fmt::Debug;
use std::hint::black_box;
use std::time::{Duration, Instant};
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

pub fn benchmark_solution<T: PartialEq + Debug>(
    func: &dyn Fn(&str) -> T,
    input: &str,
    iteration_count: u32,
) -> Duration {
    let mut times = Vec::with_capacity(iteration_count as usize);
    for _ in 0..iteration_count {
        let bench_start_time = Instant::now();
        let _ = black_box(func(&input));
        let bench_time = bench_start_time.elapsed();
        times.push(bench_time.as_secs_f32());
    }

    times.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let q1 = times[times.len() / 4];
    let q3 = times[3 * times.len() / 4];
    let iqr = q3 - q1;
    let lower_bound = q1 - 1.5 * iqr;
    let upper_bound = q3 + 1.5 * iqr;

    let cleaned = times
        .iter()
        .cloned()
        .filter(|&x| x >= lower_bound && x <= upper_bound)
        .collect::<Vec<f32>>();

    Duration::from_secs_f32(cleaned[cleaned.len() / 2])
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
        .is_some_and(|x| x.as_str().starts_with("benchmark="));
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

        let time = if benchmark {
            benchmark_solution(&function_one, &input, iteration_count)
        } else {
            initial_time
        };
        println!("Part-1 ( {:?} ) - {:?} ", result, time);
    } else {
        println!("Part One Wrong");
    }

    if !expected_value_two.is_some_and(|v| function_two(&example_2) != v) {
        let initial_start_time = Instant::now();
        let result = function_two(&input);
        let initial_time = initial_start_time.elapsed();

        let time = if benchmark {
            benchmark_solution(&function_two, &input, iteration_count)
        } else {
            initial_time
        };
        println!("Part-2 ( {:?} ) - {:?} ", result, time);
    } else {
        println!("Part Two Wrong");
    }
}

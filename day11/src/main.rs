use ahash::AHashMap;
use utils::test_solutions;

fn main() {
    test_solutions(
        11,
        &first_part,
        Some(55312),
        &second_part,
        Some(65601038650482),
    );
}

/* ------------------- Helpers ------------------- */

/// Parses the input into a hashmap of numbers and their number of occurrences.
fn parse_input(input: &str) -> AHashMap<u64, u64> {
    let numbers = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // Run through the numbers and add them to the hashmap
    let mut bundled_numbers = AHashMap::new();
    for number in numbers.iter() {
        add_number(&mut bundled_numbers, *number, 1);
    }

    bundled_numbers
}

fn add_number(numbers: &mut AHashMap<u64, u64>, number: u64, count: u64) {
    if numbers.contains_key(&number) {
        numbers.insert(number, numbers.get(&number).unwrap() + count);
    } else {
        numbers.insert(number, count);
    }
}

fn handle_blinking<const N: usize>(numbers_cache: AHashMap<u64, u64>) -> AHashMap<u64, u64> {
    let mut current_cache = numbers_cache;
    for _ in 0..N {
        let mut next_cache: AHashMap<u64, u64> = AHashMap::new();
        for (number, count) in current_cache.iter() {
            let digit_count = if *number == 0 { 0 } else { number.ilog10() + 1 };

            if *number == 0 {
                add_number(&mut next_cache, 1, *count);
            } else if digit_count % 2 == 0 {
                let scale = 10_u64.pow(digit_count / 2);
                add_number(&mut next_cache, *number / scale, *count);
                add_number(&mut next_cache, *number % scale, *count);
            } else {
                add_number(&mut next_cache, (*number * 253) << 3, *count);
            }
        }
        current_cache = next_cache;
    }

    current_cache
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u64 {
    let numbers_cache = parse_input(input);
    let after_blinking = handle_blinking::<25>(numbers_cache);
    after_blinking.iter().map(|x| x.1).sum()
}

#[allow(unused_variables)]
fn second_part(input: &str) -> u64 {
    let numbers_cache = parse_input(input);
    let after_blinking = handle_blinking::<75>(numbers_cache);
    after_blinking.iter().map(|x| x.1).sum()
}

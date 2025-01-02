use rayon::prelude::*;
use utils::test_solutions;

fn main() {
    test_solutions(22, &first_part, Some(37327623), &second_part, Some(24));
}

/* ------------------- Helpers ------------------- */

/// Parses and turns the given input into a vector of numbers
fn get_numbers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn get_next(num: u32) -> u32 {
    let mut num = num;
    num ^= num << 6;
    num &= 0xffffff;
    num ^= num >> 5;
    num ^= num << 11;
    num &= 0xffffff;
    num
}

/// Calculates the sequence of numbers while putting the difference sequence of 4 into the cache...
/// Annoying day
fn cache_sequence(
    bool_cache: &mut [bool; 130321],
    number_cache: &mut [u32; 130321],
    num: u32,
    highest_number: &mut u32,
) {
    let num_0 = num; // 123456
    let num_1 = get_next(num_0); // 15887950
    let num_2 = get_next(num_1); // 16495136
    let mut prev = get_next(num_2); // 527345

    let mut diff_0 = 9 + num_1 % 10 - num_0 % 10; // -3 + 9
    let mut diff_1 = 9 + num_2 % 10 - num_1 % 10; // 6 + 9
    let mut diff_2 = 9 + prev % 10 - num_2 % 10; // -1 + 9

    for _ in 3..2000 {
        let next = get_next(prev);
        let secret = next % 10;
        let diff_3 = 9 + secret - prev % 10;
        prev = next;

        let hash_id: usize = diff_0 as usize * 19 * 19 * 19
            + diff_1 as usize * 19 * 19
            + diff_2 as usize * 19
            + diff_3 as usize;

        diff_0 = diff_1;
        diff_1 = diff_2;
        diff_2 = diff_3;

        // If the hash id is not already in the current input cache, add it
        if bool_cache[hash_id] {
            continue;
        }
        bool_cache[hash_id] = true;
        number_cache[hash_id] += secret;

        if number_cache[hash_id] > *highest_number {
            *highest_number = number_cache[hash_id];
        }
    }
}

/* ------------------- Solutions ------------------- */

fn first_part(input: &str) -> u64 {
    let numbers = get_numbers(input);
    numbers
        .par_iter()
        .map(|&number| {
            let mut num = number;
            for _ in 0..2000 {
                num ^= num << 6;
                num &= 0xffffff;
                num ^= num >> 5;
                num ^= num << 11;
                num &= 0xffffff;
            }
            num as u64
        })
        .sum::<u64>()
}

fn second_part(input: &str) -> u32 {
    let numbers = get_numbers(input);

    let highest_number = &mut 0;
    let mut number_cache = [0; 130321];
    numbers.iter().for_each(|&number| {
        let mut bool_cache = [false; 130321];
        cache_sequence(&mut bool_cache, &mut number_cache, number, highest_number);
    });

    *number_cache.iter().max().unwrap()
}

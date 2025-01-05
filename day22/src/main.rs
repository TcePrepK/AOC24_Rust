//! --- Day 22: Monkey Market ---
//! https://adventofcode.com/2024/day/22
//!
//! This solution focuses on using threads to speed up the computation.
//! The most simplistic solution does require about two million iterations to finish.
//! Additional information about this solution can be found in the module-level documentation.
//!
//! - Part One -
//! The promise is straightforward, get the 2000th random number using the XOR-Shift algorithm.
//! We can get faster runtimes by using threads and SIMD instructions.
//! We create chunks of size 8, run all those 8 at the same time and run chunks in parallel.
//!
//! - Part Two -
//! We can assume differences as base 19 and hash the four differences into a single number.
//! We can get faster runtimes by using threads once again.

use rayon::prelude::*;
use std::sync::Mutex;
use utils::test_solutions;
use wide::u32x8;

fn main() {
    test_solutions(22, &first_part, Some(37327623), &second_part, Some(24));
}

/* ------------------- Helpers ------------------- */

/// Parses and turns the given input into a vector of numbers.
fn get_numbers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

/// Calculates the next random number using the XOR-Shift algorithm.
/// This is used to generate the random numbers for today's challenge.
fn get_next(num: u32) -> u32 {
    let mut num = num;
    num ^= num << 6;
    num &= 0xffffff;
    num ^= num >> 5;
    num ^= num << 11;
    num &= 0xffffff;
    num
}

/// For each number, calculates the sequence of numbers while putting the difference sequence of 4 into the cache.
/// Hashes the sequence of four numbers into a single number and stores it in the cache instead of HashMap.
/// For each difference, maps [-9, 9] to [0, 18] then uses base 19 to base 10 conversion.
/// Seen cache is there to avoid caching the same sequence twice.
fn cache_sequence(seen_cache: &mut [bool; 130321], number_cache: &mut [u32; 130321], num: u32) {
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
        if seen_cache[hash_id] {
            continue;
        }
        seen_cache[hash_id] = true;
        number_cache[hash_id] += secret;
    }
}

/* ------------------- Solutions ------------------- */

fn first_part(input: &str) -> u64 {
    // Activate all threads on chunks of size 8
    let numbers = get_numbers(input);
    numbers
        .par_chunks(8)
        .map(|numbers| {
            // We create an u32x8 using the chunk. Chunk can have less than 8 elements.
            let mut array = [0; 8];
            for i in 0..8.min(numbers.len()) {
                array[i] = numbers[i];
            }

            let mut num = u32x8::new(array);
            let mask = u32x8::splat(0xffffff);

            // We run the XOR-Shift algorithm 2000 times
            for _ in 0..2000 {
                num ^= num << 6;
                num &= mask;
                num ^= num >> 5;
                num ^= num << 11;
            }
            num &= mask;

            // We unbundle the u32x8 into an array and sum the results
            let results = num.to_array();
            results.iter().sum::<u32>() as u64
        })
        .sum::<u64>()
}

fn second_part(input: &str) -> u32 {
    let numbers = get_numbers(input);

    let mutex_highest = Mutex::new(0);
    let mutex_cache = Mutex::new([0; 130321]);
    numbers.par_chunks(20).for_each(|numbers| {
        let mut number_cache = [0; 130321];

        for number in numbers {
            let mut seen = [false; 130321];
            cache_sequence(&mut seen, &mut number_cache, *number);
        }

        let mut cache = mutex_cache.lock().unwrap();
        let mut highest = mutex_highest.lock().unwrap();
        for i in 0..130321 {
            cache[i] += number_cache[i];
            if cache[i] > *highest {
                *highest = cache[i];
            }
        }
    });

    let result = mutex_highest.lock().unwrap();
    *result
}

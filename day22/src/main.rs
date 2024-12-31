use utils::test_solutions;

fn main() {
    test_solutions(22, &first_part, Some(37327623), &second_part, Some(24));
}

/* ------------------- Helpers ------------------- */

/// Parses and turns the given input into a vector of numbers
fn get_numbers(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

/// Calculates the sequence of numbers while putting the difference sequence of 4 into the cache...
/// Annoying day
fn cache_sequence(
    mut num: u32,
    input_idx: usize,
    input_len: usize,
    n: usize,
    prev_four: &mut [i32; 4],
    bool_cache: &mut [bool; 130320],
    number_cache: &mut [u32; 130320],
    highest_score: &mut u32,
) {
    let prev = num % 10;
    num ^= num << 6;
    num &= 0xffffff;
    num ^= num >> 5;
    num ^= num << 11;
    num &= 0xffffff;
    let secret = num % 10;

    prev_four[3] = prev_four[2];
    prev_four[2] = prev_four[1];
    prev_four[1] = prev_four[0];
    prev_four[0] = secret as i32 - prev as i32;

    if 2000 - n >= 4 {
        // Use base19 to base10 conversation to get the hash id (18, 18, 18, 18) -> 130320
        let hash_id: usize = (prev_four[0] + 9) as usize * 19 * 19 * 19
            + (prev_four[1] + 9) as usize * 19 * 19
            + (prev_four[2] + 9) as usize * 19
            + (prev_four[3] + 9) as usize;

        // If the hash id is not already in the current input cache, add it
        if !bool_cache[hash_id] {
            bool_cache[hash_id] = true;

            if secret != 0 {
                let cached_value = &mut number_cache[hash_id];
                if *cached_value < u32::MAX {
                    *cached_value += secret;

                    if highest_score < cached_value {
                        *highest_score = *cached_value;
                    }
                } else {
                    number_cache[hash_id] = secret;
                }
            }
        }
    }

    if n == 1 {
        return;
    }

    cache_sequence(
        num,
        input_idx,
        input_len,
        n - 1,
        prev_four,
        bool_cache,
        number_cache,
        highest_score,
    );
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u64 {
    let numbers = get_numbers(input);
    let mut last_score = 0;

    for number in numbers {
        let mut num = number;
        for i in 0..2000 {
            num ^= num << 6;
            num &= 0xffffff;
            num ^= num >> 5;
            num ^= num << 11;
            num &= 0xffffff;
        }
        last_score += num as u64;
    }

    last_score
}

#[allow(unused_variables)]
fn second_part(input: &str) -> u32 {
    let numbers = get_numbers(input);
    let highest_score: &mut u32 = &mut 0;

    let input_len = numbers.len();
    let mut number_cache: [u32; 130320] = [u32::MAX; 130320];
    for (input_idx, number) in numbers.iter().enumerate() {
        let mut bool_cache: [bool; 130320] = [false; 130320];
        cache_sequence(
            *number,
            input_idx,
            input_len,
            2000,
            &mut [0; 4],
            &mut bool_cache,
            &mut number_cache,
            highest_score,
        );
    }

    *highest_score
}

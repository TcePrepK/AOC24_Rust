use std::collections::HashMap;
use std::fs;

fn read_file(path: &str) -> String {
    if fs::exists(path).expect("Could not check file") {
        fs::read_to_string(path).expect("Could not read file")
    } else {
        String::new()
    }
}

const ANSWER_ONE: u64 = 2024;

fn test_examples() -> bool {
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

    let results = first_part(&example_1);

    if results != 0 && results != ANSWER_ONE {
        println!("Part One Wrong");
    }

    results == ANSWER_ONE
}

fn test_inputs(example_solutions: bool) {
    let input = read_file("src/input");

    if example_solutions {
        let start_time = std::time::Instant::now();
        let result = first_part(&input);
        let total_time = start_time.elapsed();
        println!("Part 1 result: {}, took: {:?}", result, total_time);
    }

    let start_time = std::time::Instant::now();
    let result = second_part(&input);
    let total_time = start_time.elapsed();
    println!("Part 2 result: {}, took: {:?}", result, total_time);
}

fn main() {
    let example_solutions = test_examples();
    test_inputs(example_solutions);
}

/* ------------------- Helpers ------------------- */

/// Hashes inputs of [a...w][a...w][a...w] and [x|y|z][0...44] to a number.
/// Hash for each combination is different.
fn hash_input(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let a = chars[0];
    let b = chars[1];
    let c = chars[2];

    if a == 'x' || a == 'y' {
        12167
            + 45 * (a as usize - b'x' as usize)
            + 10 * (b as usize - b'0' as usize)
            + (c as usize - b'0' as usize)
    } else if a == 'z' {
        12167 + 10 * (b as usize - b'0' as usize) + (c as usize - b'0' as usize)
    } else {
        23 * 23 * (a as usize - b'a' as usize)
            + 23 * (b as usize - b'a' as usize)
            + (c as usize - b'a' as usize)
    }
}

/// Hashes an instruction to a number to not store unnecessary data.
/// The output is as follows, [op (2 bits)][left (15 bits)][right (15 bits)].
fn hash_instruction(op: &str, left: usize, right: usize) -> u32 {
    let op = match op {
        "AND" => 0,
        "OR" => 1,
        "XOR" => 2,
        _ => panic!("Unknown op: {}", op),
    };

    (op as u32) << 30 | (left as u32) << 15 | (right as u32)
}

/// Decodes an instruction to (op, left, right).
/// The input is as follows, [op (2 bits)][left (15 bits)][right (15 bits)].
/// Uses 0x7FFF to represent the maximum value of a 15-bit number.
fn decode_instruction(instruction: u32) -> (String, usize, usize) {
    let right = instruction & 0x7FFF;
    let left = (instruction >> 15) & 0x7FFF;
    let op = (instruction >> 30) & 0x3;

    let op = match op {
        0 => "AND",
        1 => "OR",
        2 => "XOR",
        _ => panic!("Unknown op: {}", op),
    };

    (op.to_string(), left as usize, right as usize)
}

/// Tries to find the value of an instruction in the cache.
/// If it encounters a non-registered value, it will resolve that one first.
/// After it is done, it will cache the result.
fn resolve_value(cache: &mut [i8; 12167 + 90], instructions: &[u32], instruction: u32) -> i8 {
    let (op, left, right) = decode_instruction(instruction);

    let left_cache = if cache[left] < 0 {
        resolve_value(cache, instructions, instructions[left])
    } else {
        cache[left]
    };
    let right_cache = if cache[right] < 0 {
        resolve_value(cache, instructions, instructions[right])
    } else {
        cache[right]
    };
    cache[left] = left_cache;
    cache[right] = right_cache;

    match op.as_str() {
        "AND" => left_cache & right_cache,
        "OR" => left_cache | right_cache,
        "XOR" => left_cache ^ right_cache,
        _ => panic!("Unknown op: {}", op),
    }
}

/// Parses the input to (cache, instructions, outputs).
/// Cache is a bool array to store each cached input's value.
/// Instructions is an array of (op, left, right) turned into a number once again.
/// Outputs is an array of instructions needed to get [z][0...44].
fn parse_input(input: &str) -> ([i8; 12167 + 90], [u32; 12167 + 46]) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let inputs = parts[0].split("\n").collect::<Vec<&str>>();
    let instruct_inputs = parts[1].split("\n").collect::<Vec<&str>>();

    let mut cache: [i8; 12167 + 90] = [-1; 12167 + 90];
    for input in inputs {
        let split = input.split(": ").collect::<Vec<&str>>();
        let id = hash_input(split[0]);

        cache[id] = split[1].parse::<i32>().unwrap() as i8;
    }

    let mut instructions = [0; 12167 + 46];
    for instruction in instruct_inputs {
        let split = instruction.split_whitespace().collect::<Vec<&str>>();

        let left_hash = hash_input(split[0]);
        let op = split[1].to_string();
        let right_hash = hash_input(split[2]);

        let result_hash = hash_input(split[4]);
        let instruction_hash = hash_instruction(&op, left_hash, right_hash);

        instructions[result_hash] = instruction_hash;
    }

    (cache, instructions)
}

/// This is specifically for part 2, it swaps the values inside the special caches.
fn swap_instructions(
    instruction_cache: &mut HashMap<String, String>,
    result_cache: &mut HashMap<String, String>,
    a: &String,
    b: &String,
) {
    // First, from values we get the instructions (z0 to "x0 XOR y0")
    let instruction_a = result_cache.get(a).unwrap().clone();
    let instruction_b = result_cache.get(b).unwrap().clone();

    // Then we swap the values
    result_cache.insert(a.clone(), instruction_b.clone());
    result_cache.insert(b.clone(), instruction_a.clone());
    instruction_cache.insert(instruction_a, b.clone());
    instruction_cache.insert(instruction_b, a.clone());
}

/// Parses the input to (instruction_cache, result_cache).
/// Instruction cache is a hashmap of (instruction -> value), and the result cache is a hashmap of (value -> instruction).
fn parse_input_to_caches(input: &str) -> (HashMap<String, String>, HashMap<String, String>) {
    // Instruction cache is (Instruction -> Value), meanwhile result cache is (Value -> Instruction). Basically reversed.
    let mut instruction_cache: HashMap<String, String> = HashMap::new();
    let mut result_cache: HashMap<String, String> = HashMap::new();

    let instruct_inputs = input.split("\n\n").collect::<Vec<&str>>()[1]
        .split("\n")
        .collect::<Vec<&str>>();

    // Firstly, we parse the input and store the instructions in the instruction cache
    for instruction in instruct_inputs {
        let split = instruction.split_whitespace().collect::<Vec<&str>>();

        let id_a = format!("{} {} {}", split[0], split[1], split[2]);
        instruction_cache.insert(id_a.clone(), split[4].parse().unwrap());

        let id_b = split[4].to_string();
        result_cache.insert(id_b, id_a);
    }

    (instruction_cache, result_cache)
}

/// Reads the data from the cache and returns the value if it exists, otherwise returns the empty string.
/// Considers [a op b] and [b op a] as the same value and checks for both.
fn get_or_empty(cache: &HashMap<String, String>, a: &str, b: &str, op: &str) -> String {
    let empty = String::new();
    [format!("{a} {op} {b}"), format!("{b} {op} {a}")]
        .iter()
        .find(|k| cache.contains_key(*k))
        .map(|k| cache.get(k).unwrap())
        .unwrap_or_else(|| &empty)
        .to_string()
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u64 {
    let (mut cache, instructions) = parse_input(input);

    let mut result: u64 = 0;
    let mut index = 0;
    loop {
        if index > 45 || instructions[12167 + index] == 0 {
            break;
        }

        // For each z instruction, we resolve the value then store it in the result
        let instruction = instructions[12167 + index];
        let resolved: u64 = resolve_value(&mut cache, &instructions, instruction) as u64;

        result |= resolved << index;
        index += 1;
    }

    result
}

#[allow(unused_variables)]
fn second_part(input: &str) -> String {
    let (mut instruction_cache, mut result_cache) = parse_input_to_caches(input);

    // Swapped values are stored here to get the final result
    let mut swapped_values = vec![];

    // We have to check every bit, but we can't do it in a for loop because we are editing the idx.
    // Start by setting carry_in of the first bit to empty
    let mut c_n = String::new();
    let mut idx = 0;
    while idx < 45 {
        let x_n = format!("x{}{}", if idx < 10 { "0" } else { "" }, idx);
        let y_n = format!("y{}{}", if idx < 10 { "0" } else { "" }, idx);
        let z_n = format!("z{}{}", if idx < 10 { "0" } else { "" }, idx);

        // x ^ y ^ c => Sum
        // xy + (x ^ y)c => Carry

        // Primary values
        let x_xor_y = get_or_empty(&instruction_cache, &x_n, &y_n, "XOR");
        let x_and_y = get_or_empty(&instruction_cache, &x_n, &y_n, "AND");

        if c_n == "" {
            // If c_n is empty, we are on the first bit
            let sum = x_xor_y;
            let carry = x_and_y;

            c_n = carry;
        } else {
            // Otherwise, we can continue our calculations
            let x_xor_y_and_c = get_or_empty(&instruction_cache, &x_xor_y, &c_n, "AND");

            let sum = get_or_empty(&instruction_cache, &x_xor_y, &c_n, "XOR");
            let carry = get_or_empty(&instruction_cache, &x_and_y, &x_xor_y_and_c, "OR");

            // If the sum value we've got is empty, either [x ^ y] or [c_n] is faulty.
            // Because we know c_n is not empty, we can just consider [x ^ y] as the faulty value.
            if sum.is_empty() {
                // We know the sum supposed to be [z_n] but it's not, so we are using the result cache to find the instruction that should be there.
                let expected_values = result_cache
                    .get(&z_n)
                    .unwrap()
                    .split(" XOR ")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();

                // The result will either be [[x ^ y] or [c_n]] OR [[c_n] or [x ^ y]].
                // We know c_n is not faulty, so we set swapping to the other value.
                let swapping = if expected_values[0] == c_n {
                    &expected_values[1]
                } else if expected_values[1] == c_n {
                    &expected_values[0]
                } else {
                    panic!("Could not find expected value");
                };

                // Add to the swapped values, then swap the instructions
                swapped_values.push(x_xor_y.clone());
                swapped_values.push(swapping.clone());
                swap_instructions(
                    &mut instruction_cache,
                    &mut result_cache,
                    &x_xor_y,
                    &swapping,
                );
                continue;
            } else if !sum.starts_with("z") {
                // If the sum does not start with z, we know the result of [[x ^ y] ^ [c_n]] is faulty.
                // So we can just swap that result with the z_n instruction.

                // Add to the swapped values, then swap the instructions
                swapped_values.push(z_n.clone());
                swapped_values.push(sum.clone());
                swap_instructions(&mut instruction_cache, &mut result_cache, &z_n, &sum);
                continue;
            }

            c_n = carry;
        }

        idx += 1;
    }

    // Finally, sort and print the swapped values
    swapped_values.sort();
    swapped_values.join(",")
}

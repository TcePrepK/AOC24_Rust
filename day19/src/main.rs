//! --- Day 19: Linen Layout ---
//! https://adventofcode.com/2024/day/19
//!
//! This solution is based on using a [trie](https://en.wikipedia.org/wiki/Trie)
//! Each node in the trie is a 6-element array.
//! Each index corresponds to a possible pattern character (w, u, r, g, b).
//! Luckily, none of the characters hashes into 3, so we can use that as a flag for a valid pattern.
//!
//! For each design, we check every sub-design of the design to see if it is a valid design.
//! Additional information about this solution can be found in the module-level documentation.

use rayon::prelude::*;
use utils::test_solutions;

fn main() {
    test_solutions(19, &first_part, Some(6), &second_part, Some(16));
}

/* ------------------- Helpers ------------------- */

/// To create a 1D trie, we first create the array that will be holding the trie.
/// Then we iterate through each pattern, get their characters and add them to the trie.
/// Trie works as follows;
/// Each item is an array of six elements (`w, u, r, g, b` gets hashed into `0, 2, 5, 1, 4` respectively)
/// Third index is if the pattern is valid or not. If it is an actual pattern, it is 1 otherwise 0.
/// Each index holds a pointer to the next element in the array (in this case an index).
fn parse_input(input: &str) -> (Vec<[usize; 6]>, Vec<Vec<u8>>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    // Let's start creating the trie. We first loop through each pattern.
    let mut trie: Vec<[usize; 6]> = vec![[0; 6]];
    for pattern in patterns.split(", ") {
        // This is the hashing part; each character is hashed in a way to get proper indices.
        let chars = pattern.bytes().map(|x| (x ^ (x >> 4)) & 0b111);

        // While creating the trie, we start from the root, and the root is always 0.
        let mut index = 0;
        for char in chars {
            // trie[index] gives us the node (array) at the current index.
            // char is the index of the character in the pattern.
            // if trie[index][char] is 0, that means there is no node it is pointing to.
            if trie[index][char as usize] == 0 {
                // If it isn't pointing to anything, we have to create a new node!
                // The length of trie is how many nodes there are.
                // The `length - 1` will be the last item, so `length` is the next item.
                trie[index][char as usize] = trie.len();
                index = trie.len();
                trie.push([0; 6]);
            } else {
                // If there is a node, we now have to point to it.
                // Index is our current node, so we can just change it!
                index = trie[index][char as usize];
            }
        }

        // After all the characters are added, we are at the end of the pattern.
        // We have to set the last node as valid.
        // Once again, we are using index 3 because none of the characters hash into 3
        trie[index][3] = 1;
    }

    // We do the same hashing for the designs as well.
    // For each line, we hash each character and store it in a vector.
    let hashed_designs = designs
        .par_lines()
        .map(|x| {
            x.bytes()
                .map(|x| (x ^ (x >> 4)) & 0b111)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    (trie, hashed_designs)
}

/// We will be doing this check for each design.
/// We will be checking every sub-design of the design.
/// Sub-designs are designs that start from a point and end at the end of the design.
/// For each sub-design, using the trie, we will be checking if it is a valid design.
/// If it is a valid design, we will be adding it to the cache.
fn check_design(trie: &Vec<[usize; 6]>, design: Vec<u8>) -> u64 {
    // This cache represents each sub-design.
    // We will start with the whole design as 1
    let length = design.len();
    let mut cache = vec![0; length + 1];
    cache[0] = 1;

    for start in 0..length {
        // [start to length] is our sub-design.
        // If the sub-design is not possible, we skip it.
        if cache[start] == 0 {
            continue;
        }

        // Now we will be checking each character in the sub-design.
        let mut trie_index = 0;
        for char_idx in start..length {
            // We first get the char out of the design.
            let char = design[char_idx];

            // We will get the next trie node index from the trie.
            trie_index = trie[trie_index][char as usize];
            if trie_index == 0 {
                // If the index is 0, we know that there is no node for this set of characters.
                // We can skip this iteration.
                break;
            }

            // If the next node is not a valid design, we don't have to cache it.

            // If the next node is a valid design, we will add it to the cache.
            cache[char_idx + 1] += cache[start] * trie[trie_index][3];
        }
    }

    let possible_ways = cache[length];
    possible_ways as u64
}

/* ------------------- Solutions ------------------- */

fn first_part(input: &str) -> u64 {
    let (trie, designs) = parse_input(input);

    // Iterate through the designs and call the check_design function to get the number of possible designs
    // If more than 0, it is a possible design
    let mut possible_designs: u64 = 0;
    for design in designs {
        possible_designs += check_design(&trie, design).min(1);
    }

    possible_designs
}

fn second_part(input: &str) -> u64 {
    let (trie, designs) = parse_input(input);

    // Iterate through the designs and call the check_design function to get the number of possible designs
    let mut design_counter = 0;
    for design in designs {
        design_counter += check_design(&trie, design);
    }

    design_counter
}

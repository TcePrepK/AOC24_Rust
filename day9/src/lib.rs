//! --- Day 9: Disk Fragmenter ---
//! https://adventofcode.com/2024/day/9
//!
//! -- Part One --
//!   This part is straightforward at first glance, find an id from the end, move it to the first empty space
//!   My initial solution involved expanding the memory and getting one giant vector which was slow.
//!   I ended up switching to using the input as a memory, removing the parsing step and using the values at mutable rather than vectors.
//!
//! -- Part Two --
//!   This part is a bit more complicated due to having to consider files as a whole. The method we use is pretty close to the one used in the previous part.
//!   For each size we keep a memory that indicates the earliest space it can go to. When we move a file, we have to recalculate this array.
//!   When we move a file to the earliest space, we calculate the index and size it is in so we can calculate the sum-check on the fly.
//!   If we cannot move the file, we can just add it to the sum-check without recalculating the earliest space array.

/* ------------------- Helpers ------------------- */

/* ------------------- Solutions ------------------- */

pub fn part1(memory: &mut [u8]) -> u64 {
    let mut result = 0;

    // [`left_position`] is the empty position in the expanded memory.
    // [`left_index`] and `right_index` is the index of the memory tile.
    // {`left_instance`} and `right_instance` are the data in that memory tile.
    // Reducing them to keep track of the empty spaces and the sizes.
    let mut left_position: usize = 0;
    let mut left_index = 0;
    let mut right_index = memory.len() - 1;
    let mut left_instance = memory[left_index];
    let mut right_instance = memory[right_index];
    while left_index <= right_index {
        // If the left number we are looking at is 0 (b'0' == 48), we move to the next number.
        if left_instance == 48 {
            left_index += 1;
            left_instance = memory[left_index];
            continue;
        }

        // If the right number we are looking at is 0, we move to the previous number.
        if right_instance == 48 {
            right_index -= 2;
            right_instance = memory[right_index];
            continue;
        }

        // If they met in the middle, the instance we are looking at is the smaller one.
        if left_index == right_index {
            left_instance = left_instance.min(right_instance);
            right_instance = left_instance;
        }

        // If the left number is size, we add to the result.
        if left_index % 2 == 0 {
            // Loop through the left number until the size is bigger than 0.
            while left_instance > 48 {
                let id = left_index / 2;
                result += (id * left_position) as u64;
                left_position += 1;
                left_instance -= 1;
            }
            continue;
        }

        // We can move the right number to the left position.
        // Moving a number to an empty space means adding the id to the result,
        // decreasing the size (at the right), decreasing the empty space (at the left)
        // and increasing the position index we are looking for.
        let id = right_index / 2;
        result += (id * left_position) as u64;
        right_instance -= 1;
        left_instance -= 1;
        left_position += 1;
    }

    result
}

const SIZE_TO_SUM: [u64; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

pub fn part2(memory: &mut [u8]) -> u64 {
    let mut indices_of_spaces: Vec<u32> = Vec::with_capacity(memory.len() / 2);
    let mut earliest_space_for_size = [memory.len(); 10];

    // We copy the memory from input to memory.
    // Then we also calculate the earliest space for each size at the same time.
    let mut last_index = 0;
    for id in 0..memory.len() / 2 {
        let size = memory[id * 2] - 48;
        let empty_space = memory[id * 2 + 1] - 48;

        indices_of_spaces.push(last_index + size as u32);
        last_index += (size + empty_space) as u32;

        // Loop through sizes, one to
        for effected_size in 1..empty_space + 1 {
            let prev_location = &mut earliest_space_for_size[effected_size as usize];
            if *prev_location < id {
                continue;
            }

            *prev_location = id;
        }
    }

    // memory.push(input.last().unwrap() - 48);

    let mut sum_check = 0;
    for id in (1..memory.len() / 2 + 1).rev() {
        let size = memory[id * 2] - 48;

        let earliest_space = earliest_space_for_size[size as usize];
        if earliest_space >= id {
            let index_of_space = indices_of_spaces[id - 1] + memory[id * 2 - 1] as u32 - 48;
            sum_check +=
                (index_of_space as u64 * size as u64 + SIZE_TO_SUM[size as usize]) * id as u64;
            continue;
        }

        // Move size to here
        let check_empty_space = memory[earliest_space * 2 + 1] - 48;
        let index_of_space = indices_of_spaces[earliest_space];
        sum_check += (index_of_space as u64 * size as u64 + SIZE_TO_SUM[size as usize]) * id as u64;
        memory[earliest_space * 2 + 1] -= size;
        indices_of_spaces[earliest_space] += size as u32;

        // Re-calculate the earliest_space_for_size
        for effected_size in 1..(check_empty_space + 1) {
            let prev_location = &mut earliest_space_for_size[effected_size as usize];
            if *prev_location < earliest_space {
                continue;
            }

            *prev_location = usize::MAX;
            for empty_id in earliest_space..id {
                if memory[empty_id * 2 + 1] - 48 < effected_size {
                    continue;
                }

                earliest_space_for_size[effected_size as usize] = empty_id;
                break;
            }
        }
    }

    sum_check

    // let mut memory_grid = parse_input(TEST);
    // let mut earliest_space_for_size = [u16::MAX; 10];
    //
    // for (index, tile) in memory_grid.iter().enumerate() {
    //     for size in 1..tile.empty_space + 1 {
    //         if earliest_space_for_size[size as usize] == u16::MAX {
    //             earliest_space_for_size[size as usize] = index as u16;
    //         }
    //     }
    // }
    //
    // for tile_id in (0..memory_grid.len()).rev() {
    //     // println!("{:?}", memory_grid);
    //     let size = memory_grid[tile_id].memory[0].size;
    //     let earliest_space = unsafe { *earliest_space_for_size.get_unchecked(size as usize) };
    //     if earliest_space >= tile_id as u16 {
    //         continue;
    //     }
    //
    //     // If we can move the tile to the earliest space,
    //     // We set move the tile then shift the span's memory the tile was in.
    //     let moved_data = &memory_grid[tile_id].shift().unwrap();
    //     let prev_size = memory_grid[earliest_space as usize].empty_space;
    //
    //     memory_grid[tile_id].empty_space -= moved_data.size;
    //     memory_grid[earliest_space as usize].push(*moved_data);
    //     memory_grid[tile_id - 1].empty_space += moved_data.size;
    //
    //     // Re-setting the earliest_space_for_size
    //     for effected_size in 1..prev_size + 1 {
    //         let prev_location = earliest_space_for_size[effected_size as usize];
    //         if prev_location < earliest_space {
    //             continue;
    //         }
    //
    //         earliest_space_for_size[effected_size as usize] = u16::MAX;
    //         for (check, tile) in memory_grid
    //             .iter()
    //             .enumerate()
    //             .take(tile_id)
    //             .skip(earliest_space as usize)
    //         {
    //             if tile.empty_space >= effected_size {
    //                 earliest_space_for_size[effected_size as usize] = check as u16;
    //                 break;
    //             }
    //         }
    //     }
    //
    //     if earliest_space_for_size.iter().all(|&x| x == u16::MAX) {
    //         println!("Earliest space for size: {:?}", earliest_space_for_size);
    //         break;
    //     }
    // }
    //
    // let mut total_score = 0;
    // let mut start_index = 0;
    // for tile in memory_grid {
    //     for file in tile.memory.iter() {
    //         let id = file.id;
    //         let size = file.size;
    //
    //         if id > 0 {
    //             let start = start_index as u64;
    //             let end = start_index as u64 + size as u64;
    //             let total_indices: u64 = (end * (end - 1) - start * (start - 1)) / 2;
    //
    //             total_score += total_indices * (id as u64);
    //         }
    //         start_index += size as u32;
    //     }
    //
    //     start_index += tile.empty_space as u32;
    // }
    //
    // total_score
}

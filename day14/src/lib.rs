//! --- Day 14: Restroom Redoubt ---
//! https://adventofcode.com/2024/day/14
//!
//! -- Part One --
//! This part is straightforward, we just need to move the robots by some number of steps.
//! We can do it pretty easily with `next_pos = pos + vel * steps` and `pos = next_pos % resolution`.
//! The Tricky part to make it go fast is the parser. We can just use normal string parser, it is too slow.
//! Also, instead of calculating which quarter they fall into, we just use a LUT to get it from [pos][vel]
//!
//! -- Part Two --
//! This part is faster because for this part we are using statistics!
//! The way to get the tree is finding the lowest variance for X and Y positions within the movements.
//! There are only 101 for X and 103 for Y possible steps, we loop through them and calculate the variance.
//! Thanks to error margin, we can get away with only calculating the first 80 robots instead of all 500.
//! We are also using 16 sized SIMD vectors to speed up the calculations.
//! In the end, we are doing Chinese Remainder Theorem to get the final position.

#![feature(portable_simd)]

use std::simd::cmp::SimdOrd;
use std::simd::prelude::SimdUint;
use std::simd::Simd;

/* ------------------- Helpers ------------------- */

const SIMD_SIZE: usize = 16;
const SAMPLE_SIZE: usize = 64 + 16;

const LUT_WIDTH: [[u8; 101]; 101] = {
    let mut result = [[4; 101]; 101];

    let mut x = 0;
    while x < 101 {
        let mut vx = 0;
        while vx < 101 {
            let pos = (x + vx * 100) % 101;
            if pos == 50 {
                result[x][vx] = 4;
            } else if pos < 50 {
                result[x][vx] = 0;
            } else {
                result[x][vx] = 2;
            }
            vx += 1;
        }
        x += 1;
    }
    result
};

const LUT_HEIGHT: [[u8; 103]; 103] = {
    let mut result = [[4; 103]; 103];

    let mut y = 0;
    while y < 103 {
        let mut vy = 0;
        while vy < 103 {
            let pos = (y + vy * 100) % 103;
            if pos == 51 {
                result[y][vy] = 4;
            } else if pos < 51 {
                result[y][vy] = 0;
            } else {
                result[y][vy] = 1;
            }
            vy += 1;
        }
        y += 1;
    }
    result
};

#[inline]
unsafe fn parse_robot(pointer: &mut *const u8) -> (u8, u8, u8, u8) {
    *pointer = pointer.add(3);

    let mut x = **pointer - b'0';
    *pointer = pointer.add(1);
    if **pointer != b',' {
        x = x * 10 + **pointer - b'0';
        *pointer = pointer.add(1);
    }
    if **pointer != b',' {
        x = x * 10 + **pointer - b'0';
        *pointer = pointer.add(1);
    }

    *pointer = pointer.add(1);
    let mut y = **pointer - b'0';
    *pointer = pointer.add(1);
    if **pointer != b' ' {
        y = y * 10 + **pointer - b'0';
        *pointer = pointer.add(1);
    }
    if **pointer != b' ' {
        y = y * 10 + **pointer - b'0';
        *pointer = pointer.add(1);
    }

    *pointer = pointer.add(3);

    let mut neg_x = false;
    let mut vx = **pointer - b'0';
    *pointer = pointer.add(1);
    if vx == u8::MAX - 2 {
        neg_x = true;
        vx = **pointer - b'0';
        *pointer = pointer.add(1);
    }
    if **pointer != b',' {
        vx = vx * 10 + **pointer - b'0';
        *pointer = pointer.add(1);
    }
    if neg_x {
        vx = 101 - vx;
    }

    *pointer = pointer.add(1);
    let mut neg_y = false;
    let mut vy = **pointer - b'0';
    *pointer = pointer.add(1);
    if vy == u8::MAX - 2 {
        neg_y = true;
        vy = **pointer - b'0';
        *pointer = pointer.add(1);
    }
    if **pointer != b'\n' {
        vy = vy * 10 + **pointer - b'0';
        *pointer = pointer.add(1);
    }
    if neg_y {
        vy = 103 - vy;
    }

    (x, y, vx, vy)
}

fn parse_input(
    bytes: &[u8],
) -> (
    Vec<Simd<u16, SIMD_SIZE>>,
    Vec<Simd<u16, SIMD_SIZE>>,
    Vec<Simd<u16, SIMD_SIZE>>,
    Vec<Simd<u16, SIMD_SIZE>>,
) {
    let mut x_positions = Vec::with_capacity(SAMPLE_SIZE / SIMD_SIZE);
    let mut y_positions = Vec::with_capacity(SAMPLE_SIZE / SIMD_SIZE);
    let mut x_vels = Vec::with_capacity(SAMPLE_SIZE / SIMD_SIZE);
    let mut y_vels = Vec::with_capacity(SAMPLE_SIZE / SIMD_SIZE);

    let mut x_pos = Vec::with_capacity(SIMD_SIZE);
    let mut y_pos = Vec::with_capacity(SIMD_SIZE);
    let mut x_vel = Vec::with_capacity(SIMD_SIZE);
    let mut y_vel = Vec::with_capacity(SIMD_SIZE);

    let mut pointer = bytes.as_ptr().wrapping_sub(1);
    let mut robot_count: usize = 0;
    while robot_count < SAMPLE_SIZE {
        let (x, y, vx, vy) = unsafe { parse_robot(&mut pointer) };
        robot_count += 1;

        x_pos.push(x as u16);
        y_pos.push(y as u16);
        x_vel.push(vx as u16);
        y_vel.push(vy as u16);

        if robot_count % SIMD_SIZE == 0 {
            x_positions.push(Simd::<u16, SIMD_SIZE>::from_slice(&x_pos));
            y_positions.push(Simd::<u16, SIMD_SIZE>::from_slice(&y_pos));
            x_vels.push(Simd::<u16, SIMD_SIZE>::from_slice(&x_vel));
            y_vels.push(Simd::<u16, SIMD_SIZE>::from_slice(&y_vel));

            x_pos.clear();
            y_pos.clear();
            x_vel.clear();
            y_vel.clear();
        }
    }

    (x_positions, y_positions, x_vels, y_vels)
}

/* ------------------- Solutions ------------------- */

pub fn part1(input: &[u8]) -> u32 {
    let mut quarter_vals: [u32; 4] = [0, 0, 0, 0];

    let mut pointer = input.as_ptr().wrapping_sub(1);
    let end_pointer = pointer.wrapping_add(input.len());
    while pointer < end_pointer {
        let (x, y, vx, vy) = unsafe { parse_robot(&mut pointer) };

        // Calculate the next positions using a simple movement formula.
        let dx = LUT_WIDTH[x as usize][vx as usize];
        let dy = LUT_HEIGHT[y as usize][vy as usize];
        let idx = dx + dy;
        if idx < 4 {
            quarter_vals[idx as usize] += 1;
        }
    }

    quarter_vals[0] * quarter_vals[1] * quarter_vals[2] * quarter_vals[3]
}

pub fn part2<const TEST: bool>(input: &[u8]) -> u32 {
    if TEST {
        return 0;
    }

    let width = Simd::<u16, SIMD_SIZE>::splat(101);
    let height = Simd::<u16, SIMD_SIZE>::splat(103);
    let zero16 = Simd::<u16, SIMD_SIZE>::splat(0);
    let zero32 = Simd::<u32, SIMD_SIZE>::splat(0);

    let mut robots = parse_input(&mut &*input);

    let threshold = 500 * SAMPLE_SIZE as u32;
    let mut lowest_x_score = threshold;
    let mut lowest_y_score = threshold;
    let mut x_steps: u32 = 0;
    let mut y_steps: u32 = 0;
    let mut wide_x_sum = Simd::<u16, SIMD_SIZE>::splat(0);
    let mut wide_y_sum = Simd::<u16, SIMD_SIZE>::splat(0);
    let mut wide_x_square_sum = Simd::<u32, SIMD_SIZE>::splat(0);
    let mut wide_y_square_sum = Simd::<u32, SIMD_SIZE>::splat(0);
    for step_count in 1..103 {
        for idx in 0..SAMPLE_SIZE / SIMD_SIZE {
            let x_pos = unsafe { robots.0.get_unchecked_mut(idx) };
            let y_pos = unsafe { robots.1.get_unchecked_mut(idx) };
            let x_vel = unsafe { robots.2.get_unchecked(idx) };
            let y_vel = unsafe { robots.3.get_unchecked(idx) };

            *x_pos += *x_vel;
            *y_pos += *y_vel;
            *x_pos = x_pos.simd_min(*x_pos - width);
            *y_pos = y_pos.simd_min(*y_pos - height);

            wide_x_sum += *x_pos;
            wide_y_sum += *y_pos;
            wide_x_square_sum += x_pos.cast::<u32>() * x_pos.cast::<u32>();
            wide_y_square_sum += y_pos.cast::<u32>() * y_pos.cast::<u32>();
        }

        let x_sum = wide_x_sum.reduce_sum() as u32;
        let y_sum = wide_y_sum.reduce_sum() as u32;
        let x_square_sum = wide_x_square_sum.reduce_sum();
        let y_square_sum = wide_y_square_sum.reduce_sum();

        let x_var = x_square_sum - x_sum * x_sum / SAMPLE_SIZE as u32;
        let y_var = y_square_sum - y_sum * y_sum / SAMPLE_SIZE as u32;

        if step_count < 101 && x_var < lowest_x_score {
            lowest_x_score = x_var;
            x_steps = step_count as u32;
        }
        if step_count < 103 && y_var < lowest_y_score {
            lowest_y_score = y_var;
            y_steps = step_count as u32;
        }

        if x_steps != 0 && y_steps != 0 {
            break;
        }

        wide_x_sum = wide_x_sum.simd_min(zero16);
        wide_y_sum = wide_y_sum.simd_min(zero16);
        wide_x_square_sum = wide_x_square_sum.simd_min(zero32);
        wide_y_square_sum = wide_y_square_sum.simd_min(zero32);
    }

    (51 * (x_steps * 103 + y_steps * 101)) % (101 * 103)
}

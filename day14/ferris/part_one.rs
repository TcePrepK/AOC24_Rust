//! --- Day 14: Restroom Redoubt ---
//! https://adventofcode.com/2024/day/14
//!
//! Code for leaderboard, ferris-elf!

/* ------------------- Helpers ------------------- */
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

#[inline(always)]
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

/* ------------------- Solutions ------------------- */

pub fn run(input: &[u8]) -> u32 {
    let mut quarter_vals = [0, 0, 0, 0];

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

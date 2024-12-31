use utils::test_solutions;

fn main() {
    test_solutions(25, &first_part, Some(3), &second_part, None);
}

/* ------------------- Helpers ------------------- */

fn parse_input(input: &str) -> (Vec<[u32; 5]>, Vec<[u32; 5]>) {
    let mut lock_pins = vec![];
    let mut key_heights = vec![];

    let mut current_grid = [0; 5];
    let mut in_grid = false;
    let mut top_grid = false;

    let mut iter = input.lines();
    loop {
        let line_opt = iter.next();
        if line_opt.is_none() || line_opt.unwrap() == "" {
            if top_grid {
                lock_pins.push(current_grid);
            } else {
                key_heights.push(current_grid);
            }

            in_grid = false;
            current_grid = [0; 5];

            if line_opt.is_none() {
                break;
            }
            continue;
        }

        let line = line_opt.unwrap();
        if !in_grid {
            if line == "#####" {
                in_grid = true;
                top_grid = true;
            } else {
                in_grid = true;
                top_grid = false;
            }
        }

        let chars = line.chars().collect::<Vec<char>>();
        for i in 0..5 {
            if chars[i] == '#' {
                current_grid[i] += 1;
            }
        }
    }

    (lock_pins, key_heights)
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u32 {
    let (lock_pins, key_heights) = parse_input(input);

    let mut unique_pairs = 0;
    for lock in lock_pins {
        for key in key_heights.iter() {
            if key.iter().enumerate().all(|(i, k)| k + lock[i] <= 7) {
                unique_pairs += 1;
            }
        }
    }

    unique_pairs
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i32 {
    0
}

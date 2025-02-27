use glam::{IVec2, UVec2};
use std::collections::HashMap;
use utils::test_solutions;

fn main() {
    test_solutions(8, &first_part, Some(14), &second_part, Some(34));
}

/* ------------------- Helpers ------------------- */

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_frequencies(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<UVec2>> {
    let height = grid.len();
    let width = grid[0].len();

    let mut frequencies: HashMap<char, Vec<UVec2>> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            match grid[y][x] {
                '.' => {}
                c => {
                    if !frequencies.contains_key(&c) {
                        frequencies.insert(c, vec![]);
                    }
                    let arr: &mut Vec<UVec2> = frequencies.get_mut(&c).unwrap();
                    arr.push(UVec2::new(x as u32, y as u32));
                }
            }
        }
    }

    frequencies
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i32 {
    let mut grid = get_grid(input);
    let frequencies = get_frequencies(&grid);

    let height = grid.len();
    let width = grid[0].len();

    let mut total: i32 = 0;
    for (c, arr) in frequencies.iter() {
        for i in 0..arr.len() {
            for j in (i + 1)..arr.len() {
                let a: IVec2 = arr[i].as_ivec2();
                let b: IVec2 = arr[j].as_ivec2();
                let ps = [2 * a - b, 2 * b - a];

                for p in ps.iter() {
                    let (x, y) = (p.x as usize, p.y as usize);
                    if x >= width || y >= height {
                        continue;
                    }

                    if grid[y][x] != '#' {
                        grid[y][x] = '#';
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i32 {
    let mut grid = get_grid(input);
    let frequencies = get_frequencies(&grid);

    let height = grid.len();
    let width = grid[0].len();

    let mut total: i32 = 0;
    for (c, arr) in frequencies.iter() {
        for i in 0..arr.len() {
            for j in (i + 1)..arr.len() {
                let a: IVec2 = arr[i].as_ivec2();
                let b: IVec2 = arr[j].as_ivec2();
                let d = b - a;

                let mut back_idx = 0;
                let mut front_idx = 0;
                while back_idx >= 0 || front_idx >= 0 {
                    if back_idx >= 0 {
                        let p = a - d * back_idx;
                        let (x, y) = (p.x as usize, p.y as usize);
                        if x >= width || y >= height {
                            back_idx = -1;
                            continue;
                        }

                        if grid[y][x] != '#' {
                            grid[y][x] = '#';
                            total += 1;
                        }

                        back_idx += 1;
                    }

                    if front_idx >= 0 {
                        let p = b + d * front_idx;
                        let (x, y) = (p.x as usize, p.y as usize);
                        if x >= width || y >= height {
                            front_idx = -1;
                            continue;
                        }

                        if grid[y][x] != '#' {
                            grid[y][x] = '#';
                            total += 1;
                        }

                        front_idx += 1;
                    }
                }
            }
        }
    }

    total
}

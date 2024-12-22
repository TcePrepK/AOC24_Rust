use std::fs;

fn read_file(path: &str) -> String {
    if fs::exists(path).expect("Could not check file") {
        fs::read_to_string(path).expect("Could not read file")
    } else {
        String::new()
    }
}

const ANSWER_ONE: u32 = 0;
const ANSWER_TWO: u32 = 0;

fn test_examples() -> (bool, bool) {
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

    let results = (first_part(&example_1), second_part(&example_2));

    if results.0 != 0 && results.0 != ANSWER_ONE {
        println!("Part One Wrong");
    }

    if results.1 != 0 && results.1 != ANSWER_TWO {
        println!("Part Two Wrong");
    }

    (results.0 == ANSWER_ONE, results.1 == ANSWER_TWO)
}

fn test_inputs(example_solutions: (bool, bool)) {
    let input = read_file("src/input");

    if example_solutions.0 {
        let start_time = std::time::Instant::now();
        let result = first_part(&input);
        let total_time = start_time.elapsed();
        println!("Part 1 result: {}, took: {:?}", result, total_time);
    }
    if example_solutions.1 {
        let start_time = std::time::Instant::now();
        let result = second_part(&input);
        let total_time = start_time.elapsed();
        println!("Part 2 result: {}, took: {:?}", result, total_time);
    }
}

fn main() {
    let example_solutions = test_examples();
    test_inputs(example_solutions);
}

/* ------------------- Helpers ------------------- */

/// Turns 2D grid input into 2D vector of chars.
fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

/// Finds the start and end of the given map.
fn find_start_end(grid: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                start = (x, y);
            }
            if grid[y][x] == 'E' {
                end = (x, y);
            }
        }
    }

    (start, end)
}

/// Calculates the weight grid, weight represents the number of tiles it takes to get to that tile.
fn get_weight_grid_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> (Vec<Vec<u32>>, Vec<((usize, usize), u32)>) {
    let size = grid.len();
    let mut weight_grid: Vec<Vec<u32>> = vec![vec![u32::MAX; size]; size];

    let mut path: Vec<((usize, usize), u32)> = vec![(start, 0)];
    loop {
        let (tile_pos, tile_weight) = path.last().unwrap();
        weight_grid[tile_pos.1][tile_pos.0] = *tile_weight;

        if *tile_pos == end {
            break;
        }

        for dir in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
            let (nx, ny) = (tile_pos.0 as i32 + dir.0, tile_pos.1 as i32 + dir.1);
            if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
                continue;
            }

            if grid[ny as usize][nx as usize] == '#' {
                continue;
            }

            let nw = weight_grid[ny as usize][nx as usize];
            if nw != u32::MAX {
                continue;
            }

            path.push(((nx as usize, ny as usize), tile_weight + 1));
            break;
        }
    }

    (weight_grid, path)
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u32 {
    let grid = get_grid(input);
    let (start, end) = find_start_end(&grid);
    let (weight_grid, path) = get_weight_grid_path(&grid, start, end);

    let mut total_skips = 0;
    for i in 0..(path.len() - 5) {
        let ((x, y), cw) = path[i];
        for offset in [(2, 0), (0, 2), (-2, 0), (0, -2)] {
            let (nx, ny) = (x as i32 + offset.0, y as i32 + offset.1);
            if nx < 0
                || ny < 0
                || nx >= weight_grid.len() as i32
                || ny >= weight_grid[0].len() as i32
            {
                continue;
            }

            let next_weight = weight_grid[ny as usize][nx as usize];
            if next_weight == u32::MAX {
                continue;
            }

            if next_weight > cw && next_weight - cw >= 100 + 2 {
                total_skips += 1;
            }
        }
    }

    total_skips
}

#[allow(unused_variables)]
fn second_part(input: &str) -> u32 {
    let grid = get_grid(input);
    let (start, end) = find_start_end(&grid);
    let (weight_grid, path) = get_weight_grid_path(&grid, start, end);

    let mut total_skips = 0;
    for i in 0..(path.len() - 5) {
        let ((x, y), cw) = path[i];
        for ox in -20_i32..21 {
            for oy in (ox.abs() - 20)..(21 - ox.abs()) {
                let dist = ox.abs() + oy.abs();
                if dist < 2 {
                    continue;
                }

                let (nx, ny) = (x as i32 + ox, y as i32 + oy);
                if nx < 0
                    || ny < 0
                    || nx >= weight_grid.len() as i32
                    || ny >= weight_grid[0].len() as i32
                {
                    continue;
                }

                let next_weight = weight_grid[ny as usize][nx as usize];
                if next_weight == u32::MAX {
                    continue;
                }

                if next_weight > cw && next_weight - cw >= (100 + dist) as u32 {
                    total_skips += 1;
                }
            }
        }
    }

    total_skips
}

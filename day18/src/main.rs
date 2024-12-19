use std::fs;

fn read_file(path: &str) -> String {
    if fs::exists(path).expect("Could not check file") {
        fs::read_to_string(path).expect("Could not read file")
    } else {
        String::new()
    }
}

const ANSWER_ONE: i32 = 22;
const ANSWER_TWO: (usize, usize) = (6, 1);

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

    if results.1 != (0, 0) && results.1 != ANSWER_TWO {
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
        println!("Part 2 result: {:?}, took: {:?}", result, total_time);
    }
}

fn main() {
    let example_solutions = test_examples();
    test_inputs(example_solutions);
}

/* ------------------- Helpers ------------------- */

/// Parses the input (lines of wall coordinates) and returns a vector of wall coordinates.
fn get_walls(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|x| x.parse::<usize>().unwrap());
            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect::<Vec<(usize, usize)>>()
}

/// Uses the wall coordinates to create a grid.
fn get_grid(input: &str, mut input_limit: usize) -> Vec<Vec<char>> {
    let walls = get_walls(input);
    let size: usize = if walls.len() == 25 { 7 } else { 71 };
    let mut grid = vec![vec!['.'; size]; size];

    if walls.len() == 25 {
        // Have to do for the example input
        input_limit = 12;
    }

    for i in 0..input_limit {
        if i >= walls.len() {
            break;
        }

        let wall = walls[i];
        grid[wall.1][wall.0] = '#';
    }

    grid
}

/// Calculates the weight grid, weight represents the number of tiles it takes to get to that tile.
fn get_weight_grid(grid: &mut Vec<Vec<char>>) -> Vec<Vec<Option<(i32, (usize, usize))>>> {
    let size = grid.len();
    let mut weight_grid: Vec<Vec<Option<(i32, (usize, usize))>>> = vec![vec![None; size]; size];

    let directions = [(1, 0), (0, 1), (0, -1), (-1, 0)];
    let mut tiles: Vec<((usize, usize), i32, (usize, usize))> = vec![((0, 0), 0, (0, 0))];

    while !tiles.is_empty() {
        let (tile_pos, tile_weight, tile_prev) = tiles.remove(0);

        if weight_grid[tile_pos.1][tile_pos.0].is_some_and(|tile| tile.0 <= tile_weight) {
            continue;
        }
        weight_grid[tile_pos.1][tile_pos.0] = Some((tile_weight, tile_prev));

        if tile_pos.0 == size - 1 && tile_pos.1 == size - 1 {
            break;
        }

        let mut neighbor_exists = false;
        for dir in directions {
            let (nx, ny) = (tile_pos.0 as i32 + dir.0, tile_pos.1 as i32 + dir.1);
            if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
                continue;
            }

            if grid[ny as usize][nx as usize] == '#' {
                continue;
            }

            neighbor_exists = true;
            tiles.push(((nx as usize, ny as usize), tile_weight + 1, tile_pos));
        }

        if !neighbor_exists {
            grid[tile_pos.1][tile_pos.0] = '#';
        }
    }

    weight_grid
}

/// Finds the path from end to start if it exists.
fn get_path(weight_grid: &Vec<Vec<Option<(i32, (usize, usize))>>>) -> Option<Vec<(usize, usize)>> {
    let size = weight_grid.len();
    let mut end_tile_pos = (size - 1, size - 1);
    let mut end_tile = weight_grid[end_tile_pos.1][end_tile_pos.0];
    if end_tile.is_none() {
        None
    } else {
        let mut path: Vec<(usize, usize)> = vec![];
        loop {
            let tile = end_tile.unwrap();
            path.push(end_tile_pos);
            if end_tile_pos.0 == 0 && end_tile_pos.1 == 0 {
                break;
            }

            let prev = tile.1;
            end_tile_pos = prev;
            end_tile = weight_grid[end_tile_pos.1][end_tile_pos.0];
        }

        path.reverse();
        Some(path)
    }
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i32 {
    let mut grid = get_grid(input, 1024);
    let weighted_grid = get_weight_grid(&mut grid);

    let size = weighted_grid.len();
    if let Some((weight, _)) = weighted_grid[size - 1][size - 1] {
        weight
    } else {
        panic!("No path found");
    }
}

#[allow(unused_variables)]
fn second_part(input: &str) -> (usize, usize) {
    let walls = get_walls(input);
    let mut grid = get_grid(input, 1024);
    let mut weighted_grid = get_weight_grid(&mut grid);
    let mut path = get_path(&weighted_grid).expect("No path found");

    let mut wall_index = if walls.len() == 25 { 12 } else { 1024 };
    while wall_index < walls.len() {
        let wall = walls[wall_index];
        grid[wall.1][wall.0] = '#';
        wall_index += 1;

        if !path.contains(&(wall.0, wall.1)) {
            continue;
        }

        weighted_grid = get_weight_grid(&mut grid);
        path = get_path(&weighted_grid).unwrap_or(vec![]);
        if path.len() == 0 {
            break;
        }
    }

    walls[wall_index - 1]
}

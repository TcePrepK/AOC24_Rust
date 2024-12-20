use std::collections::HashSet;
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i32 = 11048;
const ANSWER_TWO: i32 = 64;

fn test_examples() -> [bool; 2] {
    let example = read_file("src/example");

    let results = [first_part(&example), second_part(&example)];

    if results[0] > 0 && results[0] != ANSWER_ONE {
        println!("Part One Wrong");
    }

    if results[1] > 0 && results[1] != ANSWER_TWO {
        println!("Part Two Wrong");
    }

    [results[0] == ANSWER_ONE, results[1] == ANSWER_TWO]
}

fn test_inputs(example_solutions: [bool; 2]) {
    let input = read_file("src/input");

    if example_solutions[0] {
        let start_time = std::time::Instant::now();
        let result = first_part(&input);
        let total_time = start_time.elapsed();
        println!("Part 1 result: {}, took: {:?}", result, total_time);
    }
    if example_solutions[1] {
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

#[derive(Debug, Eq, Hash, PartialEq)]
enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

const DIRS: [DIRECTION; 4] = [
    DIRECTION::UP,
    DIRECTION::DOWN,
    DIRECTION::LEFT,
    DIRECTION::RIGHT,
];

impl DIRECTION {
    fn to_usize(&self) -> usize {
        match self {
            DIRECTION::UP => 0,
            DIRECTION::DOWN => 1,
            DIRECTION::LEFT => 2,
            DIRECTION::RIGHT => 3,
        }
    }
}

/// The input consists of a grid where S is start and E is end.
fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_start_end(grid: &Vec<Vec<char>>) -> ((i32, i32), (i32, i32)) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                start = (x as i32, y as i32);
            }
            if grid[y][x] == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    (start, end)
}

fn generate_weighted_map(grid: &Vec<Vec<char>>, start: (i32, i32)) -> Vec<Vec<[i32; 4]>> {
    let mut weighted_map: Vec<Vec<[i32; 4]>> = vec![vec![[-1; 4]; grid[0].len()]; grid.len()];
    weighted_map[start.1 as usize][start.0 as usize] = [0; 4];

    let mut leaves: Vec<(i32, i32, &DIRECTION, i32)> =
        vec![(start.0, start.1, &DIRECTION::RIGHT, 0)];
    while !leaves.is_empty() {
        let leaf = leaves.pop().unwrap();
        let leaf_weight = leaf.3;

        let &mut mut weights = &mut weighted_map[leaf.1 as usize][leaf.0 as usize];

        let mut min_weight = weights[0];
        for i in 1..weights.len() {
            if weights[i] < min_weight {
                min_weight = weights[i];
            }
        }

        for i in 0..weights.len() {
            if i as i32 == min_weight {
                weights[i] = leaf_weight;
            } else {
                weights[i] = leaf_weight + 1000;
            }
        }

        for dir in DIRS.iter() {
            let neighbor = match dir {
                DIRECTION::UP => (leaf.0, leaf.1 - 1),
                DIRECTION::DOWN => (leaf.0, leaf.1 + 1),
                DIRECTION::LEFT => (leaf.0 - 1, leaf.1),
                DIRECTION::RIGHT => (leaf.0 + 1, leaf.1),
            };

            if grid[neighbor.1 as usize][neighbor.0 as usize] == '#' {
                continue;
            }

            let cur_weight = weighted_map[neighbor.1 as usize][neighbor.0 as usize][dir.to_usize()];
            let next_weight = leaf_weight + if dir == leaf.2 { 1 } else { 1001 };

            if cur_weight < 0 || cur_weight > next_weight {
                weighted_map[neighbor.1 as usize][neighbor.0 as usize][dir.to_usize()] =
                    next_weight;
                leaves.push((neighbor.0, neighbor.1, &dir, next_weight));
            } else if cur_weight == next_weight {
                weighted_map[neighbor.1 as usize][neighbor.0 as usize][dir.to_usize()] =
                    next_weight;
            }
        }
    }

    weighted_map
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i32 {
    let grid = get_grid(input);
    let (start, end) = find_start_end(&grid);
    let weighted_map = generate_weighted_map(&grid, start);

    let end_weights = weighted_map[end.1 as usize][end.0 as usize];

    *end_weights.iter().filter(|p| **p > 0).min().unwrap()
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i32 {
    let grid = get_grid(input);
    let (start, end) = find_start_end(&grid);
    let weighted_map = generate_weighted_map(&grid, start);

    let end_weight = *weighted_map[end.1 as usize][end.0 as usize]
        .iter()
        .filter(|p| **p > 0)
        .min()
        .unwrap();

    let mut finished_paths: Vec<Vec<(i32, i32, &DIRECTION)>> = vec![];
    let mut paths: Vec<Vec<(i32, i32, &DIRECTION)>> =
        vec![vec![(start.0, start.1, &DIRECTION::RIGHT)]];
    while !paths.is_empty() {
        let path = paths.pop().unwrap();
        let leaf = path.last().unwrap();
        let leaf_weight = weighted_map[leaf.1 as usize][leaf.0 as usize][leaf.2.to_usize()];

        let mut neighbors: Vec<(i32, i32, &DIRECTION)> = vec![];
        for dir in DIRS.iter() {
            let neighbor = match dir {
                DIRECTION::UP => (leaf.0, leaf.1 - 1),
                DIRECTION::DOWN => (leaf.0, leaf.1 + 1),
                DIRECTION::LEFT => (leaf.0 - 1, leaf.1),
                DIRECTION::RIGHT => (leaf.0 + 1, leaf.1),
            };

            if grid[neighbor.1 as usize][neighbor.0 as usize] == '#' {
                continue;
            }

            neighbors.push((neighbor.0, neighbor.1, dir));
        }

        neighbors.retain(|n| {
            let mut weight = weighted_map[n.1 as usize][n.0 as usize][n.2.to_usize()];
            if n.0 == end.0 && n.1 == end.1 {
                weight = end_weight;
            }
            weight - leaf_weight == 1 || weight - leaf_weight == 1001
        });

        for neighbor in neighbors.iter() {
            let mut clone_path = path.clone();
            clone_path.push(*neighbor);

            let neighbor_weight =
                weighted_map[neighbor.1 as usize][neighbor.0 as usize][neighbor.2.to_usize()];
            if neighbor.0 == end.0 && neighbor.1 == end.1 {
                // if neighbor_weight == end_weight {
                //     continue;
                // }
                finished_paths.push(clone_path);
            } else {
                paths.push(clone_path);
            }
        }
    }

    let mut hash_set: HashSet<(i32, i32)> = HashSet::new();
    for path in finished_paths {
        for tile in path {
            hash_set.insert((tile.0, tile.1));
        }
    }
    let different_tiles = hash_set.len();

    different_tiles as i32

    // 502 >>
}

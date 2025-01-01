use rayon::prelude::*;
use utils::{test_solutions, Grid, Point, DIRECTIONS};

fn main() {
    test_solutions(16, &first_part, Some(11048), &second_part, Some(64));
}

/* ------------------- Helpers ------------------- */

fn parse_input(input: &str) -> (Grid<u8>, Point) {
    let grid = Grid::parse(input);
    let start = grid.find(b'S').unwrap();
    (grid, start)
}

/* ------------------- Solutions ------------------- */

fn first_part(input: &str) -> u32 {
    let (grid, start) = parse_input(input);

    let mut weighted_map = grid.clone_with(u32::MAX);
    let mut priority_leaves: Vec<(Point, usize, u32)> = vec![(start, 0, 0)];
    let mut secondary_leaves: Vec<(Point, usize, u32)> = vec![];
    weighted_map[start] = 0;

    let mut end_weight = u32::MAX;
    while !priority_leaves.is_empty() {
        let (leaf_pos, leaf_dir, leaf_weight) = priority_leaves.pop().unwrap();

        if leaf_weight >= end_weight {
            continue;
        }

        if grid[leaf_pos] == b'E' {
            end_weight = leaf_weight;
            continue;
        }

        // Directional vectors
        let left = (leaf_dir + 3) % 4;
        let right = (leaf_dir + 1) % 4;
        let neighbors_check = [
            (leaf_pos + DIRECTIONS[leaf_dir], leaf_dir, leaf_weight + 1),
            (leaf_pos + DIRECTIONS[left], left, leaf_weight + 1001),
            (leaf_pos + DIRECTIONS[right], right, leaf_weight + 1001),
        ];

        for neighbor in neighbors_check {
            let (next_position, next_direction, next_weight) = neighbor;
            if grid[next_position] == b'#' {
                continue;
            }

            let prev_weight = &mut weighted_map[next_position];
            if *prev_weight < next_weight {
                continue;
            }
            *prev_weight = next_weight;

            if next_direction == leaf_dir {
                priority_leaves.push(neighbor);
            } else {
                secondary_leaves.push(neighbor);
            }
        }

        if priority_leaves.is_empty() {
            let temp = priority_leaves;
            priority_leaves = secondary_leaves;
            secondary_leaves = temp;
        }
    }

    end_weight
}

fn second_part(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let end = grid.find(b'E').unwrap();

    let mut weighted_map = grid.clone_with([u32::MAX; 4]);

    let mut priority_leaves: Vec<(Point, usize, u32)> = vec![(start, 0, 0)];
    let mut secondary_leaves: Vec<(Point, usize, u32)> = vec![];
    weighted_map[start][0] = 0;

    let mut end_weight = u32::MAX;
    while !priority_leaves.is_empty() {
        let (leaf_pos, leaf_dir, leaf_weight) = priority_leaves.pop().unwrap();

        if leaf_weight >= end_weight {
            continue;
        }

        if leaf_pos == end {
            end_weight = leaf_weight;
            continue;
        }

        // Directional vectors
        let left = (leaf_dir + 3) % 4;
        let right = (leaf_dir + 1) % 4;
        let neighbors_check = [
            (leaf_pos + DIRECTIONS[leaf_dir], leaf_dir, leaf_weight + 1),
            (leaf_pos, left, leaf_weight + 1000),
            (leaf_pos, right, leaf_weight + 1000),
        ];

        for neighbor in neighbors_check {
            let (next_position, next_direction, next_weight) = neighbor;
            if grid[next_position] == b'#' {
                continue;
            }

            let prev_weight = &mut weighted_map[next_position][next_direction];
            if *prev_weight < next_weight {
                continue;
            }
            *prev_weight = next_weight;

            if next_direction == leaf_dir {
                priority_leaves.push(neighbor);
            } else {
                secondary_leaves.push(neighbor);
            }
        }

        if priority_leaves.is_empty() {
            let temp = priority_leaves;
            priority_leaves = secondary_leaves;
            secondary_leaves = temp;
        }
    }

    let mut tiles_to_check: Vec<(Point, usize, u32)> = vec![];
    for dir in 0..4 {
        tiles_to_check.push((end, dir, end_weight));
    }

    let mut seen_tiles = grid.clone_with(false);
    while !tiles_to_check.is_empty() {
        let (leaf_pos, leaf_dir, leaf_weight) = tiles_to_check.pop().unwrap();

        seen_tiles[leaf_pos] = true;
        if leaf_pos == start {
            continue;
        }

        // Directional vectors
        let left = (leaf_dir + 3) % 4;
        let right = (leaf_dir + 1) % 4;
        let next_pos = leaf_pos - DIRECTIONS[leaf_dir];

        let neighbors_check = [
            (next_pos, leaf_dir, leaf_weight - 1),
            (leaf_pos, left, leaf_weight - 1000),
            (leaf_pos, right, leaf_weight - 1000),
        ];

        for neighbor in neighbors_check {
            let (next_position, next_direction, next_weight) = neighbor;

            let seen_weight = weighted_map[next_position][next_direction];
            if seen_weight != next_weight {
                continue;
            }

            weighted_map[next_position][next_direction] = u32::MAX;
            tiles_to_check.push(neighbor);
        }
    }

    seen_tiles.data.par_iter().filter(|&x| *x).count()
}

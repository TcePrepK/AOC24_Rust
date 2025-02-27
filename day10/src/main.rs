use std::collections::HashSet;
use utils::test_solutions;

fn main() {
    test_solutions(10, &first_part, Some(36), &second_part, Some(81));
}

/* ------------------- Helpers ------------------- */

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn find_neighbors(grid: &Vec<Vec<usize>>, x: usize, y: usize) -> Vec<(usize, usize, usize)> {
    let mut result: Vec<(usize, usize, usize)> = vec![];

    for i in -1_i32..2 {
        for j in -1_i32..2 {
            if i == 0 && j == 0 {
                continue;
            }
            if i * j == 1 || i * j == -1 {
                continue;
            }

            let x = (x as i32 + i) as usize;
            let y = (y as i32 + j) as usize;
            if y >= grid.len() || x >= grid[y].len() {
                continue;
            }

            let n = grid[y][x];
            result.push((x, y, n));
        }
    }

    result
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i32 {
    let grid: Vec<Vec<usize>> = parse_input(input);

    let width = grid[0].len();
    let height = grid.len();

    let mut paths: Vec<(usize, usize, usize)> = vec![];
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 0 {
                paths.push((x, y, 0));
            }
        }
    }

    let mut result: i32 = 0;
    while !paths.is_empty() {
        let (sx, sy, _) = paths.pop().unwrap();

        let mut trails: Vec<(usize, usize, usize)> = vec![(sx, sy, 0)];
        for i in 0..9 {
            let mut next_trails: Vec<(usize, usize, usize)> = vec![];
            for (x, y, n) in trails.iter() {
                let mut neighbors = find_neighbors(&grid, *x, *y);
                neighbors.retain(|(_, _, nn)| *nn == i + 1);
                next_trails.extend(&neighbors);
            }
            trails = next_trails;
        }

        result += HashSet::<(usize, usize, usize)>::from_iter(trails.iter().cloned()).len() as i32;
    }

    result
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i32 {
    let grid: Vec<Vec<usize>> = parse_input(input);

    let width = grid[0].len();
    let height = grid.len();

    let mut paths: Vec<(usize, usize, usize)> = vec![];
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 0 {
                paths.push((x, y, 0));
            }
        }
    }

    let mut result: i32 = 0;
    while !paths.is_empty() {
        let (x, y, n) = paths.pop().unwrap();
        if n == 9 {
            result += 1;
            continue;
        }

        let mut neighbors = find_neighbors(&grid, x, y);
        neighbors.retain(|(_, _, nn)| *nn == n + 1);
        paths.extend(&neighbors);
    }

    result
}

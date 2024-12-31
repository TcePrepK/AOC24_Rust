use ahash::AHashSet;
use rayon::prelude::*;
use utils::test_solutions;

fn main() {
    test_solutions(6, &first_part, Some(41), &second_part, Some(6));
}

/* ------------------- Helpers ------------------- */

fn dir_to_vec(dir: u8) -> (i32, i32) {
    [(0, -1), (1, 0), (0, 1), (-1, 0)][dir as usize]
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(input.lines().count());
    let mut guard: (usize, usize) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut subgrid: Vec<char> = vec![];
        for (x, char) in line.chars().enumerate() {
            subgrid.push(char);

            if char == '^' {
                guard = (x, y);
            }
        }
        grid.push(subgrid);
    }

    (grid, guard)
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u32 {
    let (mut grid, mut guard) = parse_input(input);
    let height: usize = grid.len();
    let width: usize = grid[0].len();

    let mut total = 0;

    let mut direction: (i32, i32) = (0, -1);
    loop {
        let (fx, fy) = guard;

        if grid[fy][fx] != 'x' {
            total += 1;
        }
        grid[fy][fx] = 'x';

        let (dx, dy) = direction;
        let (nx, ny) = ((fx as i32 + dx) as usize, ((fy as i32) + dy) as usize);
        if nx >= width || ny >= height {
            break;
        }

        match grid[ny][nx] {
            '#' => {
                direction = (-dy, dx);
                guard = (fx, fy);
            }
            _ => {
                guard = (nx, ny);
            }
        }
    }

    total
}

#[allow(unused_variables)]
fn second_part(input: &str) -> u32 {
    let (grid, main_guard) = parse_input(input);
    let height: usize = grid.len();
    let width: usize = grid[0].len();

    // Run the guard once and store the path.
    let mut guard = main_guard.clone();

    let mut main_path: AHashSet<(usize, usize)> = AHashSet::new();
    let mut direction: u8 = 0;
    loop {
        let (fx, fy) = guard;
        let (dx, dy) = dir_to_vec(direction);

        let (nx, ny) = ((fx as i32 + dx) as usize, ((fy as i32) + dy) as usize);
        if nx >= width || ny >= height {
            break;
        }

        match grid[ny][nx] {
            '#' => {
                direction = (direction + 1) % 4;
                guard = (fx, fy);
            }
            '.' => {
                guard = (nx, ny);
                main_path.insert(guard.clone());
            }
            _ => {
                guard = (nx, ny);
            }
        }
    }

    let looping_paths = main_path.par_iter().filter(|(x, y)| {
        let mut guard = main_guard.clone();

        let mut visited: Vec<Vec<i8>> = vec![vec![0; width]; height];
        let mut direction: u8 = 0;

        loop {
            let (fx, fy) = guard;
            let (dx, dy) = dir_to_vec(direction);

            if visited[fy][fx] > 2 {
                return true;
            }

            visited[fy][fx] += 1;

            let (nx, ny) = ((fx as i32 + dx) as usize, ((fy as i32) + dy) as usize);
            if nx >= width || ny >= height {
                return false;
            }

            if ny == *y && nx == *x {
                direction = (direction + 1) % 4;
                guard = (fx, fy);
            } else {
                match grid[ny][nx] {
                    '#' => {
                        direction = (direction + 1) % 4;
                        guard = (fx, fy);
                    }
                    _ => {
                        guard = (nx, ny);
                    }
                }
            }
        }
    });

    looping_paths.count() as u32
}

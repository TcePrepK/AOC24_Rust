use std::collections::{HashSet};
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

#[derive(Debug)]
enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn dir_to_vec(dir: &DIRECTION) -> (i32, i32) {
    match dir {
        DIRECTION::UP => (0, -1),
        DIRECTION::RIGHT => (1, 0),
        DIRECTION::DOWN => (0, 1),
        DIRECTION::LEFT => (-1, 0),
    }
}

fn rot_dir(dir: &DIRECTION) -> DIRECTION {
    match dir {
        DIRECTION::UP => DIRECTION::RIGHT,
        DIRECTION::RIGHT => DIRECTION::DOWN,
        DIRECTION::DOWN => DIRECTION::LEFT,
        DIRECTION::LEFT => DIRECTION::UP,
    }
}

#[allow(unused_variables)]
fn main() {
    let example = read_file("src/example");
    let input = read_file("src/input");
    let used_string = input;

    println!("First Part: {:?}", first_part(&used_string));
    println!("Second Part: {:?}", second_part(&used_string));
}

#[allow(unused_variables)]
fn first_part(input: &str) -> i32 {
    let mut grid: Vec<Vec<char>> = vec![];
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
    let height: usize = grid.len();
    let width: usize = grid[0].len();

    let mut total: i32 = 0;

    let mut direction: (i32, i32) = (0, -1);
    loop {
        let (fx, fy) = guard;

        if grid[fy][fx] != 'x' { total += 1; }
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
            },
            _ => {
                guard = (nx, ny);
            }
        }
    }

    total
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i32 {
    let mut main_grid: Vec<Vec<char>> = vec![];
    let mut main_guard: (usize, usize) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut subgrid: Vec<char> = vec![];
        for (x, char) in line.chars().enumerate() {
            subgrid.push(char);

            if char == '^' {
                main_guard = (x, y);
            }
        }
        main_grid.push(subgrid);
    }
    let height: usize = main_grid.len();
    let width: usize = main_grid[0].len();

    let mut loops: i32 = 0;
    for i in 1..height {
        for j in 1..width {
            if main_grid[i][j] != '.' { continue; }

            // Loop through all tiles and check if they are empty or not.
            // If they are empty, assume there is a 'O' and run the algorithm again.

            let mut grid = main_grid.clone();
            let mut guard = main_guard.clone();
            grid[i][j] = '#';

            let mut pos_directory: HashSet<String> = HashSet::new();
            let mut direction: DIRECTION = DIRECTION::UP;

            loop {
                let (fx, fy) = guard;
                let (dx, dy) = dir_to_vec(&direction);

                let id = format!("{:?}, {:?}, {:?})", fx, fy, direction);
                if pos_directory.contains(&id) {
                    loops += 1;
                    break;
                }

                pos_directory.insert(id);

                let (nx, ny) = ((fx as i32 + dx) as usize, ((fy as i32) + dy) as usize);
                if nx >= width || ny >= height {
                    break;
                }

                match grid[ny][nx] {
                    '#' => {
                        direction = rot_dir(&direction);
                        guard = (fx, fy);
                    },
                    _ => {
                        guard = (nx, ny);
                    }
                }
            }

        }
    }

    loops
}
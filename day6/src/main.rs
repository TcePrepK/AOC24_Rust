use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i32 = 41;
const ANSWER_TWO: i32 = 6;

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

/* ------------------- Solutions ------------------- */

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
fn second_part(input: &str) -> i32 {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut main_guard: (usize, usize) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut subgrid: Vec<char> = vec![];
        for (x, char) in line.chars().enumerate() {
            subgrid.push(char);

            if char == '^' {
                main_guard = (x, y);
            }
        }
        grid.push(subgrid);
    }
    let height: usize = grid.len();
    let width: usize = grid[0].len();

    // Run the guard once and store the path.
    let mut guard = main_guard.clone();

    let mut wall_poses: Vec<(usize, usize)> = vec![];
    let mut direction: DIRECTION = DIRECTION::UP;
    loop {
        let (fx, fy) = guard;
        let (dx, dy) = dir_to_vec(&direction);

        let (nx, ny) = ((fx as i32 + dx) as usize, ((fy as i32) + dy) as usize);
        if nx >= width || ny >= height {
            break;
        }

        match grid[ny][nx] {
            '#' => {
                direction = rot_dir(&direction);
                guard = (fx, fy);
            }
            '.' => {
                guard = (nx, ny);
                if !wall_poses.contains(&guard.clone()) {
                    wall_poses.push(guard.clone());
                }
            }
            _ => {
                guard = (nx, ny);
            }
        }
    }

    let mut loops: i32 = 0;
    for (wall_x, wall_y) in wall_poses.iter() {
        grid[*wall_y][*wall_x] = '#';
        guard = main_guard.clone();

        let mut visited: Vec<Vec<i8>> = vec![vec![0; width]; height];
        let mut direction: DIRECTION = DIRECTION::UP;

        loop {
            let (fx, fy) = guard;
            let (dx, dy) = dir_to_vec(&direction);

            if visited[fy][fx] > 2 {
                loops += 1;
                break;
            }

            visited[fy][fx] += 1;

            let (nx, ny) = ((fx as i32 + dx) as usize, ((fy as i32) + dy) as usize);
            if nx >= width || ny >= height {
                break;
            }

            match grid[ny][nx] {
                '#' => {
                    direction = rot_dir(&direction);
                    guard = (fx, fy);
                }
                _ => {
                    guard = (nx, ny);
                }
            }
        }

        grid[*wall_y][*wall_x] = '.';
    }

    loops
}

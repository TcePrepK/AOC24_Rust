use std::fs;
use glam::IVec2;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
];
const CORNERS: [IVec2; 4] = [
    IVec2::new(-1, -1),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
];

#[allow(unused_variables)]
fn main() {
    let example = read_file("src/example");
    let input = read_file("src/input");
    let used_string = input;

    println!("First Part: {:?}", first_part(&used_string));
    println!("Second Part: {:?}", second_part(&used_string));
}

fn first_part(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    // grid[y][x]
    let mut xmas_count: i32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let char = grid[y][x];
            if char != 'X' { continue; }

            for directions in DIRECTIONS.iter() {
                let mut possible: bool = true;
                for i in 0..3 {
                    let dir = directions[i];
                    let new_x = x as i32 + dir.x;
                    let new_y = y as i32 + dir.y;
                    if
                        new_y < 0 || new_y >= grid.len() as i32 ||
                        new_x < 0 || new_x >= grid[new_y as usize].len() as i32
                    {
                        possible = false;
                        break;
                    }

                    if grid[new_y as usize][new_x as usize] != XMAS[i + 1] {
                        possible = false;
                        break;
                    }
                }

                if possible {
                    xmas_count += 1;
                }
            }
        }
    }

    xmas_count
}

fn second_part(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let mut xmas_count: i32 = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            let char = grid[y][x];
            if char != 'A' { continue; }

            let mut corners: [char; 4] = ['.'; 4];
            for i in 0..4 {
                let dir = CORNERS[i];
                let new_x = x as i32 + dir.x;
                let new_y = y as i32 + dir.y;
                corners[i] = grid[new_y as usize][new_x as usize];
            }

            if
                (corners[0] == 'M' && corners[1] == 'S' || corners[1] == 'M' && corners[0] == 'S') &&
                (corners[2] == 'M' && corners[3] == 'S' || corners[3] == 'M' && corners[2] == 'S')
            {
                xmas_count += 1;
            }
        }
    }

    xmas_count
}
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i32 = 1930;
const ANSWER_TWO: i32 = 1206;

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
        println!("Part One: {:?}", first_part(&input));
    }
    if example_solutions[1] {
        println!("Part Two: {:?}", second_part(&input));
    }
}

fn main() {
    let example_solutions = test_examples();
    test_inputs(example_solutions);
}

/* ------------------- Helpers ------------------- */

#[derive(Debug, PartialEq)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn to_usize(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn get_side(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dir: &Direction,
) -> Option<(usize, usize, char)> {
    let w = grid[0].len();
    let h = grid.len();

    let mut x = x as i32;
    let mut y = y as i32;

    match dir {
        Direction::Up => y -= 1,
        Direction::Down => y += 1,
        Direction::Left => x -= 1,
        Direction::Right => x += 1,
    }

    if x < 0 || x >= w as i32 || y < 0 || y >= h as i32 {
        return None;
    }

    Some((x as usize, y as usize, grid[y as usize][x as usize]))
}

fn find_neighbors(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];

    for a in 0..=1 {
        for b in 0..=1 {
            let nx: i32 = x as i32 + (1 - 2 * a) * b;
            let ny: i32 = y as i32 + (1 - 2 * a) * (1 - b);

            if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                neighbors.push((nx as usize, ny as usize));
            }
        }
    }

    neighbors
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i32 {
    let grid = parse_input(input);
    let w = grid[0].len();
    let h = grid.len();

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut result: i32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited[y][x] {
                continue;
            }

            let main_char = grid[y][x];
            let mut sides: i32 = 0;
            let mut field: Vec<(usize, usize)> = vec![];

            let mut fields_to_check = vec![(x, y)];
            while !fields_to_check.is_empty() {
                let (nx, ny) = fields_to_check.pop().unwrap();
                if visited[ny][nx] {
                    continue;
                }

                field.push((nx, ny));
                visited[ny][nx] = true;
                for dir in DIRS.iter() {
                    if let Some((sx, sy, ch)) = get_side(&grid, nx, ny, dir) {
                        if ch != main_char {
                            sides += 1;
                            continue;
                        }
                        fields_to_check.push((sx, sy));
                    } else {
                        sides += 1;
                    }
                }
            }

            let area = field.len() as i32;
            result += area * sides;
        }
    }

    result
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i32 {
    let grid = parse_input(input);
    let w = grid[0].len();
    let h = grid.len();

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut result: i32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited[y][x] {
                continue;
            }

            let main_char = grid[y][x];
            let mut field: Vec<(usize, usize)> = vec![];
            let mut sides: Vec<(usize, usize, &Direction)> = vec![];

            let mut fields_to_check = vec![(x, y)];
            while !fields_to_check.is_empty() {
                let (nx, ny) = fields_to_check.pop().unwrap();
                if visited[ny][nx] {
                    continue;
                }

                field.push((nx, ny));
                visited[ny][nx] = true;
                for dir in DIRS.iter() {
                    if let Some((sx, sy, ch)) = get_side(&grid, nx, ny, dir) {
                        if ch != main_char {
                            sides.push((nx, ny, dir));
                            continue;
                        }
                        fields_to_check.push((sx, sy));
                    } else {
                        sides.push((nx, ny, dir));
                    }
                }
            }

            let mut filtered_sides: [Vec<(usize, usize, &Direction)>; 4] =
                [vec![], vec![], vec![], vec![]];
            for side in sides.iter() {
                let dir = side.2;
                filtered_sides[dir.to_usize()].push(*side);
            }

            let mut perimeter: i32 = 0;
            for check_side in filtered_sides.iter_mut() {
                check_side.sort_by(|a, b| (a.0 + a.1).cmp(&(b.0 + b.1)));

                let mut test_sides: Vec<(usize, usize, &Direction)> = vec![];
                for side in check_side.iter() {
                    test_sides.push(*side);
                    let exists = test_sides.iter().find(|test| {
                        (test.0 as i32 - side.0 as i32).abs()
                            + (test.1 as i32 - side.1 as i32).abs()
                            == 1
                    });

                    if exists.is_some() {
                        continue;
                    }
                    perimeter += 1;
                }
            }

            let area = field.len() as i32;
            result += area * perimeter;
        }

        // 821268 >>
    }

    result
}

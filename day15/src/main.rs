use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

const ANSWER_ONE: i32 = 10092;
const ANSWER_TWO: i32 = 9021;

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

impl DIRECTION {
    fn to_vec(&self) -> (i32, i32) {
        match self {
            DIRECTION::UP => (0, -1),
            DIRECTION::DOWN => (0, 1),
            DIRECTION::LEFT => (-1, 0),
            DIRECTION::RIGHT => (1, 0),
        }
    }
}

/// The input consists of a grid where @ is a robot, # is a wall and O is a box.
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<DIRECTION>) {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let grid = data[0]
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dirs = data[1]
        .chars()
        .map(|c| match c {
            '^' => Some(DIRECTION::UP),
            'v' => Some(DIRECTION::DOWN),
            '<' => Some(DIRECTION::LEFT),
            '>' => Some(DIRECTION::RIGHT),
            _ => None,
        })
        .filter_map(|x| x)
        .collect::<Vec<DIRECTION>>();

    (grid, dirs)
}

/// Finds and replaces the robot in the grid with a space
fn find_robot(grid: &mut Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '@' {
                grid[y][x] = '.';
                return (x as i32, y as i32);
            }
        }
    }

    (0, 0)
}

/// Finds and returns all objects
fn find_objects(grid: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut objects: Vec<(i32, i32)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' || grid[y][x] == '[' {
                objects.push((x as i32, y as i32));
            }
        }
    }

    objects
}

fn can_push_box(grid: &Vec<Vec<char>>, (box_x, box_y): (i32, i32), direction: &DIRECTION) -> bool {
    let mut dir_vec = direction.to_vec();
    if dir_vec.0 == 1 {
        dir_vec.0 = 2;
    }

    let next_box = (box_x + dir_vec.0, box_y + dir_vec.1);
    match direction {
        DIRECTION::UP | DIRECTION::DOWN => {
            let left_char = grid[next_box.1 as usize][next_box.0 as usize];
            let right_char = grid[next_box.1 as usize][next_box.0 as usize + 1];

            match (left_char, right_char) {
                ('.', '[') => can_push_box(grid, (next_box.0 + 1, next_box.1), direction),
                (']', '.') => can_push_box(grid, (next_box.0 - 1, next_box.1), direction),
                ('[', ']') => can_push_box(grid, (next_box.0, next_box.1), direction),
                (']', '[') => {
                    can_push_box(grid, (next_box.0 - 1, next_box.1), direction)
                        && can_push_box(grid, (next_box.0 + 1, next_box.1), direction)
                }
                ('#', _) => false,
                (_, '#') => false,
                _ => true,
            }
        }
        DIRECTION::LEFT | DIRECTION::RIGHT => {
            match grid[next_box.1 as usize][next_box.0 as usize] {
                '.' => true,
                '#' => false,
                _ => can_push_box(grid, next_box, direction),
            }
        }
    }
}

fn push_box(grid: &mut Vec<Vec<char>>, (box_x, box_y): (i32, i32), direction: &DIRECTION) -> bool {
    let mut dir_vec = direction.to_vec();
    if dir_vec.0 == 1 {
        dir_vec.0 = 2;
    }

    let pushed = can_push_box(grid, (box_x, box_y), direction);
    if !pushed {
        return false;
    }

    let mut next_box = (box_x + dir_vec.0, box_y + dir_vec.1);
    match direction {
        DIRECTION::UP | DIRECTION::DOWN => {
            let left_char = grid[next_box.1 as usize][next_box.0 as usize];
            let right_char = grid[next_box.1 as usize][next_box.0 as usize + 1];

            match (left_char, right_char) {
                ('.', '[') => push_box(grid, (next_box.0 + 1, next_box.1), direction),
                (']', '.') => push_box(grid, (next_box.0 - 1, next_box.1), direction),
                ('[', ']') => push_box(grid, (next_box.0, next_box.1), direction),
                (']', '[') => {
                    let _ = push_box(grid, (next_box.0 - 1, next_box.1), direction);
                    push_box(grid, (next_box.0 + 1, next_box.1), direction)
                }
                _ => true,
            }
        }
        DIRECTION::LEFT | DIRECTION::RIGHT => {
            match grid[next_box.1 as usize][next_box.0 as usize] {
                ']' => push_box(grid, (next_box.0 - 1, next_box.1), direction),
                '[' => push_box(grid, (next_box.0, next_box.1), direction),
                _ => true,
            }
        }
    };

    if dir_vec.0 == 2 {
        next_box.0 -= 1;
    }

    grid[box_y as usize][box_x as usize] = '.';
    grid[box_y as usize][box_x as usize + 1] = '.';

    grid[next_box.1 as usize][next_box.0 as usize] = '[';
    grid[next_box.1 as usize][next_box.0 as usize + 1] = ']';

    true
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i32 {
    let (mut grid, directions) = parse_input(input);
    let mut robot = find_robot(&mut grid);

    for dir in directions {
        // Try to move towards the direction

        let dir_vec = dir.to_vec();
        let next_dir = (robot.0 + dir_vec.0, robot.1 + dir_vec.1);

        match grid[next_dir.1 as usize][next_dir.0 as usize] {
            '.' => {
                // Empty space, can move
                robot = next_dir;
            }
            'O' => {
                // Hit a box, move the box as much as possible

                let mut found_empty = (-1, -1);
                let mut next_grid = next_dir;
                loop {
                    next_grid = (next_grid.0 + dir_vec.0, next_grid.1 + dir_vec.1);
                    let next_char = grid[next_grid.1 as usize][next_grid.0 as usize];

                    if next_char == '.' {
                        found_empty = next_grid;
                        break;
                    } else if next_char == '#' {
                        // Hit a wall, can't move the boxes
                        break;
                    }
                }

                if found_empty.0 > 0 {
                    // Move the main box there and guard to current position
                    grid[found_empty.1 as usize][found_empty.0 as usize] = 'O';
                    grid[next_dir.1 as usize][next_dir.0 as usize] = '.';
                    robot = next_dir;
                }
            }
            _ => (),
        }
    }

    let objects = find_objects(&grid);
    let mut result = 0;
    for object in objects {
        result += object.0 + object.1 * 100;
    }

    result
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i32 {
    let (mut grid, directions) = parse_input(input);
    grid = grid
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|c| {
                    if *c == 'O' {
                        vec!['[', ']']
                    } else if *c == '@' {
                        vec!['@', '.']
                    } else {
                        vec![*c, *c]
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut robot = find_robot(&mut grid);

    for dir in directions {
        // Try to move towards the direction
        grid[robot.1 as usize][robot.0 as usize] = '.';

        let dir_vec = dir.to_vec();
        let next_dir = (robot.0 + dir_vec.0, robot.1 + dir_vec.1);

        match grid[next_dir.1 as usize][next_dir.0 as usize] {
            '.' => {
                // Empty space, can move
                robot = next_dir;
            }
            '[' => {
                // Hit left side of a box, move the box as much as possible
                if push_box(&mut grid, next_dir, &dir) {
                    robot = next_dir;
                }
            }
            ']' => {
                // Hit right side of a box, move the box as much as possible
                if push_box(&mut grid, (next_dir.0 - 1, next_dir.1), &dir) {
                    robot = next_dir;
                }
            }
            _ => {
                // Hit a wall or unknown character, can't move
            }
        }

        grid[robot.1 as usize][robot.0 as usize] = '@';
    }

    let objects = find_objects(&grid);
    let mut result = 0;
    for object in objects {
        result += object.0 + object.1 * 100;
    }

    // 1574964 <<

    result
}

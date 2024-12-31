use utils::test_solutions;

fn main() {
    test_solutions(20, &first_part, None, &second_part, None);
}

/* ------------------- Helpers ------------------- */

/// Turns 2D grid input into 2D vector of chars.
fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

/// Finds the start and end of the given map.
fn find_start_end(grid: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                start = (x, y);
            }
            if grid[y][x] == 'E' {
                end = (x, y);
            }
        }
    }

    (start, end)
}

/// Calculates the weight grid, weight represents the number of tiles it takes to get to that tile.
fn get_weight_grid_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> (Vec<Vec<u32>>, Vec<((usize, usize), u32)>) {
    let size = grid.len();
    let mut weight_grid: Vec<Vec<u32>> = vec![vec![u32::MAX; size]; size];

    let mut path: Vec<((usize, usize), u32)> = vec![(start, 0)];
    loop {
        let (tile_pos, tile_weight) = path.last().unwrap();
        weight_grid[tile_pos.1][tile_pos.0] = *tile_weight;

        if *tile_pos == end {
            break;
        }

        for dir in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
            let (nx, ny) = (tile_pos.0 as i32 + dir.0, tile_pos.1 as i32 + dir.1);
            if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
                continue;
            }

            if grid[ny as usize][nx as usize] == '#' {
                continue;
            }

            let nw = weight_grid[ny as usize][nx as usize];
            if nw != u32::MAX {
                continue;
            }

            path.push(((nx as usize, ny as usize), tile_weight + 1));
            break;
        }
    }

    (weight_grid, path)
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u32 {
    let grid = get_grid(input);
    let (start, end) = find_start_end(&grid);
    let (weight_grid, path) = get_weight_grid_path(&grid, start, end);

    let mut total_skips = 0;
    for i in 0..(path.len() - 5) {
        let ((x, y), cw) = path[i];
        for offset in [(2, 0), (0, 2), (-2, 0), (0, -2)] {
            let (nx, ny) = (x as i32 + offset.0, y as i32 + offset.1);
            if nx < 0
                || ny < 0
                || nx >= weight_grid.len() as i32
                || ny >= weight_grid[0].len() as i32
            {
                continue;
            }

            let next_weight = weight_grid[ny as usize][nx as usize];
            if next_weight == u32::MAX {
                continue;
            }

            if next_weight > cw && next_weight - cw >= 100 + 2 {
                total_skips += 1;
            }
        }
    }

    total_skips
}

#[allow(unused_variables)]
fn second_part(input: &str) -> u32 {
    let grid = get_grid(input);
    let (start, end) = find_start_end(&grid);
    let (weight_grid, path) = get_weight_grid_path(&grid, start, end);

    let mut total_skips = 0;
    for i in 0..(path.len() - 5) {
        let ((x, y), cw) = path[i];
        for ox in -20_i32..21 {
            for oy in (ox.abs() - 20)..(21 - ox.abs()) {
                let dist = ox.abs() + oy.abs();
                if dist < 2 {
                    continue;
                }

                let (nx, ny) = (x as i32 + ox, y as i32 + oy);
                if nx < 0
                    || ny < 0
                    || nx >= weight_grid.len() as i32
                    || ny >= weight_grid[0].len() as i32
                {
                    continue;
                }

                let next_weight = weight_grid[ny as usize][nx as usize];
                if next_weight == u32::MAX {
                    continue;
                }

                if next_weight > cw && next_weight - cw >= (100 + dist) as u32 {
                    total_skips += 1;
                }
            }
        }
    }

    total_skips
}

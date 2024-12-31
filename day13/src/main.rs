use utils::test_solutions;

fn main() {
    test_solutions(13, &first_part, Some(480), &second_part, Some(875318608908));
}

/* ------------------- Helpers ------------------- */

fn parse_to_matrix(input: &str) -> Vec<[f32; 6]> {
    let mut matrices: Vec<[f32; 6]> = vec![];
    for data in input.split("\n\n") {
        let processed: Vec<f32> = data
            .lines()
            .map(|line| {
                line.split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|s| s.split_at(2).1)
                    .map(|s| s.parse::<f32>().unwrap())
                    .collect::<Vec<f32>>()
            })
            .flatten()
            .collect::<Vec<f32>>();
        matrices.push(processed.try_into().unwrap());
    }

    matrices
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> i64 {
    let matrices = parse_to_matrix(input);

    let mut result: i64 = 0;
    for matrix in matrices.iter() {
        let x = matrix[0];
        let y = matrix[2];
        let z = matrix[1];
        let w = matrix[3];
        let a = matrix[4];
        let b = matrix[5];

        let det = w * x - y * z;
        if det == 0.0 {
            continue;
        }

        let b_press = (b * x - a * z) / det;
        let a_press = (a - b_press * y) / x;

        if b_press.fract() > 0.01 || a_press.fract() > 0.01 {
            continue;
        }

        result += a_press as i64 * 3 + b_press as i64;
    }

    result
}

#[allow(unused_variables)]
fn second_part(input: &str) -> i64 {
    let matrices = parse_to_matrix(input);

    let mut result: i64 = 0;
    for matrix in matrices.iter() {
        let x: f64 = matrix[0] as f64;
        let y: f64 = matrix[2] as f64;
        let z: f64 = matrix[1] as f64;
        let w: f64 = matrix[3] as f64;
        let a: f64 = 10000000000000.0 + matrix[4] as f64;
        let b: f64 = 10000000000000.0 + matrix[5] as f64;

        let det = w * x - y * z;
        if det == 0.0 {
            continue;
        }

        let b_press = (b * x - a * z) / det;
        let a_press = (a - b_press * y) / x;

        if b_press.fract() > 0.01 || a_press.fract() > 0.01 {
            continue;
        }

        result += a_press as i64 * 3 + b_press as i64;
    }

    result
}

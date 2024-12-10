use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Could not read file")
}

#[allow(unused_variables)]
fn main() {
    let example = read_file("src/example");
    let input = read_file("src/input");
    let used_string = input;

    println!("First Part: {:?}", first_part(used_string.clone()));
    println!("Second Part: {:?}", second_part(used_string.clone()));
}

fn first_part(input: String) -> i32 {
    let reports: Vec<&str> = input.lines().collect();

    let mut safe_reports: i32 = 0;
    for report in reports {
        let levels: Vec<i32> = report.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut safe: bool = true;
        for i in 1..levels.len() - 1 {
            let prev_dif = levels[i] - levels[i - 1];
            let next_dif = levels[i + 1] - levels[i];

            if prev_dif * next_dif <= 0 || prev_dif.abs() > 3 || next_dif.abs() > 3 {
                safe = false;
                break;
            }
        }

        if safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn second_part(input: String) -> i32 {
    let reports: Vec<&str> = input.lines().collect();

    let mut safe_reports: i32 = 0;
    for report in reports {
        let levels: Vec<i32> = report.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut safe: bool = false;
        for i in 0..levels.len() {
            let mut clone_levels = levels.clone();
            clone_levels.remove(i);

            let mut sub_safe: bool = true;
            for i in 1..clone_levels.len() - 1 {
                let prev_dif = clone_levels[i] - clone_levels[i - 1];
                let next_dif = clone_levels[i + 1] - clone_levels[i];

                if prev_dif * next_dif <= 0 || prev_dif.abs() > 3 || next_dif.abs() > 3 {
                    sub_safe = false;
                    break;
                }
            }

            if sub_safe {
                safe = true;
                break;
            }
        }

        if safe { safe_reports += 1; }
    }

    safe_reports
}
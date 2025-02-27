use utils::test_solutions;

fn main() {
    test_solutions(5, &first_part, Some(143), &second_part, Some(123));
}

/* ------------------- Helpers ------------------- */

/// Parses the input and returns the rules and lines.
fn parse_input(input: &str) -> (Vec<[i32; 2]>, Vec<Vec<i32>>) {
    let data = input.split("\n\n").collect::<Vec<&str>>();

    let rules = data[0]
        .split("\n")
        .map(|line| {
            line.split("|")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|lines| [lines[0], lines[1]])
        .collect::<Vec<[i32; 2]>>();
    let lines = data[1]
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (rules, lines)
}

/* ------------------- Solutions ------------------- */

fn first_part(input: &str) -> i32 {
    let (rules, lines) = parse_input(input);

    // For each line, check every rule
    let mut result: i32 = 0;
    for line_i in 0..lines.len() {
        let line = &lines[line_i];

        // Create a [false, false] for each rule
        let mut fixed_rules: Vec<[i32; 2]> = vec![];
        let mut rules_check: Vec<[i32; 2]> = vec![];
        for [num1, num2] in rules.iter() {
            if line.contains(num1) && line.contains(num2) {
                fixed_rules.push([*num1, *num2]);
                rules_check.push([-1, -1]);
            }
        }

        // Loop through each number
        let mut correct_line: bool = true;
        for num_i in 0..line.len() {
            let num = line[num_i];

            // Check every rule
            for rule_i in 0..fixed_rules.len() {
                let [check1, check2] = fixed_rules[rule_i];
                let [mut state1, mut state2] = rules_check[rule_i];

                if num == check1 {
                    rules_check[rule_i] = [num_i as i32, state2];
                    state1 = num_i as i32;
                } else if num == check2 {
                    rules_check[rule_i] = [state1, num_i as i32];
                    state2 = num_i as i32;
                }

                if state1 < 0 && state2 >= 0 {
                    correct_line = false;
                    break;
                }
            }
        }

        if correct_line {
            result += line[line.len() / 2];
        }
    }

    result
}

fn second_part(input: &str) -> i32 {
    let (rules, mut lines) = parse_input(input);

    // For each line, check every rule
    let mut result: i32 = 0;
    for line_i in 0..lines.len() {
        let line = &mut lines[line_i];

        // Create a [false, false] for each rule
        let mut old_fixed_rules: Vec<[i32; 2]> = vec![];
        let mut old_rules_check: Vec<[i32; 2]> = vec![];
        for [num1, num2] in rules.iter() {
            if line.contains(num1) && line.contains(num2) {
                old_fixed_rules.push([*num1, *num2]);
                old_rules_check.push([-1, -1]);
            }
        }

        let mut edited_once: bool = false;
        let mut correct_line: bool = false;
        while !correct_line {
            let fixed_rules = old_fixed_rules.clone();
            let mut rules_check = old_rules_check.clone();

            let mut mistake_left: i32 = -1;
            let mut mistake_right: i32 = -1;
            correct_line = true;
            for (num_i, num) in line.iter().enumerate() {
                // Check every rule
                for (rule_i, [check1, check2]) in fixed_rules.iter().enumerate() {
                    let [mut state1, mut state2] = rules_check[rule_i];

                    if num == check1 {
                        rules_check[rule_i] = [num_i as i32, state2];
                        state1 = num_i as i32;
                    } else if num == check2 {
                        rules_check[rule_i] = [state1, num_i as i32];
                        state2 = num_i as i32;
                    }

                    if state1 < 0 && state2 >= 0 {
                        correct_line = false;

                        mistake_right = state2;
                        mistake_left = line.iter().position(|a| a == check1).unwrap() as i32;

                        break;
                    }
                }

                if !correct_line {
                    break;
                }
            }

            if !correct_line {
                edited_once = true;

                let temp = line[mistake_left as usize];
                line[mistake_left as usize] = line[mistake_right as usize];
                line[mistake_right as usize] = temp;
            }
        }

        if edited_once {
            result += line[line.len() / 2];
        }
    }

    result
}

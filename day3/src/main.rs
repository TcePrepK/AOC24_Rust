use std::fs;
use regex::Regex;

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
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let nums = regex.captures_iter(&input)
        .map(|cap| {
            vec![cap[1].parse::<i32>().unwrap(), cap[2].parse::<i32>().unwrap()]
        })
        .collect::<Vec<Vec<i32>>>();

    nums.iter().map(|nums| nums[0] * nums[1]).sum()
}

fn second_part(input: String) -> i32 {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut enabled_mules: Vec<String> = Vec::new();

    let mut construct = input.clone();
    let mut doing = true;
    let mut finished = false;
    while !finished {
        let mul_opt = mul_regex.find(&construct)
            .map(|mat| (mat.start(), mat.end(), mat.as_str().to_string()));
        let do_opt = if !doing {
            do_regex.find(&construct)
                .map(|mat| (mat.start(), mat.end()))
        } else {
            None
        };
        let dont_opt = dont_regex.find(&construct)
            .map(|mat| (mat.start(), mat.end()));

        if mul_opt.is_none() || (!doing && do_opt.is_none()) {
            finished = true;
            continue;
        }

        if let Some((_, end)) = do_opt {
            construct.replace_range(0..end, &*".".repeat(end));
            doing = true;
            continue;
        }

        let (mul_start, mul_end, mul_str) = mul_opt.unwrap();
        if let Some((dont_start, dont_end)) = dont_opt {
            if dont_start < mul_start {
                construct.replace_range(0..dont_end, &*".".repeat(dont_end));
                doing = false;
                continue;
            }
        }

        enabled_mules.push(mul_str);
        construct.replace_range(0..mul_end, &*".".repeat(mul_end));
    }

    let mut result = 0;
    for mul in enabled_mules {
        let capture = mul_regex.captures(&mul).unwrap();

        let left = capture[1].parse::<i32>().unwrap();
        let right = capture[2].parse::<i32>().unwrap();
        result += left * right;
    }

    result
}
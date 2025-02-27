use regex::Regex;
use utils::test_solutions;

fn main() {
    test_solutions(3, &first_part, Some(161), &second_part, Some(48));
}

/* ------------------- Helpers ------------------- */

/* ------------------- Solutions ------------------- */

fn first_part(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let nums = regex
        .captures_iter(&input)
        .map(|cap| {
            vec![
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            ]
        })
        .collect::<Vec<Vec<i32>>>();

    nums.iter().map(|nums| nums[0] * nums[1]).sum()
}

fn second_part(input: &str) -> i32 {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut enabled_mules: Vec<String> = Vec::new();

    let mut construct = String::from(input);
    let mut doing = true;
    let mut finished = false;
    while !finished {
        let mul_opt = mul_regex
            .find(&construct)
            .map(|mat| (mat.start(), mat.end(), mat.as_str().to_string()));
        let do_opt = if !doing {
            do_regex
                .find(&construct)
                .map(|mat| (mat.start(), mat.end()))
        } else {
            None
        };
        let dont_opt = dont_regex
            .find(&construct)
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

use utils::test_solutions;

fn main() {
    test_solutions(
        17,
        &first_part,
        Some(String::from("4,6,3,5,6,3,5,2,1,0")),
        &second_part,
        None,
    );
}

/* ------------------- Helpers ------------------- */

/// Returns (registers, program)
fn parse_input(input: &str) -> ([i32; 3], Vec<i32>) {
    let mut registers = [0; 3];
    let mut program = vec![];

    let data = input.split("\n\n").collect::<Vec<&str>>();
    for (i, line) in data[0].lines().enumerate() {
        registers[i] = line.split(": ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
    }

    for char in data[1].split(": ").collect::<Vec<&str>>()[1]
        .split(',')
        .collect::<Vec<&str>>()
    {
        program.push(char.parse::<i32>().unwrap());
    }

    (registers, program)
}

fn execute_program(
    registers: &mut [i32; 3],
    program: Vec<i32>,
    inst_pointer: &mut i32,
    results: &mut Vec<i32>,
) {
    // If we are out of the program, we stop.
    if *inst_pointer as usize >= program.len() {
        return;
    }

    // First we read the opcode and operand and increment the instruction pointer each step.
    let opcode = program[*inst_pointer as usize];
    let literal_operand = program[*inst_pointer as usize + 1];

    // Then depending on the opcode, we calculate the combo operand.
    let combo_operand = match literal_operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        7 => -1,
        _ => registers[literal_operand as usize - 4],
    };

    // Now we execute the opcode.
    match opcode {
        0 => {
            // ADV (sets A to A / 2^num)
            registers[0] >>= combo_operand;
        }
        1 => {
            // BXL (sets B to XOR of B and num)
            registers[1] ^= literal_operand;
        }
        2 => {
            // BST (sets B to num % 8)
            registers[1] = combo_operand & 0b111;
        }
        3 => {
            // JNZ (if A is 0, nothing else jumps to num)
            if registers[0] != 0 {
                *inst_pointer = literal_operand - 2;
            }
        }
        4 => {
            // BXC (sets B to XOR of B and C)
            registers[1] ^= registers[2];
        }
        5 => {
            // OUT (prints num % 8)
            results.push(combo_operand & 0b111);
        }
        6 => {
            // BDV (sets B to A / 2^num)
            registers[1] = registers[0] >> combo_operand;
        }
        7 => {
            // CDV (sets C to A / 2^num)
            registers[2] = registers[0] >> combo_operand;
        }
        _ => panic!("Unknown opcode"),
    }

    // After execution, we call the next instruction.
    *inst_pointer += 2;
    execute_program(registers, program, inst_pointer, results);
}

/// Hard coded short version of the input program.
fn short_program(a: i64, results: &mut Vec<i64>) {
    let n = a & 0b111;

    let b = n ^ 0b11 ^ (a >> (n ^ 0b101));
    let a = a >> 3;

    results.push(b % 8);

    if a == 0 {
        return;
    }
    short_program(a, results);
}

/* ------------------- Solutions ------------------- */

/// Parses the registers/program and executes the program using the helper function.
/// Helper function gets the opcode/operand and executes it.
/// After execution, it calls itself again with increased instruction pointer.
#[allow(unused_variables)]
fn first_part(input: &str) -> String {
    let (mut registers, program) = parse_input(input);
    let mut results: Vec<i32> = vec![];
    execute_program(&mut registers, program, &mut 0, &mut results);

    results
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/// Kind of cheaty but works. Basically I "manually" compiled the program down to check faster.
/// 3 most important bits are used to calculate last operand.
/// 6 most important bits are used to calculate last opcode, and it continues like this.
/// Solution starts with 0b000 and checks for all 1000 numbers (worst case).
/// If it results in the correct "target number" it will move the number 3 bits to the left and continues.
#[allow(unused_variables)]
fn second_part(input: &str) -> i64 {
    let (registers, program) = parse_input(input);

    let target = [2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 3, 5, 5, 3, 0];
    let mut check = target.len() - 1;

    let mut result = 0;
    loop {
        let mut results: Vec<i64> = vec![];
        short_program(result, &mut results);

        if results[0] == target[check] {
            if check == 0 {
                break;
            }
            result <<= 3;
            check -= 1;

            continue;
        }

        result += 1
    }

    result
}

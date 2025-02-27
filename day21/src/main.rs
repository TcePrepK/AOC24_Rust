use std::collections::HashMap;
use utils::test_solutions;

fn main() {
    test_solutions(
        21,
        &first_part,
        Some(126384),
        &second_part,
        Some(154115708116294),
    );
}

/* ------------------- Helpers ------------------- */

/// Parses and turns the given input into a vector of numpad inputs ands scores.
fn get_numpad_inputs(input: &str) -> Vec<(Vec<char>, u64)> {
    input
        .lines()
        .map(|line| {
            (
                line.chars().collect::<Vec<char>>(),
                line.split_at(line.len() - 1).0.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<(Vec<char>, u64)>>()
}

/// Returns the movements you have to make to get from one number to another.
fn get_numpad_movement(input_moves: &Vec<char>) -> Vec<char> {
    let mut moves: Vec<char> = vec![];
    let positions: HashMap<char, (i32, i32)> = HashMap::from([
        ('A', (2, 3)),
        ('0', (1, 3)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
    ]);

    let mut from = 'A';
    for i in 0..input_moves.len() {
        let to = input_moves[i];

        let from_pos = positions[&from];
        let to_pos = positions[&to];

        from = to;

        let dx = to_pos.0 - from_pos.0;
        let dy = to_pos.1 - from_pos.1;

        let mut next_moves = vec![];
        if dx < 0 {
            next_moves.extend(vec!['<'; dx.abs() as usize]);
        }
        if dy < 0 {
            next_moves.extend(vec!['^'; dy.abs() as usize]);
        }
        if dy > 0 {
            next_moves.extend(vec!['v'; dy as usize]);
        }
        if dx > 0 {
            next_moves.extend(vec!['>'; dx as usize]);
        }

        if to_pos.0 == 0 && from_pos.1 == 3 {
            next_moves.reverse();
        }
        if from_pos.0 == 0 && to_pos.1 == 3 {
            next_moves.reverse();
        }

        moves.extend(next_moves);
        moves.push('A');
    }
    moves
}

/// Returns the movements you have to make to get from one move character to another.
fn get_moves_needed(from: char, to: char) -> Vec<char> {
    let mut moves_needed = vec![];
    match (from, to) {
        ('A', '^') => moves_needed.extend(vec!['<']),
        ('A', '>') => moves_needed.extend(vec!['v']),
        ('A', 'v') => moves_needed.extend(vec!['<', 'v']),
        ('A', '<') => moves_needed.extend(vec!['v', '<', '<']),
        ('^', 'A') => moves_needed.extend(vec!['>']),
        ('^', '>') => moves_needed.extend(vec!['v', '>']),
        ('^', 'v') => moves_needed.extend(vec!['v']),
        ('^', '<') => moves_needed.extend(vec!['v', '<']),
        ('>', 'A') => moves_needed.extend(vec!['^']),
        ('>', '^') => moves_needed.extend(vec!['<', '^']),
        ('>', 'v') => moves_needed.extend(vec!['<']),
        ('>', '<') => moves_needed.extend(vec!['<', '<']),
        ('v', 'A') => moves_needed.extend(vec!['^', '>']),
        ('v', '^') => moves_needed.extend(vec!['^']),
        ('v', '>') => moves_needed.extend(vec!['>']),
        ('v', '<') => moves_needed.extend(vec!['<']),
        ('<', 'A') => moves_needed.extend(vec!['>', '>', '^']),
        ('<', '^') => moves_needed.extend(vec!['>', '^']),
        ('<', '>') => moves_needed.extend(vec!['>', '>']),
        ('<', 'v') => moves_needed.extend(vec!['>']),
        _ => (),
    }

    moves_needed.push('A');
    moves_needed
}

/// Calculate the cache for the given robot
fn calculate_cache(robot: usize) -> HashMap<(char, char), u64> {
    let all_characters = vec!['A', '^', '>', '<', 'v'];
    let mut next_cache = HashMap::new();

    if robot == 1 {
        for from in all_characters.iter() {
            for to in all_characters.iter() {
                next_cache.insert((*from, *to), get_moves_needed(*from, *to).len() as u64);
            }
        }
        return next_cache;
    }

    let prev_cache = calculate_cache(robot - 1);
    for from in all_characters.iter() {
        for to in all_characters.iter() {
            let moves_needed = get_moves_needed(*from, *to);
            let mut score = 0;
            let mut prev_from = 'A';
            for movement in moves_needed.iter() {
                let prev_score = prev_cache.get(&(prev_from, *movement)).unwrap();
                prev_from = *movement;
                score += prev_score;
            }

            next_cache.insert((*from, *to), score);
        }
    }

    next_cache
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u64 {
    let numpad_inputs = get_numpad_inputs(input);

    let cache = calculate_cache(2);
    let mut total_score = 0;
    for (numpad, score) in numpad_inputs {
        let numpad_moves = get_numpad_movement(&numpad);

        let mut from = 'A';
        let mut check_score = 0;
        for to in numpad_moves {
            let score = cache.get(&(from, to)).unwrap();
            from = to;
            check_score += score;
        }
        total_score += score * check_score;
    }

    total_score
}

#[allow(unused_variables)]
fn second_part(input: &str) -> u64 {
    let numpad_inputs = get_numpad_inputs(input);

    let cache = calculate_cache(25);
    let mut total_score = 0;
    for (numpad, score) in numpad_inputs {
        let numpad_moves = get_numpad_movement(&numpad);

        let mut from = 'A';
        let mut check_score = 0;
        for to in numpad_moves {
            let score = cache.get(&(from, to)).unwrap();
            from = to;
            check_score += score;
        }
        total_score += score * check_score;
    }

    total_score
}

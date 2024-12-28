use std::collections::{HashMap, HashSet};
use std::fs;
use std::string::ToString;

fn read_file(path: &str) -> String {
    if fs::exists(path).expect("Could not check file") {
        fs::read_to_string(path).expect("Could not read file")
    } else {
        String::new()
    }
}

const ANSWER_ONE: u32 = 7;
const ANSWER_TWO: &str = "co,de,ka,ta";

fn test_examples() -> (bool, bool) {
    let mut example_1 = read_file("src/example_1");
    let mut example_2 = read_file("src/example_2");
    if example_1.is_empty() && !example_2.is_empty() {
        panic!("Example 1 is empty, but example 2 is not");
    } else if !example_1.is_empty() && example_2.is_empty() {
        panic!("Example 2 is empty, but example 1 is not");
    } else if example_1.is_empty() && example_2.is_empty() {
        example_1 = read_file("src/example");
        example_2 = example_1.clone();
    }

    let results = (first_part(&example_1), second_part(&example_2));

    if results.0 != 0 && results.0 != ANSWER_ONE {
        println!("Part One Wrong");
    }

    if results.1 != ANSWER_TWO.to_string() {
        println!("Part Two Wrong");
    }

    (results.0 == ANSWER_ONE, results.1 == ANSWER_TWO)
}

fn test_inputs(example_solutions: (bool, bool)) {
    let input = read_file("src/input");

    if example_solutions.0 {
        let start_time = std::time::Instant::now();
        let result = first_part(&input);
        let total_time = start_time.elapsed();
        println!("Part 1 result: {}, took: {:?}", result, total_time);
    }
    if example_solutions.1 {
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

/// Parses and returns a list of channels where each channel is a connection between two nodes
fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split("-");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect::<Vec<(&str, &str)>>()
}

/// Turns the channel list into a graph where each node is a channel and each edge is a connection
fn get_graphs(channels: &Vec<(&str, &str)>) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<bool>>) {
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut matrix: Vec<Vec<bool>> = vec![vec![false; 675]; 675];

    // Run through each channel and add the nodes to the graph,
    // Then add the other one to the connected list
    for (node_a, node_b) in channels {
        let a: usize = node_a
            .chars()
            .enumerate()
            .map(|(i, c)| {
                (c.to_ascii_lowercase() as usize - ('a' as usize)) * 26_usize.pow((1 - i) as u32)
            })
            .sum();
        let b: usize = node_b
            .chars()
            .enumerate()
            .map(|(i, c)| {
                (c.to_ascii_lowercase() as usize - ('a' as usize)) * 26_usize.pow((1 - i) as u32)
            })
            .sum();

        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);

        matrix[a][b] = true;
        matrix[b][a] = true;
    }

    (graph, matrix)
}

/// Alternative BBMC Algorithm. Finds all 3-cliques in the graph
fn find_all_three_cliques(
    graph: &HashMap<usize, HashSet<usize>>,
    matrix: &Vec<Vec<bool>>,
    possible_nodes: &mut Vec<usize>,
    current_clique: &mut Vec<usize>,
    total_cliques: &mut u32,
) {
    // If there are no more nodes to add, check if the current clique is larger than the max clique
    if current_clique.len() == 3 {
        *total_cliques += 1;
        return;
    }

    // If the current clique is empty, it means we're at the beginning of recursion
    // We need to find all the nodes that start with 't' and then call this function on them instead
    if current_clique.is_empty() {
        // Find all the nodes that start with 't'
        let nodes_with_t = possible_nodes
            .iter()
            .filter(|&&n| n > 493 && n < 520)
            .cloned()
            .collect::<Vec<usize>>();

        // For each node that starts with 't', remove it from the possible nodes and call this function on it
        for node in nodes_with_t {
            possible_nodes.retain(|&n| n != node);
            let mut next_nodes: Vec<usize> = possible_nodes
                .iter()
                .filter(|&n| matrix[*n][node])
                .cloned()
                .collect();

            find_all_three_cliques(
                graph,
                matrix,
                &mut next_nodes,
                &mut vec![node],
                total_cliques,
            );
        }
        return;
    }

    while let Some(node) = possible_nodes.pop() {
        // Act like we're adding the node to the clique
        current_clique.push(node.clone());

        // let mut next_nodes: Vec<String> = possible_nodes.clone();
        // next_nodes.retain(|n| graph[n].contains(&node));
        let mut next_nodes: Vec<usize> = possible_nodes
            .iter()
            .filter(|&n| matrix[*n][node])
            .cloned()
            .collect();

        find_all_three_cliques(
            graph,
            matrix,
            &mut next_nodes,
            current_clique,
            total_cliques,
        );

        // Remove the previously added node
        current_clique.pop();
    }
}

/// BBMC Algorithm. Finds the largest clique in the graph
fn branch_and_bound_algorithm(
    graph: &HashMap<usize, HashSet<usize>>,
    matrix: &Vec<Vec<bool>>,
    possible_nodes: &mut Vec<usize>,
    current_clique: &mut Vec<usize>,
    current_max: &mut Vec<usize>,
) {
    // If there are no more nodes to add, check if the current clique is larger than the max clique
    if possible_nodes.is_empty() && current_clique.len() > current_max.len() {
        *current_max = current_clique.clone();
        return;
    }

    while let Some(node) = possible_nodes.pop() {
        // Just return if this physically cannot get larger than the current max
        if current_clique.len() + possible_nodes.len() + 1 <= current_max.len() {
            return;
        }

        // Act like we're adding the node to the clique
        current_clique.push(node.clone());

        // let mut next_nodes: Vec<String> = possible_nodes.clone();
        // next_nodes.retain(|n| graph[n].contains(&node));
        let mut next_nodes: Vec<usize> = possible_nodes
            .iter()
            .filter(|&n| matrix[*n][node])
            .cloned()
            .collect();

        branch_and_bound_algorithm(graph, matrix, &mut next_nodes, current_clique, current_max);

        // Remove the previously added node
        current_clique.pop();
    }
}

/* ------------------- Solutions ------------------- */

#[allow(unused_variables)]
fn first_part(input: &str) -> u32 {
    let channels = parse_input(input);
    let (graph, matrix) = get_graphs(&channels);

    // Find the maximum clique
    let mut total_cliques = 0;
    let mut current_clique = Vec::new();
    let mut possible_nodes: Vec<usize> = graph.keys().cloned().collect();

    find_all_three_cliques(
        &graph,
        &matrix,
        &mut possible_nodes,
        &mut current_clique,
        &mut total_cliques,
    );

    total_cliques
}

#[allow(unused_variables)]
fn second_part(input: &str) -> String {
    let channels = parse_input(input);
    let (graph, matrix) = get_graphs(&channels);

    // Find the maximum clique
    let mut max_clique = Vec::new();
    let mut current_clique = Vec::new();
    let mut possible_nodes: Vec<usize> = graph.keys().cloned().collect();

    // Sort nodes by degree (descending order)
    possible_nodes.sort_by_key(|node| -(graph[node].len() as isize));

    branch_and_bound_algorithm(
        &graph,
        &matrix,
        &mut possible_nodes,
        &mut current_clique,
        &mut max_clique,
    );

    // Sort and turn into string to get the final answer
    max_clique.sort();

    max_clique
        .iter()
        .map(|x| {
            format!(
                "{}{}",
                ((x / 26) as u8 + b'a') as char,
                ((x % 26) as u8 + b'a') as char
            )
        })
        .collect::<Vec<String>>()
        .join(",")
}

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
struct NetworkNode {
    left_connection: Option<String>,
    right_connection: Option<String>,
    value: String,
}

impl NetworkNode {
    fn new(
        left_connection: Option<String>,
        right_connection: Option<String>,
        value: String,
    ) -> Self {
        Self {
            left_connection,
            right_connection,
            value,
        }
    }
}

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();

    let instructions = &lines[0].chars().collect::<Vec<char>>();

    // Build the network
    let network: &mut HashMap<String, NetworkNode> = &mut HashMap::new();
    for i in 2..lines.len() {
        let line = &lines[i];
        let mut split_line = line.split("=");
        let cur_node = split_line.next().unwrap().trim();
        let mut raw_connections = split_line.next().unwrap().split(",");
        let left_connection = raw_connections
            .next()
            .unwrap()
            .trim()
            .trim_start_matches("(");
        let right_connection = raw_connections.next().unwrap().trim().trim_end_matches(")");
        // println!("cur node {}", cur_node);
        // println!("left {}", left_connection);
        // println!("right {}", right_connection);

        network.insert(
            cur_node.to_string(),
            NetworkNode::new(
                Some(left_connection.to_string()),
                Some(right_connection.to_string()),
                cur_node.to_string(),
            ),
        );
    }
    let mut steps = 1;
    let mut cur_nodes: Vec<&NetworkNode> = vec![];

    // Lazy addition of all nodes that end in 'A'
    for k in network.keys().into_iter() {
        if k.ends_with("A") {
            cur_nodes.push(network.get(k).unwrap().borrow_mut());
        }
    }

    println!("Amt of starter nodes {}", cur_nodes.len());

    // let mut cur_node = network.get("AAA").unwrap();
    let mut i = 0;

    let mut finished_trees: Vec<u64> = vec![];
    while cur_nodes.len() > 0 as usize {
        // println!("cur node {}", cur_node.value);
        let instruction = instructions[i];
        // println!("current instruction: {}", instruction);

        for i in 0..cur_nodes.len() {
            let node = cur_nodes[i];
            if instruction == 'L' {
                let n = node.left_connection.clone().unwrap();
                cur_nodes[i] = network.get(&n).unwrap();
            } else {
                let n = node.right_connection.clone().unwrap();
                cur_nodes[i] = network.get(&n).unwrap();
            }
        }

        let mut elems_to_remove: Vec<usize> = vec![];
        for (i, node) in &mut cur_nodes.iter_mut().enumerate() {
            if node.value.ends_with("Z") {
                finished_trees.push(steps);

                println!("steps: {}", steps);
                elems_to_remove.push(i);
            }
        }

        for x in elems_to_remove {
            cur_nodes.remove(x);
        }

        i += 1;
        if i >= instructions.len() {
            i = 0;
        }

        steps += 1;
    }

    println!("network size {}", network.len());
    println!("steps required {}", lcm(&finished_trees));
    Ok(())
}

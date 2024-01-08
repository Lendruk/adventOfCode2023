use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let mut network: &mut HashMap<String, NetworkNode> = &mut HashMap::new();
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

    let mut cur_node = network.get("AAA").unwrap();
    let mut i = 0;

    println!("----");
    loop {
        println!("cur node {}", cur_node.value);
        let instruction = instructions[i];
        println!("current instruction: {}", instruction);

        if instruction == 'L' {
            let n = cur_node.left_connection.clone().unwrap();
            cur_node = network.get(&n).unwrap();
        } else {
            let n = &cur_node.right_connection.clone().unwrap();
            cur_node = network.get(n).unwrap();
        }

        if cur_node.value == "ZZZ" {
            break;
        }

        i += 1;

        if i >= instructions.len() {
            i = 0;
        }

        steps += 1;
    }

    println!("network size {}", network.len());

    println!("steps required {}", steps);
    Ok(())
}

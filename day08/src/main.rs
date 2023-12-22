use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct NetworkNode<'a> {
    left_connection: Option<Box<&'a NetworkNode<'a>>>,
    right_connection: Option<Box<&'a NetworkNode<'a>>>,
}

impl<'a> NetworkNode<'a> {
    fn new(
        left_connection: Option<Box<&'a NetworkNode>>,
        right_connection: Option<Box<&'a NetworkNode>>,
    ) -> Self {
        Self {
            left_connection,
            right_connection,
        }
    }
}

fn get_node<'a>(network: HashMap<String, NetworkNode>, node: &str) -> Option<&'a NetworkNode<'a>> {
    return network.get(node);
}

fn add_node<'a>(
    mut network: HashMap<String, NetworkNode>,
    node: String,
    left_connection: Option<Box<&'a NetworkNode>>,
    right_connection: Option<Box<&'a NetworkNode>>,
) {
    // let left_connection: &NetworkNode<'a> =
    //     self.get_node(&left_connection).unwrap_or(&NetworkNode {
    //         left_connection: None,
    //         right_connection: None,
    //     });
    // let right_connection = self.get_node(&right_connection).unwrap_or(&NetworkNode {
    //     left_connection: None,
    //     right_connection: None,
    // });

    // TODO: Try to make this work with references
    // let new_node = NetworkNode::new(
    //     Some(Box::new(left_connection)),
    //     Some(Box::new(right_connection)),
    // );
    let new_node = NetworkNode::new(left_connection, right_connection);
    network.insert(node, new_node);
}

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = File::open("./src/example.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();

    let instructions = &lines[0];

    // Build the network
    let mut network: HashMap<String, NetworkNode> = HashMap::new();
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
        println!("cur node {}", cur_node);
        println!("left {}", left_connection);
        println!("right {}", right_connection);

        let n = get_node(network, cur_node);
        match n {
            Some(node) => {}
            None => {
                add_node(
                    network,
                    cur_node.to_string(),
                    Some(Box::new(&NetworkNode::new(None, None))),
                    Some(Box::new(&NetworkNode::new(None, None))),
                );
            }
        }
    }
    let steps = 0;

    println!("steps required {}", steps);
    Ok(())
}

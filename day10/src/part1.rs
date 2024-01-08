use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn index_from_direction(direction: &Direction) -> (isize, isize) {
    // y, x
    match direction {
        Direction::NORTH => (-1, 0),
        Direction::SOUTH => (1, 0),
        Direction::EAST => (0, 1),
        Direction::WEST => (0, -1),
    }
}

fn build_connection_map() -> HashMap<char, HashMap<Direction, Vec<char>>> {
    let mut connections_map: HashMap<char, HashMap<Direction, Vec<char>>> = HashMap::new();
    // |
    let mut north_south: HashMap<Direction, Vec<char>> = HashMap::new();
    north_south.insert(Direction::NORTH, ['7', 'F', '|'].to_vec());
    north_south.insert(Direction::SOUTH, ['L', 'J', '|'].to_vec());
    connections_map.insert('|', north_south);
    // -
    let mut east_west: HashMap<Direction, Vec<char>> = HashMap::new();
    east_west.insert(Direction::EAST, ['J', '7', '-'].to_vec());
    east_west.insert(Direction::WEST, ['L', 'F', '-'].to_vec());

    connections_map.insert('-', east_west);
    // L
    let mut bend_north_east: HashMap<Direction, Vec<char>> = HashMap::new();
    bend_north_east.insert(Direction::NORTH, ['|', 'F', '7'].to_vec());
    bend_north_east.insert(Direction::EAST, ['-', 'J', '7'].to_vec());

    connections_map.insert('L', bend_north_east);
    // J
    let mut bend_north_west: HashMap<Direction, Vec<char>> = HashMap::new();
    bend_north_west.insert(Direction::NORTH, ['|', 'F', '7'].to_vec());
    bend_north_west.insert(Direction::WEST, ['-', 'F', 'L'].to_vec());

    connections_map.insert('J', bend_north_west);
    // 7
    let mut bend_south_west: HashMap<Direction, Vec<char>> = HashMap::new();
    bend_south_west.insert(Direction::SOUTH, ['|', 'L', 'J'].to_vec());
    bend_south_west.insert(Direction::WEST, ['-', 'L', 'F'].to_vec());

    connections_map.insert('7', bend_south_west);
    // F
    let mut bend_south_east: HashMap<Direction, Vec<char>> = HashMap::new();
    bend_south_east.insert(Direction::SOUTH, ['|', 'L', 'J'].to_vec());
    bend_south_east.insert(Direction::EAST, ['-', 'J', '7'].to_vec());

    connections_map.insert('F', bend_south_east);

    return connections_map;
}

fn is_connection_valid(
    origin: char,
    destination: char,
    direction: Direction,
    connection_map: &HashMap<char, HashMap<Direction, Vec<char>>>,
) -> bool {
    let origin_connections = connection_map.get(&origin);
    match origin_connections {
        Some(connections) => {
            let direction_connections = connections.get(&direction);
            match direction_connections {
                Some(available_connections) => available_connections.contains(&destination),
                None => false,
            }
        }
        None => false,
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

    let grid_width = lines[0].len();
    let grid_height = lines.len();

    // Lookup table
    let connection_map = build_connection_map();
    let mut grid: Grid = vec![];
    let mut starting_position: (usize, usize) = (0, 0);

    // Build pipe grid
    for i in 0..lines.len() {
        let line = lines[i].chars().collect::<Vec<char>>();
        let mut new_row: Vec<char> = vec![];
        for j in 0..line.len() {
            let ch = line[j];

            if ch == 'S' {
                starting_position = (i, j);
            }

            new_row.push(ch);
        }
        grid.push(new_row);
    }

    // Discover type of S
    let mut type_of_s = '.';
    for ch in ['|', '-', 'J', 'L', 'F', '7'] {
        let mut valid_connections = 0;
        for direction in [
            Direction::NORTH,
            Direction::SOUTH,
            Direction::WEST,
            Direction::EAST,
        ] {
            let offset = index_from_direction(&direction);
            if is_connection_valid(
                ch,
                grid[starting_position
                    .0
                    .checked_add_signed(offset.0)
                    .unwrap_or(0)][starting_position
                    .1
                    .checked_add_signed(offset.1)
                    .unwrap_or(0)],
                direction,
                &connection_map,
            ) {
                valid_connections += 1;
            }
        }

        if valid_connections == 2 {
            type_of_s = ch;
            break;
        }
    }

    println!("type of S: {}", type_of_s);
    grid[starting_position.0][starting_position.1] = type_of_s;

    // Build loop
    let mut pipe_loop: Vec<(usize, usize)> = vec![];

    let mut stack: Vec<(usize, usize)> = vec![];
    stack.push(starting_position);

    let mut previous_pos: (usize, usize) = (0, 0);
    while stack.len() > 0 {
        let vertex = stack.pop().unwrap();
        let pipe: char = grid[vertex.0][vertex.1];

        println!("pipe {}", pipe);
        println!("pos x {} y {}", vertex.1, vertex.0);
        if vertex.0 == starting_position.0 && vertex.1 == starting_position.1 && pipe_loop.len() > 0
        {
            break;
        }
        pipe_loop.push(vertex);

        for direction in [
            Direction::NORTH,
            Direction::SOUTH,
            Direction::WEST,
            Direction::EAST,
        ] {
            let offset = index_from_direction(&direction);
            let destination_position = (
                vertex.0.checked_add_signed(offset.0).unwrap_or(0),
                vertex.1.checked_add_signed(offset.1).unwrap_or(0),
            );

            if destination_position.0 >= grid_height || destination_position.1 >= grid_width {
                continue;
            }

            if destination_position.0 == previous_pos.0 && destination_position.1 == previous_pos.1
            {
                continue;
            }

            if is_connection_valid(
                pipe,
                grid[destination_position.0][destination_position.1],
                direction,
                &connection_map,
            ) {
                stack.push(destination_position);
                break;
            }
        }
        previous_pos = vertex;
    }

    println!("Loop length: {}", pipe_loop.len());
    println!("Loop half length: {}", pipe_loop.len() / 2);
    Ok(())
}

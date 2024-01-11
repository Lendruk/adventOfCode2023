use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = File::open("./src/example.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();

    let mut total_arrangements = 0;
    for line in lines {
        let split_line = line.split(" ").collect::<Vec<&str>>();

        let spring_sequence = split_line[0].chars().collect::<Vec<char>>();
        let spring_groups: Vec<usize> = split_line[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut arragements_per_line = 0;
        let mut unknowns: Vec<(usize, usize)> = vec![];

        let mut sequence_start: Option<usize> = None;

        let mut taken_sequence_start: Option<usize> = None;
        for (i, c) in spring_sequence.iter().enumerate() {
            if *c == '?' {
                if sequence_start == None {
                    sequence_start = Some(i);
                }
            } else if sequence_start != None {
                unknowns.push((sequence_start.unwrap(), i - 1));
                sequence_start = None;
            }

            if *c == '#' {
                if taken_sequence_start == None && i - 1 > 0 {
                    if spring_sequence[i - 1] == '.' {
                        taken_sequence_start = Some(i - 1);
                    }
                }
            } else if *c == '.' {
                if taken_sequence_start != None && i - 1 > 0 {
                    spring_groups.swap_remove()
                }
            }
        }

        if sequence_start != None {
            unknowns.push((sequence_start.unwrap(), spring_sequence.len() - 1));
        }

        println!("Unknowns: {:?} ", unknowns);
        println!("Available groups: {:?} ", spring_groups);
    }

    println!("Total Arrangements {}", total_arrangements);

    Ok(())
}

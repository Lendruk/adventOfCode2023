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
        let mut spring_groups: Vec<usize> = split_line[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        // let total_springs = spring_groups.iter().fold(0, |acc, x| acc + x);

        let mut arragements_per_line = 0;
        let mut unknowns: Vec<(usize, usize)> = vec![];

        let mut sequence_start: Option<usize> = None;

        let mut taken_sequence_start: Option<usize> = None;

        for (i, c) in spring_sequence.iter().enumerate() {
            if *c == '?' {
                if sequence_start == None {
                    sequence_start = Some(i);
                } else if i + 1 == spring_sequence.len() {
                    unknowns.push((sequence_start.unwrap(), i + 1));
                }
            } else if sequence_start != None {
                unknowns.push((sequence_start.unwrap(), i - 1));
                sequence_start = None;
            }

            if *c == '#' {
                if taken_sequence_start == None {
                    match i.checked_sub(1) {
                        Some(_) => {
                            if spring_sequence[i - 1] == '.' {
                                taken_sequence_start = Some(i);
                            }
                        }
                        None => {
                            taken_sequence_start = Some(i);
                        }
                    }
                } else if i + 1 == spring_sequence.len() {
                    let len = i - taken_sequence_start.unwrap() + 1;
                    // println!("start {}", taken_sequence_start.unwrap());
                    // println!("i {}", i);
                    // println!("len {}", len);
                    let index = spring_groups.iter().position(|x| x == &len).unwrap();
                    spring_groups.swap_remove(index);
                }
            } else if *c == '.' {
                if taken_sequence_start != None && i - 1 > 0 {
                    let len = i - taken_sequence_start.unwrap();
                    let index = spring_groups.iter().position(|x| x == &len).unwrap();
                    spring_groups.swap_remove(index);
                    taken_sequence_start = None;
                }
            }
        }

        for unknown_range in &unknowns {
            let prev: Option<char> = if unknown_range.0 - 1 >= 0 {
                Some(spring_sequence[unknown_range.0 - 1])
            } else {
                None
            };

            let next: Option<char> = if unknown_range.1 + 1 < spring_sequence.len() {
                Some(spring_sequence[unknown_range.1 + 1])
            } else {
                None
            };

            let mut range_size = unknown_range.1 - unknown_range.0 + 1;

            match prev {
                Some(c) => {
                    if c == '#' {
                        range_size += 1;
                    }
                }
                None => {}
            }

            for group in &spring_groups {
                if group <= &range_size {
                    // match remaining.checked_sub(group + 1) {
                    //     Some(val) => {}
                    //     None => {}
                    // }
                }
            }
        }

        println!("Unknowns: {:?} ", unknowns);
        println!("Available groups: {:?} ", spring_groups);
    }

    println!("Total Arrangements {}", total_arrangements);

    Ok(())
}

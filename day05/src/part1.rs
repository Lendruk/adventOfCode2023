use std::fs::File;
use std::io::{BufRead, BufReader};

fn mapValue(destination_range_start: u64, source_range_start: u64, seed: u64) -> u64 {
    return (seed - source_range_start) + destination_range_start;
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

    let mut seeds: Vec<(u64, bool)> = lines[0]
        .split_at(7)
        .1
        .split(" ")
        .map(|val| (val.to_string().parse::<u64>().unwrap(), false))
        .collect::<Vec<(u64, bool)>>();

    for i in 2..lines.len() {
        let line = lines[i].clone();
        if line.len() > 0 && line.chars().next().unwrap().is_numeric() {
            let split_line = line
                .split(" ")
                .map(|val| val.to_string().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let destination_range_start = split_line[0];
            let source_range_start = split_line[1];
            let range = split_line[2];
            for i in 0..seeds.len() {
                let seed = seeds[i];
                if source_range_start <= seed.0 && source_range_start + range >= seed.0 && !seed.1 {
                    let mapped_seed = mapValue(destination_range_start, source_range_start, seed.0);
                    seeds[i] = (mapped_seed, true);
                }
            }
        } else {
            for seed in &mut seeds {
                seed.1 = false;
            }
        }
    }

    seeds.sort();

    println!("{}", seeds[0].0);
    Ok(())
}

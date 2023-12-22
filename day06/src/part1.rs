use std::fs::File;
use std::io::{BufRead, BufReader};

struct Race {
    // ms
    duration: u64,
    // mm
    record_distance: u64,
}

impl Race {
    fn new(duration: u64, record_distance: u64) -> Race {
        Race {
            duration,
            record_distance,
        }
    }

    fn get_ways_to_beat(&self) -> u64 {
        let mut ways_to_beat = 0;

        for n in 1..self.duration + 1 {
            let distance_travelled = n * (self.duration - n);

            if distance_travelled > self.record_distance {
                ways_to_beat += 1;
            }
        }

        ways_to_beat
    }
}

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = include_str!("input.txt");
    let split_file = file.split('\n').collect::<Vec<&str>>();

    let mut races: Vec<Race> = vec![];

    let first_line = split_file[0].chars().collect::<Vec<char>>();
    let second_line = split_file[1].chars().collect::<Vec<char>>();

    let mut time_str = String::from("");
    let mut distance_str = String::from("");

    let mut time = 0;
    let mut distance = 0;
    for n in 0..first_line.len() {
        // Time
        let first_line_char = first_line[n];
        // Distance
        let second_line_char = second_line[n];

        if first_line_char.is_numeric() {
            time_str.push(first_line_char);
        } else if time_str.len() > 0 {
            time = time_str.parse::<u64>().unwrap_or(0);
            time_str = String::from("");
        }

        if second_line_char.is_numeric() {
            distance_str.push(second_line_char);
        } else if distance_str.len() > 0 {
            distance = distance_str.parse::<u64>().unwrap_or(0);
            distance_str = String::from("");
        }

        if time > 0 && distance > 0 {
            races.push(Race::new(time, distance));
            time = 0;
            distance = 0;
        }
    }

    let mut total = 1;
    for race in races {
        total *= race.get_ways_to_beat();
    }

    println!("total {}", total);

    Ok(())
}

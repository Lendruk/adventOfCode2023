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

    let first_line = split_file[0].chars().collect::<Vec<char>>();
    let second_line = split_file[1].chars().collect::<Vec<char>>();

    let mut time_str = String::from("");
    let mut distance_str = String::from("");

    for n in 0..first_line.len() {
        // Time
        let first_line_char = first_line[n];
        // Distance
        let second_line_char = second_line[n];

        if first_line_char.is_numeric() {
            time_str.push(first_line_char);
        }

        if second_line_char.is_numeric() {
            distance_str.push(second_line_char);
        }
    }
    let race: Race = Race::new(
        time_str.parse::<u64>().unwrap_or(0),
        distance_str.parse::<u64>().unwrap_or(0),
    );

    println!("total {}", race.get_ways_to_beat());

    Ok(())
}

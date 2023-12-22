use std::fs::File;
use std::io::{BufRead, BufReader};

fn map_value(value: u64, source_range_start: u64, destination_range_start: u64) -> u64 {
    return (value + source_range_start)
        .checked_sub(destination_range_start)
        .unwrap_or(value);
}

struct AlmanacMap {
    entries: Vec<AlmanacMapEntry>,
}

impl AlmanacMap {
    fn new() -> AlmanacMap {
        AlmanacMap { entries: vec![] }
    }

    fn add_entry(&mut self, entry: AlmanacMapEntry) {
        self.entries.push(entry);
    }

    fn map(&self, value: u64) -> u64 {
        let mut mapped_value = value;
        for entry in &self.entries {
            if value <= entry.destination_range_start + entry.range
                && value >= entry.destination_range_start
            {
                mapped_value = entry.map(mapped_value);
                break;
            }
        }
        return mapped_value;
    }
}

struct AlmanacMapEntry {
    source_range_start: u64,
    destination_range_start: u64,
    range: u64,
}

impl AlmanacMapEntry {
    fn new(source_range_start: u64, destination_range_start: u64, range: u64) -> AlmanacMapEntry {
        AlmanacMapEntry {
            source_range_start,
            destination_range_start,
            range,
        }
    }

    fn map(&self, value: u64) -> u64 {
        return map_value(value, self.source_range_start, self.destination_range_start);
    }
}

fn main() -> std::io::Result<()> {
    use std::time::Instant;
    let now = Instant::now();

    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();

    let raw_seeds: Vec<u64> = lines[0]
        .split_at(7)
        .1
        .split(" ")
        .map(|val| val.to_string().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut seeds: Vec<(u64, u64)> = vec![];
    let mut maps: Vec<AlmanacMap> = vec![];

    for i in (0..raw_seeds.len()).step_by(2) {
        let raw_seed = raw_seeds[i];
        seeds.push((raw_seed, raw_seeds[i + 1]));
    }

    lines.reverse();

    println!("amt of seeds {}", seeds.len());
    // Build the maps
    maps.push(AlmanacMap::new());
    for i in 0..lines.len() - 2 {
        let line = lines[i].clone();

        if line.len() > 0 {
            if line.chars().next().unwrap().is_numeric() {
                let split_line = line
                    .split(" ")
                    .map(|val| val.to_string().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                let destination_range_start = split_line[0];
                let source_range_start = split_line[1];
                let range = split_line[2];

                let entry =
                    AlmanacMapEntry::new(source_range_start, destination_range_start, range);
                maps.last_mut().unwrap().add_entry(entry)
            }
        } else {
            maps.push(AlmanacMap::new());
        }
    }

    let mut location = 0;
    while location < u64::MAX {
        let mut found_value = false;
        let mut mapped_location = location;
        for map in &maps {
            mapped_location = map.map(mapped_location);
        }

        for seed in &seeds {
            let seed_value = seed.0;
            let seed_range = seed.1;

            if mapped_location >= seed_value && mapped_location <= seed_value + seed_range {
                found_value = true;
                break;
            }
        }

        if found_value {
            break;
        } else {
            location += 1;
        }
    }

    println!("{}", location);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}

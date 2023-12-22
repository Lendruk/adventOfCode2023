use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();
    let mut total = 0;

    for i in 0..lines.len() {
        let line = lines[i].clone();
        let split_line: Vec<&str> = line.split("|").collect();
        let rawCard: Vec<&str> = split_line[0].split(":").collect();
        let cardNumbers: &str = rawCard[1].trim();
        let numbers = split_line[1];

        let mut numbers_set: HashSet<&str> = HashSet::new();
        for num in numbers.split(" ") {
            if num.len() > 0 {
                numbers_set.insert(num);
            }
        }

        let mut hits = 0;
        let mut points = 0;
        for card_num in cardNumbers.split(" ") {
            if card_num.len() > 0 && numbers_set.contains(card_num) {
                hits += 1;
            }
        }

        println!("{}", cardNumbers);
        println!("{}", numbers);
        println!("hits {}", hits);

        if hits > 0 {
            points = 1;
            if hits > 1 {
                for _ in 1..hits {
                    points *= 2;
                }
            }
        }
        println!("points {}", points);

        total += points;
    }

    println!("Total {}", total);

    Ok(())
}

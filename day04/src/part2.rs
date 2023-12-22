use std::collections::{HashMap, HashSet};
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
    let mut extra_cards_map: HashMap<usize, usize> = HashMap::new();

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

        let extra_cards = *extra_cards_map.get(&i).unwrap_or(&(0 as usize));
        let mut hits = 0;
        for card_num in cardNumbers.split(" ") {
            if card_num.len() > 0 && numbers_set.contains(card_num) {
                hits += 1;
                let next_card_index = i + hits;
                if next_card_index < lines.len() {
                    if extra_cards_map.contains_key(&next_card_index) {
                        extra_cards_map.insert(
                            next_card_index,
                            extra_cards_map.get(&next_card_index).unwrap() + 1 + extra_cards,
                        );
                    } else {
                        extra_cards_map.insert(next_card_index, 1 + extra_cards);
                    }
                }
            }
        }
        total += 1 + extra_cards;

        println!("{}", cardNumbers);
        println!("{}", numbers);
    }

    println!("Total {}", total);

    Ok(())
}

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_char_strength(c: char) -> u64 {
    match c {
        '2' => 0x1,
        '3' => 0x2,
        '4' => 0x3,
        '5' => 0x4,
        '6' => 0x5,
        '7' => 0x6,
        '8' => 0x7,
        '9' => 0x8,
        'T' => 0x9,
        'J' => 0xA,
        'Q' => 0xB,
        'K' => 0xC,
        'A' => 0xD,
        _ => c as u64,
    }
}

fn calculate_hand_type(hand_values: &HashMap<u64, u64>) -> u64 {
    let unique_values_len = hand_values.len();

    match unique_values_len {
        5 => {
            println!("High card");
            return 1;
        }
        4 => {
            println!("One pair");
            return 2;
        }
        3 => {
            let mut highest_ocurrence = 0;

            for (_, occurrences) in hand_values {
                if *occurrences > highest_ocurrence {
                    highest_ocurrence = *occurrences;
                }
            }

            if highest_ocurrence == 3 {
                println!("Three of a kind");
                return 4;
            } else {
                println!("Two pair");
                return 3;
            }
        }
        2 => {
            let mut highest_ocurrence = 0;

            for (_, occurrences) in hand_values {
                if *occurrences > highest_ocurrence {
                    highest_ocurrence = *occurrences;
                }
            }

            if highest_ocurrence == 4 {
                println!("Four of a kind");
                return 6;
            } else {
                println!("Full house");
                return 5;
            }
        }
        1 => {
            println!("Five of a kind");
            return 7;
        }
        _ => return 0,
    }
}

// The strategy here is to calculate the global absolute strength for each hand. relative position in the array matters not
// Since every hand has an unique layout this is possible
fn calculate_hand_strength(hand: &str) -> u64 {
    // (strength, occurrences)
    let mut hand_values: HashMap<u64, u64> = HashMap::new();
    let mut hand_base_strength = String::from("");

    for c in hand.chars() {
        let char_strength = get_char_strength(c);

        let char_hex: String = format!("{:x}", char_strength);
        hand_base_strength += &char_hex;
        hand_values.insert(
            char_strength,
            *hand_values.get(&char_strength).unwrap_or(&0) + 1,
        );
    }

    let final_hand_strength = u64::from_str_radix(
        &*String::from(calculate_hand_type(&hand_values).to_string() + &hand_base_strength),
        16,
    )
    .unwrap();

    println!("card {}", hand);

    return final_hand_strength;
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

    // (earnings, strength)
    let mut hands: BTreeMap<u64, u64> = BTreeMap::new();

    for line in lines {
        let mut split_entry = line.split(" ");
        let hand = split_entry.next().unwrap();
        let earnings = split_entry.next().unwrap().parse::<u64>().unwrap_or(0);
        let strength = calculate_hand_strength(hand);
        hands.insert(strength, earnings);
    }

    let mut total_earnings = 0;

    let mut i = 1;
    for (strength, earnings) in &hands {
        println!("Strength: {}, Earnings: {}, I: {}", strength, earnings, i);
        total_earnings += earnings * i;
        i += 1;
    }

    println!("Total earnings: {}", total_earnings);

    Ok(())
}

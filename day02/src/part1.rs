use std::fs::File;
use std::io::{BufRead, BufReader};

fn setIsValid(set: &str) -> bool {
    let red_amt = 12;
    let green_amt = 13;
    let blue_amt = 14;

    let parts: Vec<&str> = set.split(" ").collect();

    // println!("test {}", parts[0]);
    let amt = parts[1].replace(" ", "").parse::<i32>().unwrap();
    let color = parts[2];

    if (color == "red") {
        return amt <= red_amt;
    }

    if (color == "green") {
        return amt <= green_amt;
    }

    if (color == "blue") {
        return amt <= blue_amt;
    }

    return false;
}

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    let mut games = vec![0];
    for line in reader.lines() {
        let input_str = line?;
        let split_line = input_str.split(":");
        let collection = split_line.collect::<Vec<&str>>();
        let game_id = collection[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let sets = collection[1];

        // println!("Game id {}", game_id);
        let mut valid_game = true;
        for set in sets.split(";") {
            // println!("{}", set);
            let splitSet: Vec<&str> = set.split(",").collect();

            for set in splitSet {
                if !setIsValid(set) {
                    valid_game = false;
                    break;
                }
            }
        }

        if valid_game {
            println!("Game id {} Possible", game_id);
            games.push(game_id);
        } else {
            println!("Game id {} Impossible", game_id);
        }
    }

    let mut total = 0;

    for num in games {
        total += num;
    }

    println!("{}", total);
    return Ok(());
}

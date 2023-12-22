use std::fs::File;
use std::io::{BufRead, BufReader};

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
        let mut highest_green = 0;
        let mut highest_blue = 0;
        let mut highest_red = 0;

        // println!("Game id {}", game_id);
        for set in sets.split(";") {
            // println!("{}", set);
            let splitSet: Vec<&str> = set.split(",").collect();

            for set in splitSet {
                let parts: Vec<&str> = set.split(" ").collect();
                let amt = parts[1].replace(" ", "").parse::<i32>().unwrap();
                let color = parts[2];

                if color == "red" {
                    if amt > highest_red {
                        highest_red = amt;
                    }
                } else if color == "green" {
                    if amt > highest_green {
                        highest_green = amt;
                    }
                } else if color == "blue" {
                    if amt > highest_blue {
                        highest_blue = amt;
                    }
                }
            }
        }

        println!("Game id {} Possible", game_id);
        games.push(highest_blue * highest_red * highest_green);
    }

    let mut total = 0;

    for num in games {
        total += num;
    }

    println!("{}", total);
    return Ok(());
}

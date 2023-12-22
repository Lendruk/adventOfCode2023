use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_string(str: &String) -> u32 {
    // let regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine$").unwrap();

    if (str.contains("one")) {
        return 1;
    } else if (str.contains("two")) {
        return 2;
    } else if (str.contains("three")) {
        return 3;
    } else if (str.contains("four")) {
        return 4;
    } else if (str.contains("five")) {
        return 5;
    } else if (str.contains("six")) {
        return 6;
    } else if (str.contains("seven")) {
        return 7;
    } else if (str.contains("eight")) {
        return 8;
    } else if (str.contains("nine")) {
        return 9;
    }
    return 0;
}

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    let mut numbers = vec![0];
    for line in reader.lines() {
        let mut firstDigit = 0;
        let mut lastDigit = 0;

        let mut potential_number = String::from("");
        let mut inputStr = line?;
        // First digit
        for char in inputStr.chars() {
            if (char.is_numeric()) {
                firstDigit = char.to_digit(10).unwrap();
                potential_number = String::from("");
                break;
            } else {
                potential_number.push_str(char.to_string().as_str());
                let num = check_string(&potential_number);

                if num != 0 {
                    firstDigit = num;
                    potential_number = String::from("");
                    break;
                }
            }
        }

        // Last digit
        for char in inputStr.chars().rev() {
            if (char.is_numeric()) {
                lastDigit = char.to_digit(10).unwrap();
                potential_number = String::from("");
                break;
            } else {
                potential_number.push_str(char.to_string().as_str());
                let num = check_string(&potential_number.chars().rev().collect::<String>());

                if num != 0 {
                    lastDigit = num;
                    potential_number = String::from("");
                    break;
                }
            }
        }

        if lastDigit == 0 {
            lastDigit = firstDigit;
        }

        println!("{}{}", firstDigit, lastDigit);
        numbers.push(format!("{}{}", firstDigit, lastDigit).parse().unwrap())
    }

    let mut total = 0;

    for num in numbers {
        total += num;
    }

    println!("{}", total);
    Ok(())
}

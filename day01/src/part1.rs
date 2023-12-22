use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    let mut numbers = vec![0];
    for line in reader.lines() {
        // println!("{}", line?);
        let mut firstDigit = 0;
        let mut lastDigit = 0;

        for char in line?.chars() {
            if (char.is_numeric()) {
                if firstDigit == 0 {
                    firstDigit = char.to_digit(10).unwrap()
                } else {
                    lastDigit = char.to_digit(10).unwrap()
                }
            }
        }

        if lastDigit == 0 {
            lastDigit = firstDigit;
        }

        // println!("{}{}", firstDigit, lastDigit);
        numbers.push(format!("{}{}", firstDigit, lastDigit).parse().unwrap())
    }

    let mut total = 0;

    for num in numbers {
        total += num;
    }

    println!("{}", total);
    Ok(())
}

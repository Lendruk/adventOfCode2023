use std::fs::File;
use std::io::{BufRead, BufReader};

fn isSpecialCharacter(c: char) -> bool {
    match c {
        '/' => true,
        '#' => true,
        '@' => true,
        '*' => true,
        '=' => true,
        '-' => true,
        '%' => true,
        '&' => true,
        '$' => true,
        '+' => true,
        _ => false,
    }
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

    let mut numbers = vec![0];
    for lineIndex in 0..lines.len() {
        let line = lines[lineIndex].clone();

        let mut num = String::from("");
        let mut handlingNumber = false;
        let mut startIndex = 0;
        let chars: Vec<char> = line.chars().collect();
        for charIndex in 0..chars.len() {
            let char = chars[charIndex];

            if char.is_numeric() {
                if handlingNumber == false {
                    startIndex = charIndex;
                }

                handlingNumber = true;
                num.push(char);

                // Check next in line
                if charIndex + 1 < chars.len() {
                    let next_in_line = chars[charIndex + 1];

                    // End of the number
                    if !next_in_line.is_numeric() {
                        // Check next in line
                        if isSpecialCharacter(next_in_line) {
                            numbers.push(num.parse::<i32>().unwrap());
                        } else if (startIndex as i32) - 1 >= 0
                            && isSpecialCharacter(chars[startIndex - 1])
                        {
                            // Check start of number
                            numbers.push(num.parse::<i32>().unwrap());
                        } else {
                            let mut northHasChar = false;
                            let iterationStart = if (startIndex) as i32 - 1 < 0 {
                                0
                            } else {
                                startIndex - 1
                            };

                            // Parse north
                            if (lineIndex as i32) - 1 >= 0 {
                                let northLine: Vec<char> =
                                    lines[lineIndex - 1].clone().chars().collect();

                                for n in iterationStart..charIndex + 2 {
                                    if isSpecialCharacter(northLine[n]) {
                                        numbers.push(num.parse::<i32>().unwrap());
                                        println!("{}", num.parse::<i32>().unwrap());

                                        northHasChar = true;
                                        break;
                                    }
                                }
                            }

                            // Parse south
                            if (!northHasChar && lineIndex + 1 < lines.len()) {
                                // Check south
                                let southLine: Vec<char> =
                                    lines[lineIndex + 1].clone().chars().collect();

                                for n in iterationStart..charIndex + 2 {
                                    if isSpecialCharacter(southLine[n]) {
                                        numbers.push(num.parse::<i32>().unwrap());
                                        println!("{}", num.parse::<i32>().unwrap());
                                        break;
                                    }
                                }
                            }
                        }
                        handlingNumber = false;
                        startIndex = 0;
                        num = String::from("");
                    }
                } else {
                    // End of line = end of number

                    // Check start
                    if (startIndex as i32) - 1 >= 0 && isSpecialCharacter(chars[startIndex - 1]) {
                        // Check start of number
                        numbers.push(num.parse::<i32>().unwrap());
                    } else {
                        let mut northHasChar = false;
                        let iterationStart = if (startIndex) as i32 - 1 < 0 {
                            0
                        } else {
                            startIndex - 1
                        };

                        // Parse north
                        if (lineIndex as i32) - 1 >= 0 {
                            let northLine: Vec<char> =
                                lines[lineIndex - 1].clone().chars().collect();

                            for n in iterationStart..charIndex {
                                if isSpecialCharacter(northLine[n]) {
                                    numbers.push(num.parse::<i32>().unwrap());
                                    println!("{}", num.parse::<i32>().unwrap());

                                    northHasChar = true;
                                    break;
                                }
                            }
                        }

                        // Parse south
                        if (!northHasChar && lineIndex + 1 < lines.len()) {
                            // Check south
                            let southLine: Vec<char> =
                                lines[lineIndex + 1].clone().chars().collect();
                            for n in iterationStart..charIndex {
                                if isSpecialCharacter(southLine[n]) {
                                    numbers.push(num.parse::<i32>().unwrap());
                                    println!("{}", num.parse::<i32>().unwrap());

                                    break;
                                }
                            }
                        }
                    }

                    handlingNumber = false;
                    startIndex = 0;
                    num = String::from("");
                }
            }
        }
    }

    let mut total = 0;
    for num in numbers {
        total += num;
    }

    println!("{}", total);
    Ok(())
}

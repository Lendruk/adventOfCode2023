use std::fs::File;
use std::io::{BufRead, BufReader};

fn isGear(c: char) -> bool {
    match c {
        '*' => true,
        _ => false,
    }
}
// probably refactor this into using the current position as the source of truth
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
        let chars: Vec<char> = line.chars().collect();
        for charIndex in 0..chars.len() {
            let char = chars[charIndex];

            if isGear(char) {
                let mut partCount = 0;
                let mut parts: Vec<(i32, i32)> = vec![];
                // Search for parts
                // X X X
                // X * X
                // X X X

                // Left
                let left_index = (charIndex as i32) - 1;
                if left_index >= 0 && chars[left_index as usize].is_numeric() {
                    partCount += 1;
                    parts.push((lineIndex as i32, left_index));
                }

                // Right
                let right_index = charIndex + 1;
                if right_index < chars.len() && chars[right_index].is_numeric() {
                    partCount += 1;
                    parts.push((lineIndex as i32, right_index as i32));
                }

                let iterationStart = if (charIndex) as i32 - 1 < 0 {
                    0
                } else {
                    charIndex - 1
                };

                let iterationEnd = if charIndex as i32 + 1 < chars.len() as i32 {
                    charIndex + 1
                } else {
                    charIndex
                };

                // North
                if (lineIndex as i32) - 1 >= 0 {
                    let northLine: Vec<char> = lines[lineIndex - 1].clone().chars().collect();
                    let mut handling_number = false;
                    for n in iterationStart..iterationEnd + 1 {
                        if northLine[n as usize].is_numeric() && !handling_number {
                            partCount += 1;
                            parts.push((lineIndex as i32 - 1, n as i32));
                            handling_number = true;
                        } else if !northLine[n as usize].is_numeric() {
                            handling_number = false;
                        }
                    }
                }

                // South
                if (lineIndex + 1 < lines.len()) {
                    // Check south
                    let southLine: Vec<char> = lines[lineIndex + 1].clone().chars().collect();
                    let mut handling_number = false;
                    for n in iterationStart..iterationEnd + 1 {
                        if southLine[n].is_numeric() && !handling_number {
                            partCount += 1;
                            parts.push((lineIndex as i32 + 1, n as i32));
                            handling_number = true;
                        } else if !southLine[n].is_numeric() {
                            handling_number = false;
                        }
                    }
                }

                if partCount == 2 {
                    println!("Gear!");
                    let mut multiplication = 0;

                    let mut firstNum = String::from("");
                    let mut secondNum = String::from("");
                    let firstNumIndex = parts[0];

                    // Search left and right of number
                    let mut lineIndex = firstNumIndex.0;
                    let mut numIndex = firstNumIndex.1;
                    let mut line: Vec<char> = lines[lineIndex as usize].clone().chars().collect();

                    // Left
                    for x in (numIndex - 2..numIndex).rev() {
                        if x >= 0 && x < line.len() as i32 {
                            let ch = line[x as usize];

                            if ch.is_numeric() {
                                firstNum.insert_str(0, ch.to_string().as_str());
                            } else {
                                break;
                            }
                        }
                    }

                    firstNum.push(line[numIndex as usize]);

                    // Right
                    for x in (numIndex + 1..numIndex + 3) {
                        if x >= 0 && x < line.len() as i32 {
                            let ch = line[x as usize];

                            if ch.is_numeric() {
                                firstNum.push(ch);
                            } else {
                                break;
                            }
                        }
                    }

                    let secondNumIndex = parts[1];
                    // Search left and right of number
                    let mut lineIndex = secondNumIndex.0;
                    let mut numIndex = secondNumIndex.1;
                    let mut line: Vec<char> = lines[lineIndex as usize].clone().chars().collect();

                    // Left
                    for x in (numIndex - 2..numIndex).rev() {
                        if x >= 0 && x < line.len() as i32 {
                            let ch = line[x as usize];

                            if ch.is_numeric() {
                                secondNum.insert_str(0, ch.to_string().as_str());
                            } else {
                                break;
                            }
                        }
                    }

                    secondNum.push(line[numIndex as usize]);

                    // Right
                    for x in (numIndex + 1..numIndex + 3) {
                        if x >= 0 && x < line.len() as i32 {
                            let ch = line[x as usize];

                            if ch.is_numeric() {
                                secondNum.push(ch);
                            } else {
                                break;
                            }
                        }
                    }

                    // println!("{} - {}", firstNumIndex.0, firstNumIndex.1);
                    // println!("{} - {}", secondNumIndex.0, secondNumIndex.1);

                    println!("{}", firstNum);
                    println!("{}", secondNum);
                    numbers
                        .push(firstNum.parse::<i32>().unwrap() * secondNum.parse::<i32>().unwrap());
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

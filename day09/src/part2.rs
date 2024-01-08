use std::char::from_digit;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn generate_environmental_report(history: Vec<i64>) -> i64 {
    let mut rows: Vec<Vec<i64>> = vec![];
    rows.push(history);

    let mut running_sum = 0;
    let mut cur_row = 0;

    running_sum = 1;
    while running_sum != 0 {
        let mut new_row: Vec<i64> = vec![];
        running_sum = 0;

        let mut t: String = String::from("");

        for i in 0..rows[cur_row].len() - 1 {
            let new_val = rows[cur_row][i + 1] - rows[cur_row][i];
            running_sum += new_val.abs();
            t.push_str(&String::from(format!("{} ", new_val.to_string())));
            new_row.push(new_val);
        }
        println!("{}", t);

        rows.push(new_row);
        cur_row += 1;
    }

    let mut total = 0;

    rows.reverse();
    for vec in rows {
        total = vec.first().unwrap() - total;
    }

    return total;
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

    let mut final_values: Vec<i64> = vec![];
    for line in lines {
        let mut history = line
            .split(" ")
            .map(|val| val.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        // history.sort();
        final_values.push(generate_environmental_report(history));
    }

    println!(
        "Sum: {}",
        final_values.into_iter().reduce(|a, b| a + b).unwrap()
    );
    Ok(())
}

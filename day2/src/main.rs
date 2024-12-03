use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn format_input(path: &str) -> io::Result<Vec<Vec<u16>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut reports = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<u16> = line
            .split_whitespace()
            .map(|x| x.parse::<u16>().unwrap())
            .collect();

        reports.push(numbers);
    }

    Ok(reports)
}

fn get_safe(numbers: Vec<Vec<u16>>) -> u16 {
    let mut number_of_safe: u16 = 0;
    for row in numbers {
        let mut is_safe: bool = true;
        let mut is_decreasing: bool = false;
        if row[0] > row[1] {
            is_decreasing = true;
        }
        for i in 0..row.len().saturating_sub(1) {
            let diff = if is_decreasing {
                row[i] as i16 - row[i + 1] as i16
            } else {
                row[i + 1] as i16 - row[i] as i16
            };
            if !(diff > 0 && diff < 4) {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            number_of_safe += 1;
        }
    }
    number_of_safe
}

fn main() {
    match format_input("input.txt") {
        Ok(reports) => {
            println!("Number of safe: {:?}", get_safe(reports));
        }
        Err(e) => println!("Error: {}", e),
    }
}

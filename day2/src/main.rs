use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn format_input(path: &str) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut reports = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        reports.push(numbers);
    }

    // Combine both columns into result
    Ok(reports)
}

fn main() {
    match format_input("test.txt") {
        Ok(reports) => {
            println!("Reports: {:?}", reports); // or use {:#?} for prettier formatting
        }
        Err(e) => println!("Error: {}", e),
    }
}

use std::fs;
use regex::Regex;

fn part_1(input: String) {
    let pattern = r"mul\((\d+),\s*(\d+)\)";
    let re = Regex::new(pattern).unwrap();
    let mut total: u32 = 0;
    
    // Find all matches
    for input in re.captures_iter(&input) {
        let first_num = &input[1];      // Gets first number
        let second_num = &input[2];     // Gets second number
    
        total += first_num.parse::<u32>().unwrap() * second_num.parse::<u32>().unwrap();
    }
    println!("Total: {}", total);
}

fn main() {
    match fs::read_to_string("input.txt") {
        Ok(input) => {
            part_1(input);
        },
        Err(e) => println!("Error reading file: {}", e),
    }
}
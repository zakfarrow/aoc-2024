use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn format_input(path: &str) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();

        // Add first number to col1, second to col2
        col1.push(numbers[0].parse::<u32>().unwrap());
        col2.push(numbers[1].parse::<u32>().unwrap());
        col1.sort();
        col2.sort();
    }

    // Combine both columns into result
    Ok(vec![col1, col2])
}

fn compute_total_distance(data: &Vec<Vec<u32>>) -> u32 {
    let mut total_distance: u32 = 0;
    for i in 0..data[0].len() {
        let difference: i32 = (data[0][i] as i32) - (data[1][i] as i32);
        total_distance += difference.abs() as u32;
    }
    total_distance
}

fn compute_similarity_score(data: &Vec<Vec<u32>>) -> u32 {
    let mut similarity_score: u32 = 0;

    for &i in data[0].iter() {
        let count = data[1].iter().filter(|&&x| x == i).count() as u32;
        similarity_score += i * count;
    }
    similarity_score
}

fn main() {
    match format_input("input.txt") {
        Ok(formatted_data) => {
            let difference = compute_total_distance(&formatted_data);
            println!("Total difference: {}", difference);
            let similarity_score = compute_similarity_score(&formatted_data);
            println!("Similarity score: {}", similarity_score);
        }
        Err(e) => println!("Error: {}", e),
    }
}

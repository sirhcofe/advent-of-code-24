use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    match File::open(path) {
        Ok(file) => {
            let _ = solve(file);
        }
        Err(e) => {
            eprintln!(
                "Error reading file: {}. Ensure you have file input.txt created!",
                e
            );
        }
    }
}

fn solve(file: File) -> Result<(), io::Error> {
    let mut total = 0;
    let mut occurance;
    let (mut array_one, mut array_two) = read_columns(file)?;
    array_one.sort();
    array_two.sort();
    for a in array_one.iter() {
        occurance = 0;
        for b in array_two.iter() {
            if b == a {
                occurance += 1;
            }
        }
        total += a * occurance;
    }
    println!("The total distance is {}!", total);
    Ok(())
}

fn read_columns(file: File) -> Result<(Vec<i32>, Vec<i32>), io::Error> {
    let reader = BufReader::new(file);

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let first_number = parts[0].parse::<i32>();
                let second_number = parts[1].parse::<i32>();
                if let (Ok(num1), Ok(num2)) = (first_number, second_number) {
                    col1.push(num1);
                    col2.push(num2);
                }
            } else {
                panic!("Arrays have different size!");
            }
        }
    }

    Ok((col1, col2))
}

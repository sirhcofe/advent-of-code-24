use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn main() {
    match File::open("input.txt") {
        Ok(file) => {
            let grid = parse(file).unwrap();
            let row_size = grid.len();
            let col_size = grid[0].len();
            solve(grid, row_size, col_size);
        }
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            process::exit(1);
        }
    }
}

fn parse(file: File) -> Result<Vec<Vec<char>>, io::Error> {
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let chars_vec: Vec<char> = line.chars().collect();
        grid.push(chars_vec);
    }

    return Ok(grid);
}

fn solve(grid: Vec<Vec<char>>, row_size: usize, col_size: usize) {
    let mut total = 0;
    println!("row: {} and col: {}", row_size, col_size);

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'A' {
                total += check_pattern(&grid, i, j, row_size, col_size);
            }
        }
    }

    println!("Total: {}", total);
}

fn check_pattern(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    row_size: usize,
    col_size: usize,
) -> i32 {
    if i == 0 || i + 1 == row_size || j == 0 || j + 1 == col_size {
        return 0;
    }

    let mut chars;
    let mut count = 0;

    chars = (grid[i - 1][j - 1], grid[i + 1][j + 1]);
    match chars {
        ('M', 'S') | ('S', 'M') => {
            count += 1;
        }
        _ => (),
    }

    chars = (grid[i - 1][j + 1], grid[i + 1][j - 1]);
    match chars {
        ('M', 'S') | ('S', 'M') => {
            count += 1;
        }
        _ => (),
    }

    if count == 2 {
        return 1;
    } else {
        return 0;
    }
}
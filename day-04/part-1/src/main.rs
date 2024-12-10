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
            if c == 'X' {
                total += check_horizontal(&grid, i, j, col_size);
                total += check_vertical(&grid, i, j, row_size);
                total += check_diagonal(&grid, i, j, row_size, col_size);
            }
        }
    }

    println!("Total: {}", total);
}

fn check_horizontal(grid: &Vec<Vec<char>>, i: usize, j: usize, col_size: usize) -> i32 {
    let mut count = 0;
    if j > 2 && grid[i][j - 1] == 'M' && grid[i][j - 2] == 'A' && grid[i][j - 3] == 'S' {
        count += 1;
    }
    if j + 3 < col_size && grid[i][j + 1] == 'M' && grid[i][j + 2] == 'A' && grid[i][j + 3] == 'S' {
        count += 1;
    }
    return count;
}

fn check_vertical(grid: &Vec<Vec<char>>, i: usize, j: usize, row_size: usize) -> i32 {
    let mut count = 0;
    if i > 2 && grid[i - 1][j] == 'M' && grid[i - 2][j] == 'A' && grid[i - 3][j] == 'S' {
        count += 1;
    }
    if i + 3 < row_size && grid[i + 1][j] == 'M' && grid[i + 2][j] == 'A' && grid[i + 3][j] == 'S' {
        count += 1;
    }
    return count;
}

fn check_diagonal(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    row_size: usize,
    col_size: usize,
) -> i32 {
    let mut count = 0;
    if i > 2 {
        if j > 2
            && grid[i - 1][j - 1] == 'M'
            && grid[i - 2][j - 2] == 'A'
            && grid[i - 3][j - 3] == 'S'
        {
            count += 1;
        }
        if j + 3 < col_size
            && grid[i - 1][j + 1] == 'M'
            && grid[i - 2][j + 2] == 'A'
            && grid[i - 3][j + 3] == 'S'
        {
            count += 1;
        }
    }
    if i + 3 < row_size {
        if j > 2
            && grid[i + 1][j - 1] == 'M'
            && grid[i + 2][j - 2] == 'A'
            && grid[i + 3][j - 3] == 'S'
        {
            count += 1;
        }
        if j + 3 < col_size
            && grid[i + 1][j + 1] == 'M'
            && grid[i + 2][j + 2] == 'A'
            && grid[i + 3][j + 3] == 'S'
        {
            count += 1;
        }
    }

    return count;
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn check_reverse_left_diagonal(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // Ensure there's enough space to check 3 characters diagonally
    if i + 2 >= matrix.len() || j < 2 {
        return false;
    }

    let word: String = (0..3).map(|k| matrix[i + k][j - k]).collect();
    println!("{}", word);

    let reverse_word: String = word.chars().rev().collect();

    word == "MAS" || reverse_word == "MAS"
}

fn check_diagonal(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // Ensure there's enough space to check 3 characters diagonally
    if i + 2 >= matrix.len() || j + 2 >= matrix[i].len() {
        return false;
    }

    let word: String = (0..3).map(|k| matrix[i + k][j + k]).collect();
    let reverse_word: String = word.chars().rev().collect();
    word == "MAS" || reverse_word == "MAS"
}

fn check_all_xmas_combinations(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut xmas_count = 0;

    if check_diagonal(matrix, i, j) {
        if check_reverse_left_diagonal(matrix, i, j + 2) {
            xmas_count += 1;
        }
    }

    xmas_count
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line_content = line?;
        let chars: Vec<char> = line_content.chars().collect();
        matrix.push(chars);
    }

    let mut total_xmas_count = 0;

    for (i, line) in matrix.iter().enumerate() {
        for (j, _char) in line.iter().enumerate() {
            total_xmas_count += check_all_xmas_combinations(&matrix, i, j);
        }
    }

    println!("Total XMAS combinations: {}", total_xmas_count);

    Ok(())
}

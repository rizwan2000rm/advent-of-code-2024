extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut uncorrupted_total = 0;

    for line in reader.lines() {
        let line_content = line?;
        for cap in regex.captures_iter(&line_content) {
            let num1 = cap.get(1).unwrap().as_str();
            let num2 = cap.get(2).unwrap().as_str();
            uncorrupted_total += (num1.parse::<i32>().unwrap()) * (num2.parse::<i32>().unwrap());
        }
    }

    Ok(())
}

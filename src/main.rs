extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let regex = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d+),(\d+)\)").unwrap();
    let file = File::open("./day3/input.txt")?;
    let reader = BufReader::new(file);

    let mut should_add = true;
    let mut uncorrupted_total = 0;

    for line in reader.lines() {
        let line_content = line?;
        for cap in regex.captures_iter(&line_content) {
            if cap.get(1).is_some() {
                should_add = true;
            }

            if cap.get(2).is_some() {
                should_add = false;
            }

            // if skip the numbers
            if !should_add {
                continue;
            }

            if let (Some(num1), Some(num2)) = (cap.get(3), cap.get(4)) {
                uncorrupted_total +=
                    num1.as_str().parse::<i32>().unwrap() * num2.as_str().parse::<i32>().unwrap();
            }
        }
    }

    println!("Total: {}", uncorrupted_total);
    Ok(())
}

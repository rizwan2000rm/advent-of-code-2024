use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn follow_rules(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> i32 {
    let mut updates_map = HashMap::new();

    for (idx, &num) in update.iter().enumerate() {
        updates_map.insert(num, idx);
    }

    for rule in rules {
        if updates_map.contains_key(&rule[0]) && updates_map.contains_key(&rule[1]) {
            let indices1 = updates_map.get(&rule[0]).unwrap();
            let indices2 = updates_map.get(&rule[1]).unwrap();

            if indices1 > indices2 {
                return 0;
            }
        }
    }

    // Return middle value safely
    update[update.len() / 2]
}

fn main() -> io::Result<()> {
    // Open the input file
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut rules: Vec<Vec<i32>> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    // Iterate through lines
    for (_line_num, line) in reader.lines().enumerate() {
        let line_content = line?;

        // Check if line contains '|'
        if line_content.contains('|') {
            // Split by '|', filter out empty strings, then parse
            let pipe_values: Vec<i32> = line_content
                .split('|')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect();

            if !pipe_values.is_empty() {
                rules.push(pipe_values);
            }
        }
        // Otherwise, assume comma-separated
        else {
            // Split by ',', filter out empty strings, then parse
            let comma_values: Vec<i32> = line_content
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse().unwrap_or(0))
                .collect();

            if !comma_values.is_empty() {
                updates.push(comma_values);
            }
        }
    }

    // Compute total of mid values
    let mut total_of_mid_values = 0;
    for update in &updates {
        total_of_mid_values += follow_rules(update, &rules);
    }

    println!("\nTotal of mid values: {}", total_of_mid_values);
    Ok(())
}

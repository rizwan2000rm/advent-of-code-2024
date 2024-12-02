use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut location_list1: Vec<i32> = Vec::new();
    let mut location_list2: Vec<i32> = Vec::new();

    // 1. Parse the input file into a list of locations
    for line in reader.lines() {
        let line_content = line?;
        let nums: Vec<i32> = line_content
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        location_list1.push(nums[0]);
        location_list2.push(nums[1]);
    }

    // 2. Find all the occurences of each number in location_list2
    let mut occurences: HashMap<i32, i32> = HashMap::new();

    for num in location_list2 {
        *occurences.entry(num).or_insert(0) += 1;
    }

    let mut total: i32 = 0;

    for num in location_list1 {
        if occurences.contains_key(&num) {
            total += num * occurences[&num];
            println!("num: {}, occurences: {}", num, occurences[&num]);
        }
    }

    println!("total {:?}", total);

    Ok(())
}

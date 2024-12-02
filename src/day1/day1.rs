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

  // 2. Sort the location lists
  location_list1.sort();
  location_list2.sort();

  // 3. Calculate the distance between the two locations i.e. location_list1[n] - location_list2[n]
  let mut distance: Vec<i32> = Vec::new();
  for i in 0..location_list1.len() {
    // absolute value of the difference between the two locations
    distance.push((location_list1[i] - location_list2[i]).abs());
  }

  // 4. Add up the distances
  let total_distance = distance.iter().sum::<i32>();
  println!("total_distance: {}", total_distance);
  
  Ok(())
}
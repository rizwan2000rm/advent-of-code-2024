use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn check_safe(report: Vec<i32>) -> bool {
    if report.len() < 2 {
        return false;
    }

    let is_increasing = report[0] < report[1];

    for window in report.windows(2) {
        let difference = (window[0] - window[1]).abs();

        // Any two adjacent levels differ by at least one and at most three.
        if difference < 1 || difference > 3 {
            return false;
        };

        // The levels are either all increasing or all decreasing.
        if (is_increasing && window[0] > window[1]) || (!is_increasing && window[0] < window[1]) {
            return false;
        }
    }

    true
}

fn brute_force_safe_check(report: Vec<i32>) -> bool {
    for (idx, _) in report.iter().enumerate() {
        let mut new_report = Vec::new();

        // Add elements before the current index
        if idx > 0 {
            new_report.extend_from_slice(&report[..idx]);
        }

        // Add elements after the current index
        if idx < report.len() - 1 {
            new_report.extend_from_slice(&report[idx + 1..]);
        }

        // Assuming check_safe is a function that checks if the new report is safe
        if check_safe(new_report) {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line_content = line?;
        let nums: Vec<i32> = line_content
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        reports.push(nums);
    }

    let mut safe_reports: i32 = 0;

    for report in reports {
        if check_safe(report.clone()) {
            safe_reports += 1;
        } else {
            let is_safe = brute_force_safe_check(report.clone());

            if is_safe {
                safe_reports += 1;
            }
        }
    }

    println!("safe_reports {}", safe_reports);

    Ok(())
}

use std::io::{self};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_from_file("day2/part1.txt")?;

    let mut safe_reports = 0;

    for report in data.lines() {
        // each report is a space-separated list of ints
        let split = report.split_whitespace();
        let mut vals = Vec::new();
        for val in split {
            match val.parse::<i32>() {
                Ok(v) => vals.push(v),
                Err(_) => continue,
            }
        }
        if is_safe_report(&vals) {
            safe_reports += 1;
        }
    }
    println!("Day 2 Part 1: {}", safe_reports);

    let mut safe_reports = 0;
    for report in data.lines() {
        // Same as the above, but a report is safe if we can remove any single level and still have a safe report
        let split = report.split_whitespace();
        let mut vals = Vec::new();
        for val in split {
            match val.parse::<i32>() {
                Ok(v) => vals.push(v),
                Err(_) => continue,
            }
        }

        if is_safe_report(&vals) {
            safe_reports += 1;
            continue;
        } else {
            // Try removing each value and check if the report is safe
            for i in 0..vals.len() {
                let mut new_vals = vals.clone();
                new_vals.remove(i);
                if is_safe_report(&new_vals) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    println!("Day 2 Part 2: {}", safe_reports);
    Ok(())
}

fn is_safe_report(vals: &[i32]) -> bool {
    // if vals is either stricly increasing or strictly decreasing by 1, 2, or 3 increment safe_reports
    // get the differences between each element
    let mut differences = Vec::new();
    for i in 0..vals.len() - 1 {
        differences.push(vals[i + 1] - vals[i]);
    }

    let min = differences.iter().min().unwrap();
    let max = differences.iter().max().unwrap();
    if *min < -3 || *max > 3 {
        return false;
    }
    if *min < 0 && *max > 0 {
        return false;
    }

    if differences.iter().all(|&x| x >= -3 && x <= 3 && x != 0) {
        return true;
    } else {
    }
    false
}

fn read_from_file(file_path: &str) -> Result<String, io::Error> {
    // Read the data from the specified file path
    let data = match std::fs::read_to_string(file_path) {
        Ok(d) => d,
        Err(e) => {
            println!("Error reading file");
            return Err(e);
        }
    };
    Ok(data)
}

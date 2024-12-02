use clap::{Arg, Command};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let matches = Command::new("Advent of Code CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Runs Advent of Code solutions")
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .value_name("DAY")
                .help("Specifies which day's solution to run"),
        )
        .get_matches();

    if let Some(day) = matches.get_one::<String>("day") {
        match day.parse::<u32>() {
            Ok(1) => run_day_1(),
            Ok(2) => run_day_2(),
            Ok(n) => println!("Solution for day {} is not implemented yet.", n),
            Err(_) => println!("Invalid day: {}", day),
        }
    } else {
        println!("Please specify a day using the --day option.");
    }
}

fn run_day_1() {
    let start = Instant::now();
    match day1_part1() {
        Ok(val) => {
            println!("Day 1 Part 1: {}", val);
        }
        Err(_) => {
            println!("Error in day1");
            return;
        }
    };
    println!("Day 1 Part 1 runtime: {:?}", start.elapsed());

    let start = Instant::now();
    match day1_part2() {
        Ok(val) => {
            println!("Day 1 Part 2: {}", val);
        }
        Err(_) => {
            println!("Error in day1part2");
            return;
        }
    };
    println!("Day 1 Part 2 runtime: {:?}", start.elapsed());
}

fn run_day_2() {
    let start = Instant::now();
    let data = match read_from_file("day2/part1.txt") {
        Ok(d) => d,
        Err(_) => {
            println!("Error reading file");
            return;
        }
    };

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
    println!("Day 2 Part 1 runtime: {:?}", start.elapsed());

    let start = Instant::now();
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
    println!("Day 2 Part 2 runtime: {:?}", start.elapsed());
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
        // println!(
        //     "Skipping report (above 3) (min: {:?}, max: {:?}): {:?} {:?}",
        //     min, max, vals, differences
        // );
        return false;
    }
    if *min < 0 && *max > 0 {
        // println!("Skipping report: {:?}", vals);
        return false;
    }

    if differences.iter().all(|&x| x >= -3 && x <= 3 && x != 0) {
        // println!("Safe report: {:?} {:?}", vals, differences);
        return true;
    } else {
        // println!("Skipping report: {:?} {:?}", vals, differences);
    }
    false
}

fn read_from_file(file_path: &str) -> Result<String, i32> {
    // Read the data from the specified file path
    let data = match std::fs::read_to_string(file_path) {
        Ok(d) => d,
        Err(_) => {
            println!("Error reading file");
            return Err(-1);
        }
    };
    Ok(data)
}

fn day1_part1() -> Result<i32, i32> {
    let data = match read_from_file("day1/part1.txt") {
        Ok(d) => d,
        Err(_) => {
            println!("Error reading file");
            return Err(-1);
        }
    };

    // data is a list of numbers, get each number and add the first to left vec and second to right vec
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in data.lines() {
        let mut split = line.split_whitespace();

        match split.next() {
            None => continue,
            Some(v) => match v.parse::<i32>() {
                Ok(l) => left.push(l),
                Err(_) => continue,
            },
        }
        match split.next() {
            None => continue,
            Some(v) => match v.parse::<i32>() {
                Ok(l) => right.push(l),
                Err(_) => continue,
            },
        }
    }

    // Sort each vector
    left.sort();
    right.sort();

    // Find the difference between each index of left and right and sum it
    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    Ok(sum)
}

fn day1_part2() -> Result<i32, i32> {
    let data = match read_from_file("day1/part1.txt") {
        Ok(d) => d,
        Err(_) => {
            println!("Error reading file");
            return Err(-1);
        }
    };

    // create a vector for each in the left column, and a map of counts for values in the right column
    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = std::collections::HashMap::new();

    for line in data.lines() {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse::<i32>().unwrap());

        let right_val = split.next().unwrap().parse::<i32>().unwrap();
        let count = right.entry(right_val).or_insert(0);
        *count += 1;
    }

    let mut sum: i32 = 0;

    // For each value in the left column, find the count of the value in the right column
    // and add left_val * right_count to the sum
    for left_val in left {
        let right_count = match right.get(&left_val) {
            Some(count) => count,
            None => &0,
        };
        sum += left_val * right_count;
    }
    Ok(sum)
}

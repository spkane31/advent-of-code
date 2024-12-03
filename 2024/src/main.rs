use clap::{Arg, Command};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

type Result<T> = std::result::Result<T, AOCError>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct AOCError {
    details: String,
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for AOCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred: {}", self.details)
    }
}

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
        let start = Instant::now();
        match day.parse::<u32>() {
            Ok(1) => match run_day_1() {
                Ok(_) => (),
                Err(e) => println!("Error in day 3: {:?}", e),
            },
            Ok(2) => match run_day_2() {
                Ok(_) => (),
                Err(e) => println!("Error in day 2: {:?}", e),
            },
            Ok(3) => match run_day_3() {
                Ok(_) => (),
                Err(e) => println!("Error in day 3: {:?}", e),
            },
            Ok(n) => println!("Solution for day {} is not implemented yet.", n),
            Err(_) => println!("Invalid day: {}", day),
        }
        println!("Total runtime: {:?}", start.elapsed());
    } else {
        println!("Please specify a day using the --day option.");
    }
}

fn run_day_1() -> Result<i32> {
    match day1_part1() {
        Ok(val) => {
            println!("Day 1 Part 1: {}", val);
        }
        Err(e) => {
            return Err(e);
        }
    };

    match day1_part2() {
        Ok(val) => {
            println!("Day 1 Part 2: {}", val);
        }
        Err(e) => {
            return Err(e);
        }
    };
    Ok(0)
}

fn run_day_2() -> Result<i32> {
    let data = match read_from_file("day2/part1.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
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
    Ok(0)
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

fn run_day_3() -> Result<i32> {
    let data = match read_from_file("day3/part1.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };

    // Sample input is xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    // What I want is only the mul(number,number) instructions, which can be done with regex
    // I want to get all of the valid instances of mul(number,number) and then multiply the
    // two numbers together and get the sum
    let re = match Regex::new(r"mul\((\d+),(\d+)\)") {
        Ok(r) => r,
        Err(e) => {
            return Err(AOCError {
                details: format!("Error creating regex: {:?}", e),
            });
        }
    };

    let mut sum = 0;
    for cap in re.captures_iter(&data) {
        let num1 = match cap[1].parse::<i32>() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let num2 = match cap[2].parse::<i32>() {
            Ok(n) => n,
            Err(_) => continue,
        };
        sum += num1 * num2;
    }

    println!("Day 3 Part 1: {:?}", sum);

    // For part two we also want to capture the instructions for 'do()' and "don't()", when
    // we hit "don't" we stop processing until we hit "do" again
    let re = match Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)") {
        Ok(r) => r,
        Err(e) => {
            return Err(AOCError {
                details: format!("Error creating regex: {:?}", e),
            });
        }
    };

    let mut sum = 0;
    let mut processing: bool = true;
    for cap in re.captures_iter(&data) {
        let matched = match cap.get(0) {
            Some(m) => m.as_str(),
            None => continue,
        };
        match matched {
            m if m.starts_with("mul") => {
                if processing {
                    let num1 = match cap[1].parse::<i32>() {
                        Ok(n) => n,
                        Err(_) => continue,
                    };
                    let num2 = match cap[2].parse::<i32>() {
                        Ok(n) => n,
                        Err(_) => continue,
                    };
                    sum += num1 * num2;
                }
            }
            "do()" => {
                processing = true;
            }
            "don't()" => {
                processing = false;
            }
            _ => {
                println!("Skipping: {:?}", cap);
            }
        };
    }

    println!("Day 3 Part 2: {:?}", sum);

    Ok(0)
}

fn read_from_file(file_path: &str) -> Result<String> {
    // Read the data from the specified file path
    let data = match std::fs::read_to_string(file_path) {
        Ok(d) => d,
        Err(_) => {
            println!("Error reading file");
            return Err(AOCError {
                details: "Error reading file".to_string(),
            });
        }
    };
    Ok(data)
}

fn day1_part1() -> Result<i32> {
    let data = match read_from_file("day1/part1.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
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

fn day1_part2() -> Result<i32> {
    let data = match read_from_file("day1/part1.txt") {
        Ok(d) => d,
        Err(_) => {
            return Err(AOCError {
                details: "Error reading file".to_string(),
            });
        }
    };

    // create a vector for each in the left column, and a map of counts for values in the right column
    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = std::collections::HashMap::new();

    for line in data.lines() {
        let mut split = line.split_whitespace();
        match split.next() {
            None => continue,
            Some(s) => match s.parse::<i32>() {
                Ok(l) => left.push(l),
                Err(_) => continue,
            },
        }

        let right_val = match split.next() {
            None => continue,
            Some(s) => match s.parse::<i32>() {
                Ok(l) => l,
                Err(_) => continue,
            },
        };
        // let right_val = split.next().unwrap().parse::<i32>().unwrap();
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

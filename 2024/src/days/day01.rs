use core::panic;
use std::io::{self};
use std::{collections::HashMap, fmt::Error};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    day1_part1()?;
    day1_part2()?;
    Ok(())
}

fn day1_part1() -> Result<(), Error> {
    let data = match read_from_file("day1/part1.txt") {
        Ok(d) => d,
        Err(e) => {
            panic!("Error reading file: {}", e);
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

    println!("Day 1 Part 1: {}", sum);
    Ok(())
}

fn day1_part2() -> Result<(), Error> {
    let data = match read_from_file("day1/part1.txt") {
        Ok(d) => d,
        Err(_) => {
            return Err(Error);
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

    println!("Day 1 Part 2: {}", sum);
    Ok(())
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

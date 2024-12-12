use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_from_file("day11/input.txt")?;

    let mut stones = HashMap::new();

    for d in data {
        *stones.entry(d).or_default() += 1;
    }

    for i in 0..75 {
        if i == 25 {
            println!("Day 11 Part 1: {}", stones.values().sum::<u64>());
        }
        stones = blink(stones);
    }

    println!("Day 11 Part 2: {}", stones.values().sum::<u64>());

    Ok(())
}

fn read_from_file(filename: &str) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Input data is a single list with a space separated list of numbers
    // 773 79858 0 71 213357 2937 1 3998391
    // Want to convert into a Vec of u64s

    let mut data: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        for num in line.split_whitespace() {
            data.push(num.parse()?);
        }
    }

    Ok(data)
}

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::with_capacity(stones.len());
    for (k, v) in stones {
        let num = format!("{}", k);
        match k {
            0 => *new_stones.entry(1).or_default() += v,
            _ => {
                if num.len() % 2 > 0 {
                    *new_stones.entry(2024 * k).or_default() += v;
                } else {
                    let l: u64 = num[..num.len() / 2].parse().unwrap();
                    let r: u64 = num[num.len() / 2..].parse().unwrap();
                    *new_stones.entry(l).or_default() += v;
                    *new_stones.entry(r).or_default() += v;
                }
            }
        };
    }
    new_stones
}

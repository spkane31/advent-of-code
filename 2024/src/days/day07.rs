use std::{fs, io};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = match read_from_file("day7/input.txt") {
        Ok(d) => d,
        Err(e) => return Err(e.into()),
    };

    let mut sum: u64 = 0;
    let mut sum2: u64 = 0;
    for line in data.lines() {
        // Split on the ":"
        let (total, vals) = line.split_once(":").unwrap();

        // total is an i32, vals is a space separated list of i32s
        let total = match total.parse::<u64>() {
            Ok(t) => t,
            Err(e) => return Err(e.into()),
        };

        let vals = vals.split_whitespace();
        let mut all_vals = Vec::new();
        for val in vals {
            match val.parse::<u64>() {
                Ok(v) => all_vals.push(v),
                Err(e) => return Err(e.into()),
            }
        }

        if match_operands_1(total, &all_vals.as_slice()) {
            sum += total;
        } else if match_operands(total, &all_vals.as_slice()) {
            sum2 += total;
        }
    }
    println!("Day 7 Part 1: {:?}", sum);
    println!("Day 7 Part 2: {:?}", sum + sum2);

    Ok(())
}

fn read_from_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

fn match_operands(val: u64, operands: &[u64]) -> bool {
    match operands {
        [] => panic!("No operands"),
        [last] => *last == val,
        [rest @ .., last] => {
            let mask = 10_u64.pow(last.ilog10() as u32 + 1);
            (val % last == 0 && match_operands(val / last, rest))
                || (val >= *last && match_operands(val - last, rest))
                || (val % mask == *last && match_operands(val / mask, rest))
        }
    }
}

fn match_operands_1(val: u64, operands: &[u64]) -> bool {
    match operands {
        [] => panic!("No operands"),
        [last] => *last == val,
        [rest @ .., last] => {
            (val % last == 0 && match_operands(val / last, rest))
                || (val >= *last && match_operands(val - last, rest))
        }
    }
}

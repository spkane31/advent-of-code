use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("day22/input.txt")?;
    let numbers = parse_numbers(&input);

    // let sum: i64 = numbers.iter().map(|&x| get_next(x, 2000)).sum();

    println!("Day 22 Part 1: {}", part1(numbers.clone()));

    println!("Day 22 Part 2: {}", part2(numbers.clone()));

    Ok(())
}

fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect()
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

fn part1(secrets: Vec<i64>) -> i64 {
    secrets
        .iter()
        .map(|&secret| {
            let mut result = secret;

            for _ in 0..2000 {
                result = encode(result);
            }

            result as i64
        })
        .sum::<i64>()
}

fn part2(secrets: Vec<i64>) -> i64 {
    // store state in vectors instead of hashmaps/hashsets
    let mut costs = vec![0; 0xFFFFF];
    let mut seen = vec![0; 0xFFFFF];

    for secret in secrets {
        let mut result = secret;
        let mut previous_cost = result % 10;
        let mut deltas = 0;

        for i in 0..2000 {
            result = encode(result);
            let cost = result % 10;

            // offset cost delta by +10 and represent as 5 bit unsigned int (max == 19 == 0b10101)
            // store sliding window of 4 deltas as a 20 bit unsigned int (max == 0xFFFFF)
            deltas = ((deltas << 5) & 0xFFFFF) + 10 + cost - previous_cost;

            // start checking prices once deltas window is populated
            // only counting the first occurance of each unique delta sequence
            if seen[deltas as usize] != secret && i > 3 {
                seen[deltas as usize] = secret;
                costs[deltas as usize] += cost;
            }

            previous_cost = cost;
        }
    }

    costs.iter().max().cloned().unwrap()
}

fn encode(n: i64) -> i64 {
    // this function is a simplification of operations:
    //
    // n ^= n * 64    --> operand is base 2, can be done by bit shifting left by log2(64) == 6
    // n %= 16777216  --> 16777216 == 0x1000000, can be done by bit masking 0xFFFFFF
    // n ^= n / 32    --> operand is base 2, can be done by bit shifting right by log2(32) == 5
    // n %= 16777216  --> number of bits will never increase after division, modulo not needed
    // n ^= n * 2048  --> operand is base 2, can be done by bit shifting left by log2(2048) == 11
    // n %= 16777216

    let a = (n ^ (n << 6)) & 0xFFFFFF;
    let b = a ^ (a >> 5);
    (b ^ (b << 11)) & 0xFFFFFF
}

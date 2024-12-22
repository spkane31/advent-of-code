use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("day22/sample2.txt")?;
    let numbers = parse_numbers(&input);

    let sum: i64 = numbers.iter().map(|&x| get_next(x, 2000)).sum();

    println!("Day 22 Part 1: {}", sum);

    let mut total: i64 = 0;
    for num in numbers {
        let prices: Vec<i64> = get_prices(num, 2000);

        let ones: Vec<i64> = prices.iter().map(|w| w % 10).collect();

        let ones_diff: Vec<i64> = prices.windows(2).map(|w| ones_diff(w[0], w[1])).collect();
        match find_subvector(ones_diff, vec![-2, 1, -1, 3]) {
            Some(i) => {
                println!("num, i, ones[i]: {}, {}, {}", num, i, ones[i]);
                total += ones[i];
            }
            None => (),
        }
    }

    println!("Day 22 Part 2: {}", total);

    Ok(())
}

fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect()
}

fn get_prices(num: i64, count: i64) -> Vec<i64> {
    (0..count).map(|c| get_next(num, c)).collect()
}

fn find_subvector(haystack: Vec<i64>, needle: Vec<i64>) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle.as_slice())
}

fn ones_diff(a: i64, b: i64) -> i64 {
    (b % 10) - (a % 10)
}

fn get_next(num: i64, count: i64) -> i64 {
    let mut next = num;
    for _ in 0..count {
        next = next_number(next);
    }
    next
}

fn next_number(num: i64) -> i64 {
    let a = prune(mix(num, num * 64));
    let b = prune(mix(a, a / 32));
    prune(mix(b, b * 2048))
}

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

use std::collections::HashSet;
use std::fs;
use std::io;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (p, designs) = match read_and_split_file("day19/sample.txt") {
        Ok((a, b)) => (a, b),
        Err(e) => return Err(e.into()),
    };

    let mut patterns: HashSet<String> = parse_comma_separated_list(&p);
    let original_patterns = patterns.clone();

    let mut count: u32 = 0;
    let mut total: u32 = 0;
    for design in designs.lines() {
        if can_make(design, &mut patterns) {
            count += 1;
            total += total_combos(design, &original_patterns);
        }
        break;
    }

    println!("Day 19 Part 1: {}", count);

    println!("Day 19 Part 2: {}", total);
    Ok(())
}

fn parse_comma_separated_list(input: &str) -> HashSet<String> {
    input.split(',').map(|s| s.trim().to_string()).collect()
}

fn can_make(design: &str, patterns: &mut HashSet<String>) -> bool {
    if patterns.contains(design) {
        return true;
    }

    for i in 1..design.len() {
        let (left, right) = design.split_at(i);
        if patterns.contains(left) && can_make(right, patterns) {
            patterns.insert(design.to_string());
            return true;
        }
    }

    false
}

fn total_combos(design: &str, patterns: &HashSet<String>) -> u32 {
    println!("total_combos: {} {:?}", design, patterns);
    if design.is_empty() {
        return 1;
    }

    let mut total: u32 = 0;
    for i in 1..design.len() {
        let (left, right) = design.split_at(i);
        println!("left: {} right: {}", left, right);
        if patterns.contains(left) {
            total += total_combos(right, patterns);
        }
        println!("total: {}", total);
    }

    total
}

fn read_and_split_file(file_path: &str) -> io::Result<(String, String)> {
    let content = fs::read_to_string(file_path)?;
    if let Some((part1, part2)) = content.split_once("\n\n") {
        Ok((part1.to_string(), part2.to_string()))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Delimiter not found",
        ))
    }
}

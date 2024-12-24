use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("day23/sample.txt")?;

    let pairs: Vec<(&str, &str)> = input
        .split("\n")
        .map(|line| {
            let mut parts = line.split("-");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            (left, right)
        })
        .collect();

    let mut g: HashMap<String, HashSet<String>> = HashMap::new();

    for (left, right) in pairs.iter() {
        let left_set = g.entry(left.to_string()).or_insert(HashSet::new());
        left_set.insert(right.to_string());
        let right_set = g.entry(right.to_string()).or_insert(HashSet::new());
        right_set.insert(left.to_string());
    }

    println!("graph: {:?}", g);

    let mut all_cycles: HashSet<Vec<String>> = HashSet::new();
    for (k, v) in g.iter() {
        println!("{}: {:?}", k, v);
        let mut visited = HashSet::new();

        let cycles = find_cycles_of_size_n(&g, 3, k, &mut visited);

        for cycle in cycles.iter() {
            if !cycle_contains(cycle, "t") {
                continue;
            }

            // sort the cycle before adding to all list
            let mut sorted_cycle = cycle.clone();
            sorted_cycle.sort();
            all_cycles.insert(sorted_cycle);
        }
    }

    println!("cycles: {:?}", all_cycles);
    println!("cycles count: {}", all_cycles.len());

    println!("Day 23 Part 1: {}", 0);

    println!("Day 23 Part 2: {}", 0);

    Ok(())
}

fn find_cycles_of_size_n(
    g: &HashMap<String, HashSet<String>>,
    n: usize,
    start: &str,
    visited: &mut HashSet<String>,
) -> Vec<Vec<String>> {
    let mut cycles = vec![];

    visited.insert(start.to_string());

    for neighbor in g.get(start).unwrap() {
        if visited.contains(neighbor) {
            continue;
        }

        let mut cycle = vec![start.to_string(), neighbor.to_string()];
        let mut current = neighbor;

        while cycle.len() < n {
            let next = g.get(current).unwrap().iter().next().unwrap();
            if cycle.contains(next) {
                break;
            }
            cycle.push(next.to_string());
            current = next;
        }

        if cycle.len() == n {
            cycles.push(cycle);
        }
    }

    visited.remove(start);

    cycles
}

fn cycle_contains(cycle: &Vec<String>, node: &str) -> bool {
    for i in 0..cycle.len() {
        // if cycle[i] starts with node, return true
        if cycle[i].starts_with(node) {
            return true;
        }
    }
    false
}

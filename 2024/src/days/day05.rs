use std::collections::{HashMap, HashSet};
use std::io::{self};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = match read_from_file("day5/input.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    // To parse the input, need to parse to the first empty line and those are the "rules". Rules are a | separated tuple
    // The rest are the updates, a list of ints

    let mut parsing_updates = false;
    // let mut rules = Vec::<Vec<i32>>::new();
    let mut updates = Vec::<Vec<i32>>::new();
    let mut befores = HashMap::<i32, HashSet<i32>>::new();
    let mut afters = HashMap::<i32, HashSet<i32>>::new();
    for line in data.lines() {
        if line.is_empty() {
            parsing_updates = true;
            continue;
        } else if parsing_updates {
            let mut update = Vec::new();
            for val in line.split(",") {
                match val.parse::<i32>() {
                    Ok(v) => update.push(v),
                    Err(e) => return Err(Box::new(e)),
                }
            }
            updates.push(update);
        } else {
            // Split on the | and insert into rules
            let mut rule = Vec::new();
            for val in line.split("|") {
                match val.parse::<i32>() {
                    Ok(v) => rule.push(v),
                    Err(e) => return Err(Box::new(e)),
                }
            }
            match befores.get(&rule[0]) {
                Some(_) => {
                    let h = befores.get_mut(&rule[0]).unwrap();
                    h.insert(rule[1]);
                }
                None => {
                    let mut h = HashSet::new();
                    h.insert(rule[1]);
                    befores.insert(rule[0], h);
                }
            }

            match afters.get(&rule[1]) {
                Some(_) => {
                    let h = afters.get_mut(&rule[1]).unwrap();
                    h.insert(rule[0]);
                }
                None => {
                    let mut h = HashSet::new();
                    h.insert(rule[0]);
                    afters.insert(rule[1], h);
                }
            }
        }
    }

    let mut sum = 0;
    let mut sum2: i32 = 0;
    for update in updates {
        let (_, _, correct) = in_right_order(update.clone(), befores.clone(), afters.clone());
        if correct {
            sum += middle_value(update.clone());
        } else {
            sum2 += fix_broken_rule(&mut update.clone(), befores.clone(), afters.clone())
        }
    }

    println!("Day 5 Part 1: {:?}", sum);
    println!("Day 5 Part 2: {:?}", sum2);

    Ok(())
}

fn read_from_file(file_path: &str) -> Result<String, io::Error> {
    // Read the data from the specified file path
    let data = match std::fs::read_to_string(file_path) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    Ok(data)
}

fn in_right_order(
    update: Vec<i32>,
    befores: HashMap<i32, HashSet<i32>>,
    afters: HashMap<i32, HashSet<i32>>,
) -> (usize, usize, bool) {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let i_befores = match befores.get(&update[i]) {
                Some(b) => b,
                None => {
                    continue;
                }
            };

            if !i_befores.contains(&update[j]) {
                return (i as usize, j as usize, false);
            }

            let j_befores = match befores.get(&update[j]) {
                Some(b) => b,
                None => {
                    continue;
                }
            };

            if j_befores.contains(&update[i]) {
                return (i as usize, j as usize, false);
            }
        }
    }
    // Do the same thing but backwards
    for i in (1..update.len()).rev() {
        for j in (0..=i).rev() {
            if i == j {
                continue;
            }
            let i_afters = match afters.get(&update[i]) {
                Some(b) => b,
                None => {
                    continue;
                }
            };

            if !i_afters.contains(&update[j]) {
                return (j, i, false);
            }

            let j_afters = match afters.get(&update[j]) {
                Some(b) => b,
                None => {
                    continue;
                }
            };

            if j_afters.contains(&update[i]) {
                return (j, i, false);
            }
        }
    }
    (0, 0, true)
}

fn middle_value(update: Vec<i32>) -> i32 {
    if update.len() == 0 {
        return 0;
    }
    *update.get(update.len() / 2).unwrap()
}

fn fix_broken_rule(
    update: &mut Vec<i32>,
    befores: HashMap<i32, HashSet<i32>>,
    afters: HashMap<i32, HashSet<i32>>,
) -> i32 {
    let (i, j, correct_order) = in_right_order(update.clone(), befores.clone(), afters.clone());
    if correct_order {
        return middle_value(update.to_vec());
    }

    // swap i and j
    swap(update, i, j);
    let (_, _, correct) = in_right_order(update.to_vec(), befores.clone(), afters.clone());
    if !correct {
        return fix_broken_rule(update, befores.clone(), afters.clone());
    }

    return middle_value(update.to_vec());
}

fn swap(vec: &mut Vec<i32>, index1: usize, index2: usize) {
    if index1 != index2 {
        let (left, right) = vec.split_at_mut(index1.max(index2));
        if index1 < index2 {
            std::mem::swap(&mut left[index1], &mut right[0]);
        } else {
            std::mem::swap(&mut right[0], &mut left[index2]);
        }
    }
}

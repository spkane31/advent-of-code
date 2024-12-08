use clap::Parser;
use regex::Regex;
use std::collections::{HashMap, HashSet};
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

/// This is a simple program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, action)]
    all: bool,

    #[clap(long, short, action)]
    day: Option<u32>,
}

fn main() {
    let args = Args::parse();
    if let Some(day) = args.day {
        let start = Instant::now();
        match day {
            1 => match run_day_1() {
                Ok(_) => (),
                Err(e) => println!("Error in day 3: {:?}", e),
            },
            2 => match run_day_2() {
                Ok(_) => (),
                Err(e) => println!("Error in day 2: {:?}", e),
            },
            3 => match run_day_3() {
                Ok(_) => (),
                Err(e) => println!("Error in day 3: {:?}", e),
            },
            4 => match run_day_4() {
                Ok(_) => (),
                Err(e) => println!("Error in day 4: {:?}", e),
            },
            5 => match run_day_5() {
                Ok(_) => (),
                Err(e) => println!("Error in day 5: {:?}", e),
            },
            6 => match run_day_6() {
                Ok(_) => (),
                Err(e) => println!("Error in day 6: {:?}", e),
            },
            7 => match run_day_7() {
                Ok(_) => (),
                Err(e) => println!("Error in day 7: {:?}", e),
            },
            8 => match run_day_8() {
                Ok(_) => (),
                Err(e) => println!("Error in day 8: {:?}", e),
            },
            n => println!("Solution for day {} is not implemented yet.", n),
        }
        println!("Total runtime: {:?}", start.elapsed());
    } else if args.all {
        let total = Instant::now();
        let mut count: usize = 0;
        for i in 1..26 {
            let start = Instant::now();
            match i {
                1 => match run_day_1() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 1: {:?}", e),
                },
                2 => match run_day_2() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 2: {:?}", e),
                },
                3 => match run_day_3() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 3: {:?}", e),
                },
                4 => match run_day_4() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 4: {:?}", e),
                },
                5 => match run_day_5() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 5: {:?}", e),
                },
                6 => match run_day_6() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 6: {:?}", e),
                },
                7 => match run_day_7() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 7: {:?}", e),
                },
                8 => match run_day_8() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 8: {:?}", e),
                },
                _ => {}
            }
            count += 1;
            println!("Day {} runtime: {:?}", i, start.elapsed());
        }
        let elapsed = total.elapsed();
        println!("Total runtime: {:?}", total.elapsed());
        println!("Average runtime: {:?}", elapsed / count as u32);
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
        return false;
    }
    if *min < 0 && *max > 0 {
        return false;
    }

    if differences.iter().all(|&x| x >= -3 && x <= 3 && x != 0) {
        return true;
    } else {
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

fn run_day_4() -> Result<i32> {
    let data = match read_from_file("day4/input.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };

    // Input is a word search, where the goal is to find all instances of XMAS in the puzzle

    // First, we need to parse the data into a 2D vector
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        puzzle.push(row);
    }

    // Next, we need to find all instances of XMAS in the puzzle
    // We'll use a graph search when we find the X, and search in all 8 directions
    // for the rest of the word
    let word = "XMAS";
    let mut instances = 0;
    for i in 0..puzzle.len() {
        for j in 0..puzzle[i].len() {
            if puzzle[i][j] == 'X' {
                // Search in all 8 directions for the rest of the word
                for (dx, dy) in &[
                    (0, 1),
                    (1, 0),
                    (1, 1),
                    (1, -1),
                    (0, -1),
                    (-1, 0),
                    (-1, -1),
                    (-1, 1),
                ] {
                    let mut found = true;
                    for k in 1..word.len() {
                        let new_i = i as i32 + k as i32 * dx;
                        let new_j = j as i32 + k as i32 * dy;
                        if new_i < 0
                            || new_i >= puzzle.len() as i32
                            || new_j < 0
                            || new_j >= puzzle[i].len() as i32
                        {
                            found = false;
                            break;
                        }
                        let c = match word.chars().nth(k) {
                            Some(c) => c,
                            None => {
                                found = false;
                                break;
                            }
                        };
                        if puzzle[new_i as usize][new_j as usize] != c {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        instances += 1;
                    }
                }
            }
        }
    }

    println!("Day 4 Part 1: {:?}", instances);

    // For part 2, need to find all crossing MAS instances so:
    // M . S
    // . A .
    // M . S
    // The above counts for one instance, so we need to find all A
    // instances and look for two M-S pairs diagonal to it
    let mut instances = 0;
    for a in 1..puzzle.len() - 1 {
        for b in 1..puzzle[a].len() - 1 {
            if puzzle[a][b] == 'A' {
                // Configuration one is S . M above and S . M below, check for that specifically
                if puzzle[a - 1][b - 1] == 'S'
                    && puzzle[a - 1][b + 1] == 'M'
                    && puzzle[a + 1][b - 1] == 'S'
                    && puzzle[a + 1][b + 1] == 'M'
                {
                    instances += 1;
                }

                // Configuration two is M . M above and S . S below, check for that specifically
                if puzzle[a - 1][b - 1] == 'M'
                    && puzzle[a - 1][b + 1] == 'M'
                    && puzzle[a + 1][b - 1] == 'S'
                    && puzzle[a + 1][b + 1] == 'S'
                {
                    instances += 1;
                }

                // Configuration three is S . S above and M . M below, check for that specifically
                if puzzle[a - 1][b - 1] == 'S'
                    && puzzle[a - 1][b + 1] == 'S'
                    && puzzle[a + 1][b - 1] == 'M'
                    && puzzle[a + 1][b + 1] == 'M'
                {
                    instances += 1;
                }

                // Configuration four is M . S above and M . S below, check for that specifically
                if puzzle[a - 1][b - 1] == 'M'
                    && puzzle[a - 1][b + 1] == 'S'
                    && puzzle[a + 1][b - 1] == 'M'
                    && puzzle[a + 1][b + 1] == 'S'
                {
                    instances += 1;
                }
            }
        }
    }

    println!("Day 4 Part 2: {:?}", instances);

    Ok(0)
}

fn run_day_5() -> Result<i32> {
    let data = match read_from_file("day5/input.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
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
                    Err(e) => {
                        return Err(AOCError {
                            details: format!("Error parsing update: {:?}, {:?}", e, val),
                        })
                    }
                }
            }
            updates.push(update);
        } else {
            // Split on the | and insert into rules
            let mut rule = Vec::new();
            for val in line.split("|") {
                match val.parse::<i32>() {
                    Ok(v) => rule.push(v),
                    Err(e) => {
                        return Err(AOCError {
                            details: format!("Error parsing update: {:?}, {:?}", e, val),
                        })
                    }
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

    Ok(0)
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

fn run_day_6() -> Result<i32> {
    let data = match read_from_file("day6/sample.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };

    // Conver the input into a 2D vector of chars
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    part1(grid.clone());

    let mut set = HashSet::new();

    let (mut i, mut j, mut on_board) = has_guard(grid.clone());
    while on_board {
        (grid, set) = move_guard(grid.clone(), i, j, set.clone());
        (i, j, on_board) = has_guard(grid.clone());
    }

    println!("Day 6 Part 1: {:?}", set.len());

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    Ok(0)
}

fn has_guard(grid: Vec<Vec<char>>) -> (usize, usize, bool) {
    // Looking for either ^, >, <, or v
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' || grid[i][j] == '>' || grid[i][j] == '<' || grid[i][j] == 'v' {
                return (i, j, true);
            }
        }
    }
    (0, 0, false)
}

fn move_guard(
    mut grid: Vec<Vec<char>>,
    i: usize,
    j: usize,
    mut set: HashSet<(usize, usize)>,
) -> (Vec<Vec<char>>, HashSet<(usize, usize)>) {
    // Get the character at i,j and find di, dj for this move
    let c = grid[i][j];
    let (di, dj) = match c {
        '^' => (-1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        'v' => (1, 0),
        _ => (0, 0),
    };
    set.insert((i, j));

    // Now move the guard in the direction of di,dj until we hit a wall '#' or move off the board (negative values)
    let mut new_i = i as i32 + di;
    let mut new_j = j as i32 + dj;
    while new_i >= 0
        && new_j >= 0
        && new_i < grid.len() as i32
        && new_j < grid[new_i as usize].len() as i32
    {
        if grid[new_i as usize][new_j as usize] == '#' {
            break;
        }
        set.insert((new_i as usize, new_j as usize));
        grid[new_i as usize][new_j as usize] = 'X';
        new_i += di;
        new_j += dj;
    }
    // Set the previous spot to '.'
    grid[i][j] = 'X';
    // Set the new spot to the guard with a 90 degree right turn
    let new_c = match c {
        '^' => '>',
        '>' => 'v',
        '<' => '^',
        'v' => '<',
        _ => '.',
    };
    if new_i >= 0
        && new_j >= 0
        && new_i < grid.len() as i32
        && new_j < grid[new_i as usize].len() as i32
    {
        grid[(new_i - di) as usize][(new_j - dj) as usize] = new_c;
    }
    return (grid, set);
}

fn pretty_print(grid: &Vec<Vec<char>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

type Graph = HashMap<(usize, usize), HashSet<(usize, usize)>>;

fn print_graph(graph: &Graph) {
    // Want to print as a grid, with '.' indicating no edge and '#' indicating an edge
    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..10 {
        let mut row = Vec::new();
        for _ in 0..10 {
            row.push('.');
        }
        grid.push(row);
    }

    for ((i, j), _) in graph {
        grid[*i][*j] = '#';
    }

    pretty_print(&grid);
}

fn add_edge(graph: &mut Graph, i: usize, j: usize, new_i: usize, new_j: usize) {
    if i == new_i && j == new_j {
        return;
    }
    println!(
        "Adding edge from {:?} {:?} to {:?} {:?}",
        i, j, new_i, new_j
    );
    match graph.get_mut(&(i, j)) {
        Some(h) => {
            h.insert((new_i, new_j));
        }
        None => {
            let mut h = HashSet::new();
            h.insert((new_i, new_j));
            graph.insert((i, j), h);
        }
    }
}

fn size(graph: &Graph) -> usize {
    let mut s: usize = 0;

    for (_, h) in graph {
        s += h.len();
    }

    return s;
}

fn part1(grid: Vec<Vec<char>>) {
    let mut graph: Graph = HashMap::new();

    let (mut a, mut b) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '<' || grid[i][j] == '>' || grid[i][j] == '^' || grid[i][j] == 'v' {
                (a, b) = (i, j);
                break;
            }
        }
        if a != 0 && b != 0 {
            break;
        }
    }

    println!("Found starting point at {:?} {:?}", a, b);

    // Now walk and add edges until the guard walks off the map (defined as the width and length of the grid)
    let mut i = a;
    let mut j = b;
    let mut c = grid[i][j];
    while i > 0 && j > 0 && i < grid.len() - 1 && j < grid[i].len() - 1 {
        let (mut di, mut dj) = match c {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => (0, 0),
        };

        // Walk in the direction of di, dj until we hit an obstacle (#) or walk off the map
        let mut new_i = i as i32 + di;
        let mut new_j = j as i32 + dj;
        while new_i >= 0 && new_j >= 0 && new_i < grid.len() as i32 && new_j < grid[i].len() as i32
        {
            if grid[new_i as usize][new_j as usize] == '#' {
                new_i -= di;
                new_j -= dj;
                println!("Rotating at {:?} {:?}", new_i, new_j);
                // Rotate the direction 90 degrees
                (di, dj) = match c {
                    '^' => (0, 1),
                    '>' => (1, 0),
                    'v' => (0, -1),
                    '<' => (-1, 0),
                    _ => (0, 0),
                };
                c = match c {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => '.',
                };
                // break;
                continue;
            }
            add_edge(&mut graph, i, j, new_i as usize, new_j as usize);
            i = new_i as usize;
            j = new_j as usize;
            new_i += di;
            new_j += dj;
        }
    }

    print_graph(&graph);
    println!("Day 6 Part 1: {:?}", size(&graph));
}

fn run_day_7() -> Result<i32> {
    let data = match read_from_file("day7/input.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };

    let mut sum: u64 = 0;
    let mut sum2: u64 = 0;
    for line in data.lines() {
        // Split on the ":"
        let (total, vals) = line.split_once(":").unwrap();

        // total is an i32, vals is a space separated list of i32s
        let total = match total.parse::<u64>() {
            Ok(t) => t,
            Err(e) => {
                return Err(AOCError {
                    details: format!("Error parsing total ({:?}): {:?}", total, e),
                })
            }
        };

        let vals = vals.split_whitespace();
        let mut all_vals = Vec::new();
        for val in vals {
            match val.parse::<u64>() {
                Ok(v) => all_vals.push(v),
                Err(e) => {
                    return Err(AOCError {
                        details: format!("Error parsing val: {:?}", e),
                    })
                }
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

    Ok(0)
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

fn run_day_8() -> Result<i32> {
    let data = match read_from_file("day8/input.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    // Create a hashmap of all the non-'.' values. There are multiple for each character
    for line in data.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            if c != '.' {
                if !nodes.contains_key(&c.clone()) {
                    nodes.insert(c.clone(), Vec::new());
                }
                nodes
                    .get_mut(&c.clone())
                    .unwrap()
                    .push((grid.len(), row.len()));
            }
            row.push(c);
        }
        grid.push(row);
    }

    let (width, height) = (grid[0].len(), grid.len());

    let mut new_nodes = HashSet::<(usize, usize)>::new();
    for (_node, locations) in nodes.iter_mut() {
        locations.sort();

        for i in 0..locations.len() - 1 {
            for j in i + 1..locations.len() {
                // 1. Want to extend the line by locations_i, locations_j in both directions one time and add
                // the new locations to new_nodes. Make sure to only add one time in each direction

                let (new_i, new_j) = next_point(
                    locations[i].0 as i32,
                    locations[i].1 as i32,
                    locations[j].0 as i32,
                    locations[j].1 as i32,
                    1,
                );
                if new_i >= 0 && new_i < height as i32 && new_j >= 0 && new_j < width as i32 {
                    new_nodes.insert((new_i as usize, new_j as usize));
                }
                let (new_i, new_j) = next_point(
                    locations[i].0 as i32,
                    locations[i].1 as i32,
                    locations[j].0 as i32,
                    locations[j].1 as i32,
                    -1,
                );
                if new_i >= 0 && new_i < height as i32 && new_j >= 0 && new_j < width as i32 {
                    new_nodes.insert((new_i as usize, new_j as usize));
                }
            }
        }
    }

    println!("day 8 part 1: {:?}", new_nodes.len());

    let mut new_nodes = HashSet::<(usize, usize)>::new();
    for (_node, locations) in nodes.iter_mut() {
        locations.sort();

        for i in 0..locations.len() - 1 {
            for j in i + 1..locations.len() {
                // 1. Want to extend the line by locations_i, locations_j in both directions one time and add
                // the new locations to new_nodes. Make sure to only add one time in each direction

                new_nodes.insert(locations[i]);
                new_nodes.insert(locations[j]);

                let mut direction = 1;
                let (mut new_i, mut new_j) = next_point(
                    locations[i].0 as i32,
                    locations[i].1 as i32,
                    locations[j].0 as i32,
                    locations[j].1 as i32,
                    direction,
                );
                while new_i >= 0 && new_i < height as i32 && new_j >= 0 && new_j < width as i32 {
                    new_nodes.insert((new_i as usize, new_j as usize));
                    direction += 1;
                    (new_i, new_j) = next_point(
                        locations[i].0 as i32,
                        locations[i].1 as i32,
                        locations[j].0 as i32,
                        locations[j].1 as i32,
                        direction,
                    );
                }
                direction = -1;
                (new_i, new_j) = next_point(
                    locations[i].0 as i32,
                    locations[i].1 as i32,
                    locations[j].0 as i32,
                    locations[j].1 as i32,
                    -1,
                );
                while new_i >= 0 && new_i < height as i32 && new_j >= 0 && new_j < width as i32 {
                    new_nodes.insert((new_i as usize, new_j as usize));
                    direction -= 1;
                    (new_i, new_j) = next_point(
                        locations[i].0 as i32,
                        locations[i].1 as i32,
                        locations[j].0 as i32,
                        locations[j].1 as i32,
                        direction,
                    );
                }
            }
        }
    }

    println!("day 8 part 2: {:?}", new_nodes.len());

    Ok(0)
}

fn next_point(i0: i32, j0: i32, i1: i32, j1: i32, direction: i32) -> (i32, i32) {
    let dx = i1 - i0;
    let dy = j1 - j0;
    if direction > 0 {
        let next_i = i1 + (direction * dx);
        let next_j = j1 + (direction * dy);
        return (next_i, next_j);
    }
    let next_i = i0 + (direction * dx);
    let next_j = j0 + (direction * dy);
    (next_i, next_j)
}

use clap::Parser;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::time::Instant;

mod days;

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
            1 => match days::day01::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 1: {:?}", e),
            },
            2 => match days::day02::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 2: {:?}", e),
            },
            3 => match days::day03::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 3: {:?}", e),
            },
            4 => match days::day04::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 4: {:?}", e),
            },
            5 => match days::day05::run() {
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
            9 => match run_day_9() {
                Ok(_) => (),
                Err(e) => println!("Error in day 9: {:?}", e),
            },
            10 => match run_day_10() {
                Ok(_) => (),
                Err(e) => println!("Error in day 10: {:?}", e),
            },
            11 => match days::day11::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 11: {:?}", e),
            },
            12 => match days::day12::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 12: {:?}", e),
            },
            13 => match days::day13::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 13: {:?}", e),
            },
            14 => match days::day14::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 14: {:?}", e),
            },
            15 => match days::day15::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 15: {:?}", e),
            },
            // 16 => match days::day16::run() {
            //     Ok(_) => (),
            //     Err(e) => println!("Error in day 16: {:?}", e),
            // },
            17 => match days::day17::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 17: {:?}", e),
            },
            _ => {}
            n => println!("Solution for day {} is not implemented yet.", n),
        }
        println!("Total runtime: {:?}", start.elapsed());
    } else if args.all {
        let total = Instant::now();
        let mut count: usize = 0;

        let funcs = vec![
            days::day01::run,
            days::day02::run,
            days::day03::run,
            days::day04::run,
            days::day05::run,
            days::day10::run,
            days::day11::run,
            days::day12::run,
            days::day13::run,
            days::day14::run,
            days::day15::run,
            days::day17::run,
        ];

        // for func in funcs {
        //     let start = Instant::now();
        //     match func() {
        //         Ok(_) => (),
        //         Err(e) => println!("Error in day {}: {:?}", count + 1, e),
        //     }
        //     count += 1;
        //     println!("Day {} runtime: {:?}", count, start.elapsed());
        // }

        for i in 1..26 {
            let start = Instant::now();
            match i {
                1 => match days::day01::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 1: {:?}", e),
                },
                2 => match days::day02::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 2: {:?}", e),
                },
                3 => match days::day03::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 3: {:?}", e),
                },
                4 => match days::day04::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 4: {:?}", e),
                },
                5 => match days::day05::run() {
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
                9 => match run_day_9() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 9: {:?}", e),
                },
                10 => match run_day_10() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 10: {:?}", e),
                },
                11 => match days::day11::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 11: {:?}", e),
                },
                12 => match days::day12::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 12: {:?}", e),
                },
                13 => match days::day13::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 13: {:?}", e),
                },
                14 => match days::day14::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 14: {:?}", e),
                },
                15 => match days::day15::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 15: {:?}", e),
                },
                // 16 => match days::day16::run() {
                //     Ok(_) => (),
                //     Err(e) => println!("Error in day 16: {:?}", e),
                // },
                17 => match days::day17::run() {
                    Ok(_) => (),
                    Err(e) => println!("Error in day 17: {:?}", e),
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

type Graph = HashMap<(usize, usize), HashSet<(usize, usize)>>;

fn add_edge(graph: &mut Graph, i: usize, j: usize, new_i: usize, new_j: usize) {
    if i == new_i && j == new_j {
        return;
    }
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

fn run_day_9() -> Result<i32> {
    let data = match read_from_file("day9/input.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };

    // Convert data (a single string) to a Vec of i64s, there is no spacing it's a single long string
    let mut vals: Vec<usize> = Vec::new();
    for c in data.chars() {
        match c.to_digit(10) {
            Some(d) => vals.push(d as usize),
            None => continue,
        }
    }

    let disk = vals;

    // Start at the first free block and the last file.
    let mut left = 0;
    let mut right = disk.len() - 2 + disk.len() % 2;
    let mut needed = disk[right];
    let mut block = 0;
    let mut checksum = 0;

    while left < right {
        // When moving to the next free block, add the checksum for the file we're skipping over.
        (checksum, block) = update(checksum, block, left, disk[left]);
        let mut available = disk[left + 1];
        left += 2;

        while available > 0 {
            if needed == 0 {
                if left == right {
                    break;
                }
                right -= 2;
                needed = disk[right];
            }

            // Take as much space as possible from the current free block range.
            let size = needed.min(available);
            (checksum, block) = update(checksum, block, right, size);
            available -= size;
            needed -= size;
        }
    }

    // Account for any remaining file blocks left over.
    (checksum, _) = update(checksum, block, right, needed);
    println!("Day 9 Part 1: {:?}", checksum);
    println!("Day 9 Part 2: {:?}", part2(&disk));

    Ok(0)
}

pub fn part2(disk: &[usize]) -> usize {
    let mut block = 0;
    let mut checksum = 0;
    let mut free: Vec<_> = (0..10).map(|_| MinHeap::with_capacity(1_000)).collect();

    // Build a min-heap (leftmost free block first) where the size of each block is
    // implicit in the index of the array.
    for (index, &size) in disk.iter().enumerate() {
        if index % 2 == 1 && size > 0 {
            free[size].push(block, ());
        }

        block += size;
    }

    for (index, &size) in disk.iter().enumerate().rev() {
        block -= size;

        // Count any previous free blocks to decrement block offset correctly.
        if index % 2 == 1 {
            continue;
        }

        // Find the leftmost free block that can fit the file (if any).
        let mut next_block = block;
        let mut next_index = usize::MAX;

        #[allow(clippy::needless_range_loop)]
        for i in size..free.len() {
            if let Some((&first, ())) = free[i].peek() {
                if first < next_block {
                    next_block = first;
                    next_index = i;
                }
            }
        }

        // We can make smaller free block from bigger blocks but not the other way around.
        // As an optimization if all blocks of the biggest size are after our position then
        // we can ignore them.
        if !free.is_empty() {
            let last = free.len() - 1;
            if let Some((&first, ())) = free[last].peek() {
                if first > block {
                    free.pop();
                }
            }
        }

        // Update the checksum with the file's location (possibly unchanged).
        let id = index / 2;
        let extra = next_block * size + EXTRA[size];
        checksum += id * extra;

        // If we used a free block, remove then add back any leftover space.
        if next_index != usize::MAX {
            free[next_index].pop();
            if size < next_index {
                free[next_index - size].push(next_block + size, ());
            }
        }
    }

    checksum
}

const EXTRA: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

fn update(checksum: usize, block: usize, index: usize, size: usize) -> (usize, usize) {
    let id = index / 2;
    let extra = block * size + EXTRA[size];
    (checksum + id * extra, block + size)
}

struct Wrapper<K: Ord, V> {
    key: K,
    value: V,
}

impl<K: Ord, V> PartialEq for Wrapper<K, V> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for Wrapper<K, V> {}

impl<K: Ord, V> PartialOrd for Wrapper<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for Wrapper<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.key.cmp(&self.key)
    }
}

#[derive(Default)]
pub struct MinHeap<K: Ord, V> {
    heap: BinaryHeap<Wrapper<K, V>>,
}

impl<K: Ord, V> MinHeap<K, V> {
    pub fn new() -> Self {
        MinHeap {
            heap: BinaryHeap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        MinHeap {
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn push(&mut self, key: K, value: V) {
        self.heap.push(Wrapper { key, value });
    }

    #[inline]
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.heap.pop().map(|w| (w.key, w.value))
    }

    #[inline]
    pub fn peek(&self) -> Option<(&K, &V)> {
        self.heap.peek().map(|w| (&w.key, &w.value))
    }
}

fn run_day_10() -> Result<i32> {
    let data = match read_from_file("day10/input.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };

    // Convert data (a single string) to a Vec of i64s, there is no spacing it's a single long string
    let mut vals: Vec<Vec<u8>> = Vec::new();
    for line in data.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            match c.to_digit(10) {
                Some(d) => row.push(d as u8),
                None => continue,
            }
        }
        vals.push(row);
    }

    let total_score = calculate_total_score(vals.clone());
    let total_rating = calculate_total_rating(vals.clone());
    println!("Day 10 Part 1: {}", total_score);
    println!("Day 10 Part 2: {}", total_rating);

    Ok(0)
}

/// Directions for up, down, left, and right movements.
const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // up
    (1, 0),  // down
    (0, -1), // left
    (0, 1),  // right
];

/// Check if a position is within the bounds of the height map.
fn in_bounds(x: isize, y: isize, height: usize, width: usize) -> bool {
    x >= 0 && x < height as isize && y >= 0 && y < width as isize
}

/// Perform a breadth-first search (BFS) to find all 9-height positions reachable from a given trailhead.
fn bfs_find_trail_score(map: &Vec<Vec<u8>>, start_x: usize, start_y: usize) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<((usize, usize), u8)> = VecDeque::new(); // ((x, y), current height)
    queue.push_back(((start_x, start_y), 0));
    let mut score = 0;

    while let Some(((x, y), current_height)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        // Check if this position is a 9 and count it for the score
        if map[x][y] == 9 {
            score += 1;
        }

        for (dx, dy) in DIRECTIONS.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if in_bounds(nx, ny, height, width) {
                let nx = nx as usize;
                let ny = ny as usize;

                // The trail can only move to a tile that is exactly one height higher
                if map[nx][ny] == current_height + 1 {
                    queue.push_back(((nx, ny), current_height + 1));
                }
            }
        }
    }

    score
}

/// Perform a depth-first search (DFS) to find all distinct hiking trails from a given trailhead.
fn dfs_count_trails(
    map: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    current_height: u8,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if visited.contains(&(x, y)) || map[x][y] != current_height {
        return 0;
    }

    visited.insert((x, y));

    if map[x][y] == 9 {
        visited.remove(&(x, y));
        return 1;
    }

    let mut total_trails = 0;
    for (dx, dy) in DIRECTIONS.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if in_bounds(nx, ny, map.len(), map[0].len()) {
            total_trails +=
                dfs_count_trails(map, nx as usize, ny as usize, current_height + 1, visited);
        }
    }

    visited.remove(&(x, y));
    total_trails
}

/// Calculate the total score for all trailheads in the height map.
fn calculate_total_score(map: Vec<Vec<u8>>) -> usize {
    let mut total_score = 0;
    let height = map.len();
    let width = map[0].len();

    for x in 0..height {
        for y in 0..width {
            if map[x][y] == 0 {
                // A trailhead must start at height 0
                let trail_score = bfs_find_trail_score(&map, x, y);
                total_score += trail_score;
            }
        }
    }

    total_score
}

/// Calculate the total rating for all trailheads in the height map.
fn calculate_total_rating(map: Vec<Vec<u8>>) -> usize {
    let mut total_rating = 0;
    let height = map.len();
    let width = map[0].len();

    for x in 0..height {
        for y in 0..width {
            if map[x][y] == 0 {
                // A trailhead must start at height 0
                let mut visited = HashSet::new();
                let trail_rating = dfs_count_trails(&map, x, y, 0, &mut visited);
                total_rating += trail_rating;
            }
        }
    }

    total_rating
}

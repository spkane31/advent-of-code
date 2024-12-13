use std::io::{self};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = match read_from_file("day4/input.txt") {
        Ok(d) => d,
        Err(e) => {
            return Err(Box::new(e));
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

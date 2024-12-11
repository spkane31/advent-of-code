use std::collections::{HashSet, VecDeque};
use std::io::{self};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_from_file("day10/input.txt")?;

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

    Ok(())
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

fn read_from_file(file_path: &str) -> Result<String, io::Error> {
    // Read the data from the specified file path
    let data = match std::fs::read_to_string(file_path) {
        Ok(d) => d,
        Err(e) => {
            println!("Error reading file");
            return Err(e);
        }
    };
    Ok(data)
}

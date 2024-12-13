use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // read in the data from day12/sample.txt and parse into a Vec<Vec<char>>
    let data = read_from_file("day12/input.txt")?;

    // data is a map of chars, find the number of groups where the char is the same
    // the char can be any letter. And a region can be any number of the same letter
    // the region can move horizontally or vertically. Find all the regions using
    // a depth first search in a Vec<HashSet<(usize, usize)>> where the usize is the
    // row and column of the char in the data Vec<Vec<char>>.

    let mut seen: Vec<Vec<bool>> = vec![vec![false; data[0].len()]; data.len()];

    let mut maps: Vec<Vec<(usize, usize)>> = Vec::new();

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if !seen[i][j] {
                let new_map = dfs(&data, &mut seen, i, j);
                for (x, y) in new_map.clone() {
                    seen[x][y] = true;
                }
                maps.push(new_map);
            }
        }
    }

    let mut sum: usize = 0;
    for map in maps.iter() {
        sum += perimeter(map.clone()) * map.len();
    }

    println!("Day 12 Part 1: {}", sum);

    let mut sum: usize = 0;
    for map in maps.iter() {
        let v: Vec<(i32, i32)> = map.iter().map(|(x, y)| (*x as i32, *y as i32)).collect();
        sum += count_region_sides(v.clone()) * map.len();
    }

    println!("Day 12 Part 2: {}", sum);
    Ok(())
}

fn dfs(
    data: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
) -> Vec<(usize, usize)> {
    let mut stack = vec![(i, j)];
    let mut new_map = Vec::new();

    while let Some((x, y)) = stack.pop() {
        if x >= data.len() || y >= data[x].len() || seen[x][y] {
            continue;
        }

        seen[x][y] = true;
        new_map.push((x, y));

        if x > 0 && data[x - 1][y] == data[x][y] {
            stack.push((x - 1, y));
        }
        if y > 0 && data[x][y - 1] == data[x][y] {
            stack.push((x, y - 1));
        }
        if x < data.len() - 1 && data[x + 1][y] == data[x][y] {
            stack.push((x + 1, y));
        }
        if y < data[x].len() - 1 && data[x][y + 1] == data[x][y] {
            stack.push((x, y + 1));
        }
    }

    new_map
}

fn read_from_file(filename: &str) -> Result<Vec<Vec<char>>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        data.push(row);
    }

    Ok(data)
}
fn perimeter(points: Vec<(usize, usize)>) -> usize {
    let mut perimeter = 0;
    let point_set: HashSet<(usize, usize)> = points.into_iter().collect();

    // Directions representing top, bottom, left, and right neighbors
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for &(x, y) in &point_set {
        let mut exposed_sides = 4; // Each cell has 4 sides
        for &(dx, dy) in &directions {
            let neighbor = (x.wrapping_add(dx as usize), y.wrapping_add(dy as usize));
            if point_set.contains(&neighbor) {
                exposed_sides -= 1; // Neighboring point reduces the number of exposed sides
            }
        }
        perimeter += exposed_sides;
    }

    perimeter
}

fn count_region_sides(region: Vec<(i32, i32)>) -> usize {
    let mut side_count = 0;
    for dir in DIR {
        let mut sides = HashSet::new();
        for pos in region.iter() {
            let tmp = (pos.0 + dir.0, pos.1 + dir.1);
            if !region.contains(&tmp) {
                sides.insert(tmp);
            }
        }
        let mut remove: HashSet<(i32, i32)> = HashSet::default();
        for side in &sides {
            let mut tmp = (side.0 + dir.1, side.1 + dir.0);
            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = (tmp.0 + dir.1, tmp.1 + dir.0);
            }
        }
        side_count += sides.len() - remove.len();
    }

    side_count as usize
}

const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

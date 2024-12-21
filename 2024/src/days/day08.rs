use std::collections::{HashMap, HashSet};
use std::{fs, io};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = match read_from_file("day8/input.txt") {
        Ok(d) => d,
        Err(e) => return Err(e.into()),
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

    Ok(())
}

fn read_from_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
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

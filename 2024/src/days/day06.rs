use std::collections::{HashMap, HashSet};
use std::{fs, io};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = match read_from_file("day6/sample.txt") {
        Ok(d) => d,
        Err(e) => return Err(e.into()),
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

    Ok(())
}

fn read_from_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
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

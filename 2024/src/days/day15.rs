use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let data = match read_file_to_string("day15/sample.txt") {
        Ok(content) => content,
        Err(e) => return Err(e.into()),
    };

    let (grid, moves) = data.split_once("\n\n").unwrap();

    let grid = parse_grid(grid);
    let moves = parse_moves(moves);
    let (x, y) = location_of(&grid);

    for m in moves {
        match m {
            "<" => {
                println!("left")
            }
            ">" => {
                println!("right")
            }
            "^" => {
                println!("up")
            }
            "v" => {
                println!("down")
            }
            _ => println!("unknown"),
        }
    }

    println!("Day 14 Part 1: {}", 0);

    println!("Day 14 Part 2: {}", 0);

    Ok(())
}

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

fn parse_grid(grid: &str) -> Vec<Vec<char>> {
    grid.lines().map(|line| line.chars().collect()).collect()
}

fn parse_moves(moves: &str) -> Vec<&str> {
    moves.lines().collect()
}

fn location_of(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '@' {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day13/input.txt")?;
    let reader = BufReader::new(file);

    let mut total_tokens = 0;
    let mut idx = 0;
    let mut matrix = Matrix2x2 {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    let mut b: Vector2 = Vector2 { x: 0, y: 0 };
    for line in reader.lines() {
        let line = line?;

        if idx % 4 == 0 || idx % 4 == 1 {
            let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();

            if let Some(captures) = re.captures(line.as_str()) {
                let x_value = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let y_value = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

                if idx % 4 == 0 {
                    matrix.a = x_value as i64;
                    matrix.c = y_value as i64;
                } else {
                    matrix.b = x_value as i64;
                    matrix.d = y_value as i64;
                }
            } else {
                println!("No match found");
            }
        } else if idx % 4 == 2 {
            let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

            if let Some(captures) = re.captures(line.as_str()) {
                let x_value = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let y_value = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
                b.x = x_value as i64;
                b.y = y_value as i64;
            } else {
                println!("No match found: {}", line);
            }
        } else {
            match solve_linear_system(matrix, b) {
                Some(solution) => {
                    if validate(&matrix, &b, &solution) {
                        let t = (3 * (solution.x as i64) + (solution.y as i64)) as i64;
                        total_tokens += t;
                    }
                }
                None => {}
            }
        }

        idx += 1;
    }
    match solve_linear_system(matrix, b) {
        Some(solution) => {
            if validate(&matrix, &b, &solution) {
                let t = (3 * (solution.x as i64) + (solution.y as i64)) as i64;
                total_tokens += t;
            }
        }
        None => {}
    }

    println!("Day 13 Part 1: {}", total_tokens);

    run2()
}

fn run2() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day13/input.txt")?;
    let reader = BufReader::new(file);

    let mut total_tokens = 0;
    let mut idx = 0;
    let mut matrix = Matrix2x2 {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    let mut b: Vector2 = Vector2 { x: 0, y: 0 };
    for line in reader.lines() {
        let line = line?;

        if idx % 4 == 0 || idx % 4 == 1 {
            let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();

            if let Some(captures) = re.captures(line.as_str()) {
                let x_value = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let y_value = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

                if idx % 4 == 0 {
                    matrix.a = x_value as i64;
                    matrix.c = y_value as i64;
                } else {
                    matrix.b = x_value as i64;
                    matrix.d = y_value as i64;
                }
            } else {
                println!("No match found");
            }
        } else if idx % 4 == 2 {
            let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

            if let Some(captures) = re.captures(line.as_str()) {
                let x_value = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let y_value = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
                b.x = x_value + 10000000000000;
                b.y = y_value + 10000000000000;
            } else {
                println!("No match found: {}", line);
            }
        } else {
            match solve_linear_system(matrix, b) {
                Some(solution) => {
                    if validate(&matrix, &b, &solution) {
                        let t = (3 * (solution.x as i64) + (solution.y as i64)) as i64;
                        total_tokens += t;
                    }
                }
                None => {}
            }
        }

        idx += 1;
    }
    match solve_linear_system(matrix, b) {
        Some(solution) => {
            if validate(&matrix, &b, &solution) {
                let t = (3 * (solution.x as i64) + (solution.y as i64)) as i64;
                total_tokens += t;
            }
        }
        None => {}
    }

    println!("Day 13 Part 2: {}", total_tokens);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Matrix2x2 {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

#[derive(Debug, Clone, Copy)]
struct Vector2 {
    x: i64,
    y: i64,
}

impl Matrix2x2 {
    /// Computes the determinant of the 2x2 matrix
    fn determinant(&self) -> i64 {
        self.a * self.d - self.b * self.c
    }
}

fn solve_linear_system(a: Matrix2x2, b: Vector2) -> Option<Vector2> {
    let det = a.determinant();
    if det == 0 {
        // No unique solution exists if the determinant is zero
        return None;
    }

    // Apply Cramer's Rule to solve for X and B
    let x_num = b.x * a.d - a.b * b.y; // Numerator for X
    let b_num = a.a * b.y - b.x * a.c; // Numerator for B

    if x_num % det != 0 || b_num % det != 0 {
        // Check if x_num and b_num are cleanly divisible by the determinant
        return None; // No integer solution exists
    }

    let x = x_num / det;
    let y = b_num / det;

    Some(Vector2 { x, y })
}

fn validate(matrix: &Matrix2x2, b: &Vector2, solution: &Vector2) -> bool {
    let x = solution.x;
    let y = solution.y;

    let x_prime = matrix.a * x + matrix.b * y;
    let y_prime = matrix.c * x + matrix.d * y;

    (x_prime - b.x).abs() == 0 && (y_prime - b.y).abs() == 0
}

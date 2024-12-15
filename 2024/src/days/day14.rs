use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day14/input.txt")?;
    let reader = BufReader::new(file);

    let sim: usize = 100;
    let wide: usize = 101;
    let tall: usize = 103;

    let mut robots = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

        if let Some(captures) = re.captures(&line) {
            let p1 = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let p2 = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let v1 = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let v2 = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();

            robots.push(Robot {
                position: (p1, p2),
                velocity: (v1, v2),
                grid: (wide, tall),
            });
        }
    }

    for i in 0..sim {
        for robot in robots.iter_mut() {
            robot.simulate();
        }
        let (xi, yi) = std_dev(robots.clone());
        if (xi < 25) && (yi < 25) {
            println!("{}: {:?}", i, std_dev(robots.clone()));
        }
    }

    let mut grid = vec![vec![0; wide]; tall];
    for robot in robots.iter() {
        let x = robot.position.0 as usize % wide;
        let y = robot.position.1 as usize % tall;
        grid[y][x] += 1;
    }

    let mut quads = vec![0; 4];

    for robot in robots.iter() {
        let x = robot.position.0 as usize % wide;
        let y = robot.position.1 as usize % tall;

        if x < wide / 2 && y < tall / 2 {
            quads[0] += 1;
        } else if x > wide / 2 && y < tall / 2 {
            quads[1] += 1;
        } else if x < wide / 2 && y > tall / 2 {
            quads[2] += 1;
        } else if x > wide / 2 && y > tall / 2 {
            quads[3] += 1;
        }
    }

    println!("Day 14 Part 1: {}", product(quads));

    let mut i = sim;
    loop {
        i += 1;
        for robot in robots.iter_mut() {
            robot.simulate();
        }
        let (xi, yi) = std_dev(robots.clone());
        if (xi < 25) && (yi < 25) {
            break;
        }
    }
    println!("Day 14 Part 2: {}", i);

    Ok(())
}

#[derive(Debug, Clone)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
    grid: (usize, usize),
}

impl Robot {
    fn simulate(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        while self.position.0 < 0 {
            self.position.0 += self.grid.0 as i64;
        }
        while self.position.1 < 0 {
            self.position.1 += self.grid.1 as i64;
        }
        while self.position.0 >= self.grid.0 as i64 {
            self.position.0 -= self.grid.0 as i64;
        }
        while self.position.1 >= self.grid.1 as i64 {
            self.position.1 -= self.grid.1 as i64;
        }
    }
    // }
}

fn product(quads: Vec<usize>) -> usize {
    let mut prod = 1;
    for quad in quads.iter() {
        prod *= quad;
    }
    prod
}

fn std_dev(robots: Vec<Robot>) -> (i64, i64) {
    // Compute the std dev of the robots
    let mut pos: Vec<(i64, i64)> = vec![(0, 0); robots.len()];
    for (i, robot) in robots.iter().enumerate() {
        pos[i] = robot.position;
    }

    let mut mean = (0, 0);
    for p in pos.iter() {
        mean.0 += p.0;
        mean.1 += p.1;
    }
    mean.0 /= pos.len() as i64;
    mean.1 /= pos.len() as i64;

    let mut std_dev = (0, 0);
    for p in pos.iter() {
        std_dev.0 += (p.0 - mean.0).pow(2);
        std_dev.1 += (p.1 - mean.1).pow(2);
    }
    std_dev.0 /= pos.len() as i64;
    std_dev.1 /= pos.len() as i64;

    std_dev.0 = (std_dev.0 as f64).sqrt() as i64;
    std_dev.1 = (std_dev.1 as f64).sqrt() as i64;

    return std_dev;
}

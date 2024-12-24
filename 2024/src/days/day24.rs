use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Instruction {
    wire1: String,
    wire2: String,
    op: Op,
    target: String,
    completed: bool,
}

#[derive(Debug)]
enum Op {
    AND,
    OR,
    XOR,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Op::AND => write!(f, "AND"),
            Op::OR => write!(f, "OR"),
            Op::XOR => write!(f, "XOR"),
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("day24/input.txt")?;

    let (registers, instructions_str) = input.split_once("\n\n").unwrap();

    let mut state: HashMap<String, i64> = HashMap::new();
    registers.lines().for_each(|line| {
        let mut parts = line.split(": ");
        let register = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i64>().unwrap();
        state.insert(register.to_string(), value);
    });

    let mut instructions: Vec<Instruction> = Vec::new();
    instructions_str.lines().for_each(|line| {
        let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        instructions.push(Instruction {
            wire1: parts[0].clone(),
            wire2: parts[2].clone(),
            op: match parts[1].as_str() {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => Op::AND,
            },
            target: parts[4].clone(),
            completed: false,
        });
    });

    // println!("state: {:?}", state);
    // println!("instructions: {:?}", instructions);

    let mut yet_to_finish = true;
    let mut count = 0;
    while yet_to_finish {
        println!("count: {}", count);
        yet_to_finish = false;
        for instruction in instructions.iter_mut() {
            if instruction.completed {
                continue;
            } else {
                yet_to_finish = true;
            }

            // println!("instruction: {:?}", instruction);

            let value1 = match state.get(instruction.wire1.as_str()) {
                Some(v) => *v,
                None => {
                    continue;
                }
            };
            let value2 = match state.get(instruction.wire2.as_str()) {
                Some(v) => *v,
                None => {
                    continue;
                }
            };

            // println!("value1: {}, value2: {}", value1, value2);

            if state.contains_key(&instruction.wire1.clone())
                && state.contains_key(&instruction.wire2.clone())
            {
                // println!("instruction: {:?}", instruction);
                // let value1 = state.get(&instruction.wire1).unwrap();
                // let value2 = state.get(&instruction.wire2).unwrap();
                match instruction.op {
                    Op::XOR => {
                        let result = value1 ^ value2;
                        state.insert(instruction.target.clone(), result);
                    }
                    Op::AND => {
                        let result = value1 & value2;
                        state.insert(instruction.target.clone(), result);
                    }
                    Op::OR => {
                        let result = value1 | value2;
                        state.insert(instruction.target.clone(), result);
                    }
                    _ => {
                        println!("unknown op: {}", instruction.op);
                    }
                }
                instruction.completed = true;
            }
        }
        count += 1;
        // if count > 10 {
        //     break;
        // }
    }

    println!("state: {:?}", state);

    // Get all the wires starting with z00 and up to zXX and convert to an int
    let mut z_values: HashMap<i64, i64> = HashMap::new();
    for (k, v) in state.iter() {
        if k.starts_with("z") {
            println!("{}: {}", k, v);
            let z_value = k[1..].parse::<i64>().unwrap();
            z_values.insert(z_value, *v);
        }
    }
    println!("z_values: {:?}", z_values);

    let mut val: i64 = 0;
    for i in 0..z_values.len() {
        let value = z_values.get(&(i as i64)).unwrap();
        println!("z{}: {}", i, value);
        val += value * 2_i64.pow(i as u32);
    }

    println!("Day 23 Part 1: {}", val);

    println!("Day 23 Part 2: {}", 0);

    Ok(())
}

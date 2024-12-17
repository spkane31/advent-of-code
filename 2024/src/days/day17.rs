use regex::Regex;
use std::fs;
use std::io;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (registers, opcodes) = match read_and_split_file("day17/input.txt") {
        Ok((a, b)) => (a, b),
        Err(e) => return Err(e.into()),
    };

    let (mut a_reg, mut b_reg, mut c_reg) = match extract_register_values(registers.as_str()) {
        Some(a) => a,
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid input").into()),
    };

    let opcodes = match parse_opcodes(opcodes.as_str()) {
        Some(a) => a,
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid input").into()),
    };

    let mut res: Vec<usize> = Vec::new();

    let mut instruction_pointer: usize = 0;
    while instruction_pointer < opcodes.len() - 1 {
        let (opcode, literal) = (
            opcodes[instruction_pointer],
            opcodes[instruction_pointer + 1],
        );

        match opcode {
            0 => {
                // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
                let combo = get_combo(literal, a_reg, b_reg, c_reg);
                a_reg = a_reg / 2usize.pow(combo as u32);
            }
            1 => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                b_reg = b_reg ^ literal;
            }
            2 => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                let combo = get_combo(literal, a_reg, b_reg, c_reg);
                b_reg = combo % 8;
            }
            3 => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if a_reg != 0 {
                    instruction_pointer = literal as usize;
                    continue;
                }
            }
            4 => {
                // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                b_reg = b_reg ^ c_reg;
            }
            5 => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                let c = get_combo(literal, a_reg, b_reg, c_reg);
                res.push(c % 8);
            }
            6 => {
                // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
                let c = get_combo(literal, a_reg, b_reg, c_reg);
                b_reg = a_reg / 2usize.pow(c as u32);
            }
            7 => {
                // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
                let c = get_combo(literal, a_reg, b_reg, c_reg);
                c_reg = a_reg / 2usize.pow(c as u32);
            }
            _ => {
                println!("Unknown opcode: {}", opcode);
            }
        }
        instruction_pointer += 2;
    }

    println!(
        "Day 17 Part 1: {}",
        res.iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    println!(
        "Day 17 Part 2: {}",
        run_on_loop(opcodes, a_reg, b_reg, c_reg)
    );
    Ok(())
}

fn eval(a: usize, b: usize, c: usize, program: Vec<usize>) -> usize {
    let (mut a, mut b, mut c) = (a, b, c);

    let mut ip = 0;

    let mut output = 0;

    loop {
        let (it, op) = (program[ip], program[ip + 1]);

        let combo = |op: usize| match op {
            0..=3 => op,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!(),
        };

        match it {
            0 => a >>= combo(op),
            1 => b ^= op,
            2 => b = combo(op) % 8,
            3 => {
                if a == 0 {
                    return output;
                } else {
                    ip = op as usize;
                }
            }
            4 => b ^= c,
            5 => output = output * 10 + combo(op) % 8,
            6 => b = a >> combo(op),
            7 => c = a >> combo(op),
            _ => unreachable!(),
        }

        if it != 3 {
            ip += 2;
        }
    }
}

use std::mem;

fn run_on_loop(program: Vec<usize>, _a: usize, b: usize, c: usize) -> usize {
    let target = program.iter().fold(0, |acc, digit| acc * 10 + digit);

    let mut a_candidates = vec![0];
    let mut next_a_candidates = Vec::with_capacity(16);

    for digit in 0..=target.ilog10() {
        let look_for = target % 10usize.pow(digit + 1);
        next_a_candidates.clear();
        for initial in a_candidates.iter() {
            let shifted = *initial << 3;
            next_a_candidates.extend(
                (0..8)
                    .map(|offset| shifted + offset)
                    .filter(|&cand| eval(cand, b, c, program.clone()) == look_for),
            );
        }
        mem::swap(&mut a_candidates, &mut next_a_candidates);
    }

    let p2 = a_candidates.into_iter().min().expect("no solution?!");

    p2
}

fn read_and_split_file(file_path: &str) -> io::Result<(String, String)> {
    let content = fs::read_to_string(file_path)?;
    if let Some((part1, part2)) = content.split_once("\n\n") {
        Ok((part1.to_string(), part2.to_string()))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Delimiter not found",
        ))
    }
}
fn extract_register_values(input: &str) -> Option<(usize, usize, usize)> {
    let re = Regex::new(r"Register A: (\d+)\s+Register B: (\d+)\s+Register C: (\d+)").unwrap();
    if let Some(captures) = re.captures(input) {
        let a = captures.get(1)?.as_str().parse::<usize>().ok()?;
        let b = captures.get(2)?.as_str().parse::<usize>().ok()?;
        let c = captures.get(3)?.as_str().parse::<usize>().ok()?;
        Some((a, b, c))
    } else {
        None
    }
}

fn parse_opcodes(input: &str) -> Option<Vec<usize>> {
    let prefix = "Program: ";
    if input.starts_with(prefix) {
        let numbers: Vec<usize> = input[prefix.len()..]
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();

        if numbers.len() % 2 != 0 {
            return None; // Ensure we have an even number of elements
        }

        Some(numbers)
    } else {
        None
    }
}

fn get_combo(n: usize, a: usize, b: usize, c: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => a,
        5 => b,
        6 => c,
        _ => 0,
    }
}

use clap::Parser;
use std::time::Instant;

mod days;

/// This is a simple program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, action)]
    all: bool,

    #[clap(long, short, action)]
    day: Option<u32>,
}

fn main() {
    let args = Args::parse();
    if let Some(day) = args.day {
        let start = Instant::now();
        match day {
            1 => match days::day01::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 1: {:?}", e),
            },
            2 => match days::day02::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 2: {:?}", e),
            },
            3 => match days::day03::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 3: {:?}", e),
            },
            4 => match days::day04::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 4: {:?}", e),
            },
            5 => match days::day05::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 5: {:?}", e),
            },
            6 => match days::day06::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 6: {:?}", e),
            },
            7 => match days::day07::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 7: {:?}", e),
            },
            8 => match days::day08::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 8: {:?}", e),
            },
            9 => match days::day09::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 9: {:?}", e),
            },
            10 => match days::day10::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 10: {:?}", e),
            },
            11 => match days::day11::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 11: {:?}", e),
            },
            12 => match days::day12::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 12: {:?}", e),
            },
            13 => match days::day13::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 13: {:?}", e),
            },
            14 => match days::day14::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 14: {:?}", e),
            },
            15 => match days::day15::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 15: {:?}", e),
            },
            // 16 => match days::day16::run() {
            //     Ok(_) => (),
            //     Err(e) => println!("Error in day 16: {:?}", e),
            // },
            17 => match days::day17::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 17: {:?}", e),
            },
            19 => match days::day19::run() {
                Ok(_) => (),
                Err(e) => println!("Error in day 19: {:?}", e),
            },
            n => println!("Solution for day {} is not implemented yet.", n),
        }
        println!("Total runtime: {:?}", start.elapsed());
    } else if args.all {
        let total = Instant::now();

        let funcs = vec![
            days::day01::run,
            days::day02::run,
            days::day03::run,
            days::day04::run,
            days::day05::run,
            days::day06::run,
            days::day07::run,
            days::day08::run,
            days::day09::run,
            days::day10::run,
            days::day11::run,
            days::day12::run,
            days::day13::run,
            days::day14::run,
            days::day15::run,
            // days::day16::run,
            days::day17::run,
            days::day19::run,
        ];

        for (count, func) in funcs.iter().enumerate() {
            let start = Instant::now();
            match func() {
                Ok(_) => (),
                Err(e) => println!("Error in day {}: {:?}", count + 1, e),
            }
            println!("Day {} runtime: {:?}", count, start.elapsed());
        }
        let elapsed = total.elapsed();
        println!("Total runtime: {:?}", total.elapsed());
        println!("Average runtime: {:?}", elapsed / funcs.len() as u32);
    } else {
        println!("Please specify a day using the --day option.");
    }
}

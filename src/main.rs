mod utils;

use crate::utils::load_input;
use clap::Parser;
use clearscreen::clear;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "NUMBER")]
    day: u8,
}

fn main() {
    let args = Args::parse();

    let _ = clear();
    println!("Advent of Code 2025, Day {}\n", args.day);

    let input = format!("day{}.txt", args.day);
    match load_input(input.as_str()) {
        Ok(lines) => process(args.day, &lines),
        Err(e) => println!("{}", e),
    }
}

fn process(day: u8, lines: &Vec<String>) {
    let mut solution_one = 0;
    let mut solution_two = 0;
    match day {
        1 => (solution_one, solution_two) = day_one(lines),
        _ => {}
    }

    println!("Solution for Day {}, Part 1: {}", day, solution_one);
    println!("Solution for Day {}, Part 2: {}", day, solution_two);
}

fn day_one(lines: &Vec<String>) -> (i32, i32) {
    let mut inputs = Vec::new();
    for line in lines {
        let direction = line.chars().next().unwrap();
        let mut distance: i32 = line[1..].parse().unwrap();
        if direction == 'L' || direction == 'l' {
            distance *= -1;
        }
        inputs.push(distance);
    }

    let mut indicator = 50;
    let mut solution_one = 0;
    let mut solution_two = 0;
    for distance in &inputs {
        solution_two += (distance / 100).abs();
        let remaining = distance % 100;

        let last_indicator = indicator;
        indicator += remaining;
        if indicator < 0 {
            indicator += 100;
            if last_indicator > 0 {
                solution_two += 1;
            }
        } else if indicator > 99 {
            indicator -= 100;
            if indicator != 0 {
                solution_two += 1;
            }
        }

        if indicator == 0 {
            solution_one += 1;
        }
    }

    (solution_one, solution_one + solution_two)
}

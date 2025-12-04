mod utils;
mod day;

use crate::utils::load_input;
use clap::Parser;
use clearscreen::clear;
use day::*;

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
    let mut solution_one: i64 = 0;
    let mut solution_two: i64 = 0;
    match day {
        1 => (solution_one, solution_two) = one::solve(lines),
        2 => (solution_one, solution_two) = two::solve(lines),
        3 => (solution_one, solution_two) = three::solve(lines),
        _ => println!("No solution for day {}\n", day),
    }

    println!("Solution for Day {}, Part 1: {}", day, solution_one);
    println!("Solution for Day {}, Part 2: {}\n", day, solution_two);
}

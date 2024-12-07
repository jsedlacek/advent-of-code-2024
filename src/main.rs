mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::collections::HashMap;

use clap::Parser;

pub trait Puzzle {
    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>>;
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: Option<u64>,

    #[arg(short, long)]
    part: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut days: HashMap<(u64, u64), Box<dyn Puzzle>> = HashMap::new();

    days.insert((1, 1), Box::new(day1::Part1));
    days.insert((1, 2), Box::new(day1::Part2));
    days.insert((2, 1), Box::new(day2::Part1));
    days.insert((2, 2), Box::new(day2::Part2));
    days.insert((3, 1), Box::new(day3::Part1));
    days.insert((3, 2), Box::new(day3::Part2));
    days.insert((4, 1), Box::new(day4::Part1));
    days.insert((4, 2), Box::new(day4::Part2));
    days.insert((5, 1), Box::new(day5::Part1));
    days.insert((5, 2), Box::new(day5::Part2));
    days.insert((6, 1), Box::new(day6::Part1));
    days.insert((6, 2), Box::new(day6::Part2));
    days.insert((7, 1), Box::new(day7::Part1));
    days.insert((7, 2), Box::new(day7::Part2));

    let mut puzzles = days
        .iter()
        .filter(|((day, part), _)| {
            (args.day.is_none() || Some(*day) == args.day)
                && (args.part.is_none() || Some(*part) == args.part)
        })
        .collect::<Vec<_>>();

    puzzles.sort_by_key(|((day, part), _)| (*day, *part));

    if puzzles.is_empty() {
        return Err(format!("Puzzle not found").into());
    }

    for ((day, part), puzzle) in puzzles {
        println!("Day {day} part {part} = {}", puzzle.solve()?);
    }

    Ok(())
}

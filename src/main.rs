mod day1;

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

    let puzzles = days
        .iter()
        .filter(|((day, part), _)| {
            (args.day.is_none() || Some(*day) == args.day)
                && (args.part.is_none() || Some(*part) == args.part)
        })
        .collect::<Vec<_>>();

    if puzzles.is_empty() {
        return Err(format!("Puzzle not found").into());
    }

    for ((day, part), puzzle) in puzzles {
        println!("Day {day} part {part} = {}", puzzle.solve()?);
    }

    Ok(())
}

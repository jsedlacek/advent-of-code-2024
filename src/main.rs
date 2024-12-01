mod day1;

use std::collections::HashMap;

use clap::Parser;

pub trait Puzzle {
    fn solve(&self) -> u64;
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u64,

    #[arg(short, long, default_value_t = 1)]
    part: u64,
}

fn main() {
    let args = Args::parse();

    let mut days: HashMap<(u64, u64), Box<dyn Puzzle>> = HashMap::new();

    days.insert((1, 1), Box::new(day1::Part1));
    days.insert((1, 2), Box::new(day1::Part2));

    let day = days.get(&(args.day, args.part));

    if let Some(day) = day {
        println!("Running day {} part {}", args.day, args.part,);

        println!("Solution: {}", day.solve());
    } else {
        println!("Day {} part {} not implemented", args.day, args.part);
    }
}

mod util;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

use std::{collections::HashMap, time::Instant};

use clap::Parser;

pub trait Puzzle {
    fn solve(&self) -> Result<String, Box<dyn std::error::Error>>;
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

    days.insert((8, 1), Box::new(day8::Part1));
    days.insert((8, 2), Box::new(day8::Part2));

    days.insert((9, 1), Box::new(day9::Part1));
    days.insert((9, 2), Box::new(day9::Part2));

    days.insert((10, 1), Box::new(day10::Part1));
    days.insert((10, 2), Box::new(day10::Part2));

    days.insert((11, 1), Box::new(day11::Part1));
    days.insert((11, 2), Box::new(day11::Part2));

    days.insert((12, 1), Box::new(day12::Part1));
    days.insert((12, 2), Box::new(day12::Part2));

    days.insert((13, 1), Box::new(day13::Part1));
    days.insert((13, 2), Box::new(day13::Part2));

    days.insert((14, 1), Box::new(day14::Part1));
    days.insert((14, 2), Box::new(day14::Part2));

    days.insert((15, 1), Box::new(day15::Part1));
    days.insert((15, 2), Box::new(day15::Part2));

    days.insert((16, 1), Box::new(day16::Part1));
    days.insert((16, 2), Box::new(day16::Part2));

    days.insert((17, 1), Box::new(day17::Part1));
    days.insert((17, 2), Box::new(day17::Part2));

    days.insert((18, 1), Box::new(day18::Part1));
    days.insert((18, 2), Box::new(day18::Part2));

    days.insert((19, 1), Box::new(day19::Part1));
    days.insert((19, 2), Box::new(day19::Part2));

    days.insert((20, 1), Box::new(day20::Part1));
    days.insert((20, 2), Box::new(day20::Part2));

    days.insert((21, 1), Box::new(day21::Part1));
    days.insert((21, 2), Box::new(day21::Part2));

    days.insert((22, 1), Box::new(day22::Part1));
    days.insert((22, 2), Box::new(day22::Part2));

    days.insert((23, 1), Box::new(day23::Part1));
    days.insert((23, 2), Box::new(day23::Part2));

    days.insert((24, 1), Box::new(day24::Part1));
    days.insert((24, 2), Box::new(day24::Part2));

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
        let now = Instant::now();
        let result = puzzle.solve()?;
        let elapsed = now.elapsed();

        print!("Day {day} part {part} = {}", result);

        if elapsed.as_millis() > 1 {
            let ms = elapsed.as_millis();
            print!(" [{ms}ms]");
        }

        println!();
    }

    Ok(())
}

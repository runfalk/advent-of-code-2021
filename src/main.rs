use anyhow::{anyhow, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    /// The day to run the solution for (1-25)
    day: usize,

    /// The input data file. Will look for `data/day<num>.txt` by default
    input: Option<PathBuf>,
}

fn pad_newlines(answer: String) -> String {
    answer.lines().collect::<Vec<_>>().join("\n   ")
}

fn as_result<A: ToString, B: ToString>((a, b): (A, Option<B>)) -> (String, Option<String>) {
    (a.to_string(), b.map(|answer| answer.to_string()))
}

fn main() -> Result<()> {
    let opts = Options::from_args();
    let input = opts
        .input
        .unwrap_or_else(|| format!("data/day{}.txt", opts.day).into());

    #[allow(
        overlapping_range_endpoints,
        unreachable_patterns,
        clippy::match_overlapping_arm
    )]
    let (a, b): (String, Option<String>) = match opts.day {
        1 => as_result(advent_of_code_2021::day1::main(&input)?),
        2 => as_result(advent_of_code_2021::day2::main(&input)?),
        3 => as_result(advent_of_code_2021::day3::main(&input)?),
        5 => as_result(advent_of_code_2021::day5::main(&input)?),
        6 => as_result(advent_of_code_2021::day6::main(&input)?),
        7 => as_result(advent_of_code_2021::day7::main(&input)?),
        8 => as_result(advent_of_code_2021::day8::main(&input)?),
        9 => as_result(advent_of_code_2021::day9::main(&input)?),
        10 => as_result(advent_of_code_2021::day10::main(&input)?),
        11 => as_result(advent_of_code_2021::day11::main(&input)?),
        12 => as_result(advent_of_code_2021::day12::main(&input)?),
        13 => as_result(advent_of_code_2021::day13::main(&input)?),
        14 => as_result(advent_of_code_2021::day14::main(&input)?),
        15 => as_result(advent_of_code_2021::day15::main(&input)?),
        16 => as_result(advent_of_code_2021::day16::main(&input)?),
        17 => as_result(advent_of_code_2021::day17::main(&input)?),
        18 => as_result(advent_of_code_2021::day18::main(&input)?),
        19 => as_result(advent_of_code_2021::day19::main(&input)?),
        20 => as_result(advent_of_code_2021::day20::main(&input)?),
        21 => as_result(advent_of_code_2021::day21::main(&input)?),
        22 => as_result(advent_of_code_2021::day22::main(&input)?),
        23 => as_result(advent_of_code_2021::day23::main(&input)?),
        1..=25 => return Err(anyhow!("No implementation for this day yet")),
        day => return Err(anyhow!("Day {} is not a valid day for advent of code", day)),
    };

    println!("A: {}", pad_newlines(a));
    if let Some(b) = b {
        println!("B: {}", pad_newlines(b));
    }

    Ok(())
}

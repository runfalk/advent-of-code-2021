use advent_of_code_2021::day1;
use advent_of_code_2021::day2;
use anyhow::{anyhow, Result};
use std::path::Path;

fn pad_newlines(answer: String) -> String {
    answer.lines().collect::<Vec<_>>().join("\n   ")
}

fn as_result<A: ToString, B: ToString>(value: (A, Option<B>)) -> (String, Option<String>) {
    (
        value.0.to_string(),
        value.1.map(|answer| answer.to_string()),
    )
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return Err(anyhow!("Not enough arguments"));
    }

    let path: Option<&Path> = if args.len() == 3 {
        Some(Path::new(&args[2]))
    } else {
        None
    };

    #[allow(overlapping_range_endpoints, unreachable_patterns)]
    let result: (String, Option<String>) = match args[1].parse() {
        Ok(1) => as_result(day1::main(
            path.unwrap_or_else(|| Path::new("data/day1.txt")),
        )?),
        Ok(2) => as_result(day2::main(
            path.unwrap_or_else(|| Path::new("data/day2.txt")),
        )?),
        Ok(1..=25) => return Err(anyhow!("No implementation for this day yet")),
        Ok(day) => return Err(anyhow!("Day {} is not a valid day for advent of code", day)),
        Err(_) => return Err(anyhow!("{:?} is not a valid day", args[1])),
    };

    println!("A: {}", pad_newlines(result.0));
    if let Some(b) = result.1 {
        println!("B: {}", pad_newlines(b));
    }

    Ok(())
}

use anyhow::{anyhow, Error, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub enum Direction {
    Forward(isize),
    Up(isize),
    Down(isize),
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        let direction = parts.next().unwrap(); // Unwrap is fine since it can't fail here
        let value: isize = parts
            .next()
            .ok_or_else(|| anyhow!("No value found"))?
            .parse()?;

        Ok(match direction {
            "forward" => Self::Forward(value),
            "up" => Self::Up(value),
            "down" => Self::Down(value),
            d => return Err(anyhow!("Unknown direction {}", d)),
        })
    }
}

pub fn part_a(directions: &[Direction]) -> isize {
    let mut hpos = 0;
    let mut depth = 0;

    for d in directions {
        match d {
            Direction::Forward(d) => hpos += d,
            Direction::Up(d) => depth -= d,
            Direction::Down(d) => depth += d,
        }
    }
    hpos * depth
}

pub fn part_b(directions: &[Direction]) -> isize {
    let mut aim = 0;
    let mut hpos = 0;
    let mut depth = 0;

    for d in directions {
        match d {
            Direction::Forward(d) => {
                hpos += d;
                depth += aim * d;
            }
            Direction::Up(d) => aim -= d,
            Direction::Down(d) => aim += d,
        }
    }
    hpos * depth
}

pub fn main(path: &Path) -> Result<(isize, Option<isize>)> {
    let file = File::open(path)?;
    let directions = io::BufReader::new(file)
        .lines()
        .map(|lr| lr?.parse::<Direction>())
        .collect::<Result<Vec<Direction>>>()?;
    Ok((part_a(&directions), Some(part_b(&directions))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIRECTIONS: &'static [Direction] = &[
        Direction::Forward(5),
        Direction::Down(5),
        Direction::Forward(8),
        Direction::Up(3),
        Direction::Down(8),
        Direction::Forward(2),
    ];

    #[test]
    fn test_part_a() -> Result<()> {
        assert_eq!(part_a(&DIRECTIONS), 150);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        assert_eq!(part_b(&DIRECTIONS), 900);
        Ok(())
    }
}

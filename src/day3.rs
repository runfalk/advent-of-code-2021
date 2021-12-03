use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_a<R: AsRef<str>>(report: &[R]) -> Result<usize> {
    let mut ones = Vec::new();
    for line in report {
        let line = line.as_ref();
        ones.resize(line.len(), 0);
        for (i, c) in line.chars().rev().enumerate() {
            match c {
                '0' => (),
                '1' => ones[i] += 1,
                _ => return Err(anyhow!("NO")),
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, num_ones) in ones.into_iter().enumerate() {
        if num_ones > report.len() / 2 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    Ok(gamma * epsilon)
}

fn count_ones<R: AsRef<str>>(report: impl Iterator<Item = R>) -> Result<Vec<usize>> {
    let mut iter = report.peekable();

    let num_digits = match iter.peek() {
        Some(line) => line.as_ref().len(),
        None => return Ok(Vec::new()),
    };

    let mut ones = Vec::new();
    ones.resize(num_digits, 0usize);

    for line in iter {
        let line = line.as_ref();
        for (num_ones, c) in ones.iter_mut().zip(line.chars()) {
            match c {
                '0' => (),
                '1' => *num_ones += 1,
                _ => return Err(anyhow!("NO")),
            }
        }
    }

    Ok(ones)
}

fn part_b<R: AsRef<str>>(report: &[R]) -> Result<usize> {
    let mut oxygen_generators: HashSet<_> = report.iter().map(AsRef::as_ref).collect();
    let mut co2_scrubbers: HashSet<_> = oxygen_generators.clone();

    let mut i = 0;
    while oxygen_generators.len() > 1 {
        let ones = count_ones(oxygen_generators.iter())?;
        let most_common = if ones[i] >= oxygen_generators.len() - ones[i] {
            '1'
        } else {
            '0'
        };
        oxygen_generators.retain(|line| line.chars().nth(i).unwrap() == most_common);
        i += 1;
    }

    let mut i = 0;
    while co2_scrubbers.len() > 1 {
        let ones = count_ones(co2_scrubbers.iter())?;
        let most_common = if ones[i] >= co2_scrubbers.len() - ones[i] {
            '1'
        } else {
            '0'
        };
        co2_scrubbers.retain(|line| line.chars().nth(i).unwrap() != most_common);
        i += 1;
    }

    let oxygen_generator_rating =
        usize::from_str_radix(oxygen_generators.into_iter().next().unwrap(), 2)?;
    let co2_scrubber_rating = usize::from_str_radix(co2_scrubbers.into_iter().next().unwrap(), 2)?;

    Ok(oxygen_generator_rating * co2_scrubber_rating)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let report = io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;
    Ok((part_a(&report)?, Some(part_b(&report)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const REPORT: &'static [&str] = &[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_part_a() -> Result<()> {
        assert_eq!(part_a(&REPORT)?, 198);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        assert_eq!(part_b(&REPORT)?, 230);
        Ok(())
    }
}

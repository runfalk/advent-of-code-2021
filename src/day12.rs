use anyhow::{anyhow, Error as AnyhowError, Result};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

impl FromStr for Cave {
    type Err = AnyhowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            c if c.chars().all(char::is_uppercase) => Ok(Self::Large(c.to_string())),
            c if c.chars().all(char::is_lowercase) => Ok(Self::Small(c.to_string())),
            o => Err(anyhow!("Invalid cave {:?}", o)),
        }
    }
}

fn num_paths<T: Clone + FnMut(&Cave) -> bool>(
    connections: &HashMap<Cave, HashSet<Cave>>,
    try_visit: T,
    start: &Cave,
) -> usize {
    if start == &Cave::End {
        return 1;
    }

    connections[start]
        .iter()
        .zip(std::iter::repeat(try_visit))
        .filter_map(|(next_cave, mut try_visit)| {
            if try_visit(next_cave) {
                Some(num_paths(connections, try_visit, next_cave))
            } else {
                None
            }
        })
        .sum()
}

fn part_a(connections: &HashMap<Cave, HashSet<Cave>>) -> usize {
    let mut visited = HashSet::new();
    visited.insert(Cave::Start);
    let tracker = move |cave: &Cave| matches!(cave, Cave::Large(_)) || visited.insert(cave.clone());
    num_paths(connections, tracker, &Cave::Start)
}

fn part_b(connections: &HashMap<Cave, HashSet<Cave>>) -> usize {
    let mut second_visit = false;
    let mut visited = HashSet::new();
    visited.insert(Cave::Start);
    let tracker = move |cave: &Cave| {
        if matches!(cave, Cave::Large(_)) || visited.insert(cave.clone()) {
            return true;
        }

        if cave == &Cave::Start || second_visit {
            return false;
        }
        second_visit = true;
        true
    };
    num_paths(connections, tracker, &Cave::Start)
}

fn parse_connections<S: AsRef<str>>(lines: &[S]) -> Result<HashMap<Cave, HashSet<Cave>>> {
    lines.iter().try_fold(
        HashMap::new(),
        |mut connections, line| -> Result<HashMap<Cave, HashSet<Cave>>> {
            let (a, b): (Cave, Cave) = line
                .as_ref()
                .split_once("-")
                .ok_or_else(|| anyhow!("{:?} is not a valid cave connection", line.as_ref()))
                .and_then(|(a, b)| Ok((a.parse()?, b.parse()?)))?;
            connections
                .entry(a.clone())
                .or_insert_with(HashSet::new)
                .insert(b.clone());
            connections.entry(b).or_insert_with(HashSet::new).insert(a);
            Ok(connections)
        },
    )
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let lines = io::BufReader::new(File::open(path)?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let paths = parse_connections(&lines)?;
    Ok((part_a(&paths), Some(part_b(&paths))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static [&str] =
        &["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];

    const EXAMPLE2: &'static [&str] = &[
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];

    #[test]
    fn test_part_a() -> Result<()> {
        assert_eq!(part_a(&parse_connections(EXAMPLE1)?), 10);
        assert_eq!(part_a(&parse_connections(EXAMPLE2)?), 226);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        assert_eq!(part_b(&parse_connections(EXAMPLE1)?), 36);
        assert_eq!(part_b(&parse_connections(EXAMPLE2)?), 3509);
        Ok(())
    }
}

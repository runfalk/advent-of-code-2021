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

impl Cave {
    fn is_large(&self) -> bool {
        matches!(self, Self::Large(_))
    }
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

trait VisitTracker: Clone {
    fn visit(&mut self, cave: &Cave);
    fn can_visit(&self, cave: &Cave) -> bool;
}

#[derive(Debug, Clone, Default)]
struct VisitSmallCavesOnce {
    visited: HashSet<Cave>,
}

#[derive(Debug, Clone, Default)]
struct VisitOneSmallCaveTwice {
    visited: HashSet<Cave>,
    second_visit: bool,
}

impl VisitTracker for VisitSmallCavesOnce {
    fn visit(&mut self, cave: &Cave) {
        if !self.visited.insert(cave.clone()) && !cave.is_large() {
            // We panic here since it's an implementation error rather than something that can be
            // triggered by user input
            panic!("Tried to visit cave {:?} twice", cave);
        }
    }

    fn can_visit(&self, cave: &Cave) -> bool {
        cave != &Cave::Start && (cave.is_large() || !self.visited.contains(cave))
    }
}

impl VisitTracker for VisitOneSmallCaveTwice {
    fn visit(&mut self, cave: &Cave) {
        if cave.is_large() || self.visited.insert(cave.clone()) {
            return;
        }

        if cave == &Cave::Start || self.second_visit {
            // We panic here since it's an implementation error rather than something that can be
            // triggered by user input
            panic!("Tried to visit cave {:?} twice", cave);
        }

        self.second_visit = true;
    }

    fn can_visit(&self, cave: &Cave) -> bool {
        cave != &Cave::Start
            && (cave.is_large() || !self.visited.contains(cave) || !self.second_visit)
    }
}

fn num_paths<T: VisitTracker>(
    paths: &HashMap<Cave, HashSet<Cave>>,
    mut visit_tracker: T,
    start: &Cave,
) -> usize {
    if start == &Cave::End {
        return 1;
    }
    visit_tracker.visit(start);

    let mut count = 0;
    for next_cave in &paths[start] {
        if !visit_tracker.can_visit(next_cave) {
            continue;
        }
        count += num_paths(paths, visit_tracker.clone(), next_cave);
    }
    count
}

fn part_a(paths: &HashMap<Cave, HashSet<Cave>>) -> usize {
    num_paths(paths, VisitSmallCavesOnce::default(), &Cave::Start)
}

fn part_b(paths: &HashMap<Cave, HashSet<Cave>>) -> usize {
    num_paths(paths, VisitOneSmallCaveTwice::default(), &Cave::Start)
}

fn parse_paths<S: AsRef<str>>(lines: &[S]) -> Result<HashMap<Cave, HashSet<Cave>>> {
    lines
        .iter()
        .map(|line| -> Result<(Cave, Cave)> {
            let (a, b) = line.as_ref().split_once("-").ok_or_else(|| {
                anyhow!(
                    "Line {:?} doesn't seem to be a cave connection",
                    line.as_ref()
                )
            })?;
            Ok((a.parse()?, b.parse()?))
        })
        .try_fold(HashMap::new(), |mut paths, cave_res| -> Result<_> {
            let (a, b) = cave_res?;
            paths
                .entry(a.clone())
                .or_insert_with(HashSet::new)
                .insert(b.clone());
            paths.entry(b).or_insert_with(HashSet::new).insert(a);
            Ok(paths)
        })
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    let paths = parse_paths(&lines)?;

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
        assert_eq!(part_a(&parse_paths(EXAMPLE1)?), 10);
        assert_eq!(part_a(&parse_paths(EXAMPLE2)?), 226);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        assert_eq!(part_b(&parse_paths(EXAMPLE1)?), 36);
        assert_eq!(part_b(&parse_paths(EXAMPLE2)?), 3509);
        Ok(())
    }
}

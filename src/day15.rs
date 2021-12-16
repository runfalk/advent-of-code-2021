use anyhow::{anyhow, Result};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn iter_neighbors(&self) -> impl Iterator<Item = Self> {
        [
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x - 1, self.y),
        ]
        .into_iter()
    }
}

fn lowest_risk(
    map: &HashMap<Coordinate, usize>,
    start: Coordinate,
    end: Coordinate,
) -> Option<usize> {
    if !map.contains_key(&start) {
        return None;
    }
    let mut lowest_risk = HashMap::new();
    lowest_risk.insert(start, 0usize);

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse((0, start)));

    while let Some(Reverse((risk, cell))) = to_visit.pop() {
        for (neighbor, neighbor_risk) in cell
            .iter_neighbors()
            .filter_map(|n| map.get(&n).map(|r| (n, r + risk)))
        {
            if let Some(curr_lowest_risk) = lowest_risk.get_mut(&neighbor) {
                if *curr_lowest_risk <= neighbor_risk {
                    continue;
                }
                *curr_lowest_risk = neighbor_risk;
            } else {
                lowest_risk.insert(neighbor, neighbor_risk);
            }
            to_visit.push(Reverse((neighbor_risk, neighbor)));
        }
    }

    lowest_risk.get(&end).copied()
}

fn enlarge_map(map: &HashMap<Coordinate, usize>, factor: isize) -> HashMap<Coordinate, usize> {
    if map.is_empty() {
        return map.clone();
    }

    let mut new_map = HashMap::new();
    let width = map.keys().map(|c| c.x).max().unwrap() + 1;
    let height = map.keys().map(|c| c.y).max().unwrap() + 1;

    for dy in 0..factor {
        for dx in 0..factor {
            for (c, risk) in map.iter() {
                new_map.insert(
                    Coordinate::new(c.x + width * dx, c.y + height * dy),
                    (risk + dx as usize + dy as usize - 1) % 9 + 1,
                );
            }
        }
    }
    new_map
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut map: HashMap<Coordinate, usize> = HashMap::new();
    for (y, line) in io::BufReader::new(File::open(path)?).lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            map.insert(
                Coordinate::new(x.try_into()?, y.try_into()?),
                c.to_digit(10)
                    .ok_or_else(|| anyhow!("Invalid digit {:?}", c))?
                    .try_into()?,
            );
        }
    }
    let large_map = enlarge_map(&map, 5);

    let end = Coordinate::new(
        map.keys().map(|c| c.x).max().unwrap(),
        map.keys().map(|c| c.y).max().unwrap(),
    );
    let a = lowest_risk(&map, Coordinate::new(0, 0), end).unwrap();

    let end = Coordinate::new(
        large_map.keys().map(|c| c.x).max().unwrap(),
        large_map.keys().map(|c| c.y).max().unwrap(),
    );
    let b = lowest_risk(&large_map, Coordinate::new(0, 0), end).unwrap();

    Ok((a, Some(b)))
}

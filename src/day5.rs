use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vent {
    start: (isize, isize),
    end: (isize, isize),
}

impl Vent {
    fn iter_coords(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        let dx = (self.end.0 - self.start.0).signum();
        let dy = (self.end.1 - self.start.1).signum();
        (0..)
            .map(move |i| (self.start.0 + dx * i, self.start.1 + dy * i))
            .take_while(move |&(x, y)| (x, y) != (self.end.0 + dx, self.end.1 + dy))
    }
}

impl FromStr for Vent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("No delimiter found for vent"))?;

        let (start_x, start_y) = start
            .split_once(',')
            .ok_or_else(|| anyhow!("Invalid vent start"))?;
        let (end_x, end_y) = end
            .split_once(',')
            .ok_or_else(|| anyhow!("Invalid vent end"))?;

        Ok(Vent {
            start: (start_x.parse()?, start_y.parse()?),
            end: (end_x.parse()?, end_y.parse()?),
        })
    }
}

pub fn part_a(vents: &[Vent]) -> usize {
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();
    for v in vents {
        if v.start.0 != v.end.0 && v.start.1 != v.end.1 {
            continue;
        }
        for (x, y) in v.iter_coords() {
            *map.entry((x, y)).or_default() += 1;
        }
    }
    map.into_values().filter(|count| *count >= 2).count()
}

pub fn part_b(vents: &[Vent]) -> usize {
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();
    for v in vents {
        for (x, y) in v.iter_coords() {
            *map.entry((x, y)).or_default() += 1;
        }
    }
    map.into_values().filter(|count| *count >= 2).count()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let vents = io::BufReader::new(file)
        .lines()
        .map(|lr| lr?.parse::<Vent>())
        .collect::<Result<Vec<Vent>>>()?;
    Ok((part_a(&vents), Some(part_b(&vents))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const VENTS: &'static [&str] = &[
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn test_from_str() -> Result<()> {
        assert_eq!(
            "1,2 -> 3,4".parse::<Vent>()?,
            Vent {
                start: (1, 2),
                end: (3, 4)
            }
        );
        Ok(())
    }

    #[test]
    fn test_part_a() -> Result<()> {
        let vents = VENTS
            .iter()
            .map(|l| l.parse())
            .collect::<Result<Vec<Vent>, _>>()?;
        assert_eq!(part_a(&vents), 5);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        let vents = VENTS
            .iter()
            .map(|l| l.parse())
            .collect::<Result<Vec<Vent>, _>>()?;
        assert_eq!(part_b(&vents), 12);
        Ok(())
    }
}

use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn part_ab(heightmap: &HashMap<Coordinate, usize>) -> (usize, usize) {
    // Find the lowest point in every pool and calculate the total risk
    let mut low_points = Vec::new();
    let mut risk = 0;
    for (&c, v) in heightmap.iter() {
        if c.iter_neighbors()
            .filter_map(|n| heightmap.get(&n))
            .all(|n| v < n)
        {
            risk += v + 1;
            low_points.push(c);
        }
    }

    // Use breadth first flood fill to find the size of all pools
    let mut pool_sizes = Vec::new();
    for low_point in low_points {
        let mut queue = VecDeque::new();
        queue.push_back(low_point);

        let mut visited = HashSet::new();
        visited.insert(low_point);

        while let Some(c) = queue.pop_front() {
            for n in c.iter_neighbors() {
                // Ignore explored coordinates and points with height 9
                if visited.contains(&n) || heightmap.get(&n).filter(|&nv| *nv < 9).is_none() {
                    continue;
                }
                queue.push_back(n);
                visited.insert(n);
            }
        }
        pool_sizes.push(visited.len());
    }
    pool_sizes.sort_unstable();

    (
        risk,
        pool_sizes.into_iter().rev().take(3).product::<usize>(),
    )
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut heightmap: HashMap<_, usize> = HashMap::new();

    let file = File::open(path)?;
    for (y, line) in io::BufReader::new(file).lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            heightmap.insert(
                Coordinate::new(x.try_into()?, y.try_into()?),
                c.to_digit(10)
                    .ok_or_else(|| anyhow!("{} is not a digit", c))?
                    .try_into()?,
            );
        }
    }

    let (a, b) = part_ab(&heightmap);
    Ok((a, Some(b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let map = [
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        let heightmap = map
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(x, v)| (Coordinate::new(x as isize, y as isize), v))
            })
            .collect();
        assert_eq!(part_ab(&heightmap), (15, 1134));

        Ok(())
    }
}

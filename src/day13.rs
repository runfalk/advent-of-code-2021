use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Fold {
    X(isize),
    Y(isize),
}

pub fn main(path: &Path) -> Result<(usize, Option<String>)> {
    let input = std::fs::read_to_string(path)?;
    let (points_str, fold_str) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Unable to find folds, there should be a blank line in there"))?;

    let mut points = points_str
        .lines()
        .map(|l| {
            let (x, y) = l
                .split_once(',')
                .ok_or_else(|| anyhow!("No comma found for point"))?;
            Ok((x.parse()?, y.parse()?))
        })
        .collect::<Result<HashSet<(isize, isize)>>>()?;

    let folds = fold_str
        .lines()
        .map(|l| {
            let (prefix, pos) = l
                .split_once('=')
                .ok_or_else(|| anyhow!("No equal sign found for fold instruction ({:?})", l))?;
            match prefix {
                "fold along x" => Ok(Fold::X(pos.parse()?)),
                "fold along y" => Ok(Fold::Y(pos.parse()?)),
                _ => Err(anyhow!("Invalid fold specification ({:?})", prefix)),
            }
        })
        .collect::<Result<Vec<Fold>>>()?;

    let mut a = None;
    for fold in folds {
        points = match fold {
            Fold::X(fx) => points
                .into_iter()
                .map(|(x, y)| {
                    let x = if x <= fx { x } else { 2 * fx - x };
                    (x, y)
                })
                .collect::<HashSet<(isize, isize)>>(),
            Fold::Y(fy) => points
                .into_iter()
                .map(|(x, y)| {
                    let y = if y <= fy { y } else { 2 * fy - y };
                    (x, y)
                })
                .collect::<HashSet<(isize, isize)>>(),
        };

        if a.is_none() {
            a = Some(points.len());
        }
    }

    let min_x = points.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = points.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let min_y = points.iter().map(|(_, y)| *y).min().unwrap_or(0);
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap_or(0);

    let mut b = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            b.push(if points.contains(&(x, y)) { '#' } else { ' ' });
        }
        b.push('\n');
    }

    Ok((a.unwrap(), Some(b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let mut input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        input.sort_unstable();
        assert_eq!(part_a(&input), 37);
        assert_eq!(part_b(&input), 168);
        Ok(())
    }
}

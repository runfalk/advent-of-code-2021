use anyhow::{anyhow, Result};
use regex::Regex;
use std::ops::RangeInclusive;
use std::path::Path;

fn iter_x(mut acc: isize) -> impl Iterator<Item = isize> {
    let mut pos = 0isize;
    std::iter::repeat_with(move || {
        pos += acc;
        if acc != 0 {
            acc -= 1;
        }
        pos
    })
}

fn iter_y(mut acc: isize, min_y: isize) -> impl Iterator<Item = isize> {
    let mut pos = 0isize;
    std::iter::repeat_with(move || {
        pos += acc;
        acc -= 1;
        pos
    })
    .take_while(move |y| *y >= min_y)
}

// This doesn't generalize to targets above Y: 0
fn part_a(min_y: isize) -> isize {
    // We need to remove one from the minimum Y since the acceleration will increase by one due to
    // gravity when the probe passes 0 on the way down
    let acc = min_y.abs() - 1;
    iter_y(acc, min_y)
        .zip(iter_y(acc, min_y).skip(1))
        .map_while(|(c, n)| if c <= n { Some(c) } else { None })
        .last()
        .unwrap_or(0)
}

// This doesn't generalize to targets above Y: 0 or X < 0
fn part_b(target_x: &RangeInclusive<isize>, target_y: &RangeInclusive<isize>) -> usize {
    (*target_y.start()..=-*target_y.start())
        .flat_map(|acc_y| (0..=*target_x.end()).map(move |acc_x| (acc_x, acc_y)))
        .filter(|&(acc_x, acc_y)| {
            iter_x(acc_x)
                .zip(iter_y(acc_y, *target_y.start()))
                .any(|(x, y)| target_x.contains(&x) && target_y.contains(&y))
        })
        .count()
}

pub fn main(path: &Path) -> Result<(isize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let re = Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    let captures = re
        .captures(input.trim_end())
        .ok_or_else(|| anyhow!("Invalid input"))?;

    let target_x =
        captures.get(1).unwrap().as_str().parse()?..=captures.get(2).unwrap().as_str().parse()?;
    let target_y =
        captures.get(3).unwrap().as_str().parse()?..=captures.get(4).unwrap().as_str().parse()?;

    Ok((
        part_a(*target_y.start()),
        Some(part_b(&target_x, &target_y)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() -> Result<()> {
        assert_eq!(part_a(-10), 45);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        let target_x = 20..=30isize;
        let target_y = -10..=-5isize;
        assert_eq!(part_b(&target_x, &target_y), 112);
        Ok(())
    }
}

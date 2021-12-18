use anyhow::{anyhow, Result};
use regex::Regex;
use std::ops::RangeInclusive;
use std::path::Path;

fn iter_x(mut acc: isize) -> impl Clone + Iterator<Item = isize> {
    let mut pos = 0isize;
    std::iter::repeat_with(move || {
        pos += acc;
        if acc != 0 {
            acc -= 1;
        }
        pos
    })
}

fn iter_x_terminated(acc: isize) -> impl Iterator<Item = isize> {
    // This awkwardness exists because we don't have a .take_while_inclusive()
    let mut terminate_next = false;
    iter_x(acc)
        .zip(iter_x(acc).skip(1))
        .take_while(move |(c, n)| {
            let should_continue = !terminate_next;
            if c == n {
                terminate_next = true;
            }
            should_continue
        })
        .map(|(x, _)| x)
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
    let mut x_acc_candidates = Vec::new();
    for acc in 0..=*target_x.end() {
        if iter_x_terminated(acc).any(|x| target_x.contains(&x)) {
            x_acc_candidates.push(acc);
        }
    }

    let mut y_acc_candidates = Vec::new();
    for acc in *target_y.start()..=-*target_y.start() {
        if iter_y(acc, *target_y.start()).any(|y| target_y.contains(&y)) {
            y_acc_candidates.push(acc);
        }
    }

    let mut num_parabolas = 0;
    for acc_y in y_acc_candidates {
        for acc_x in x_acc_candidates.iter().copied() {
            if iter_x(acc_x)
                .zip(iter_y(acc_y, *target_y.start()))
                .any(|(x, y)| target_x.contains(&x) && target_y.contains(&y))
            {
                num_parabolas += 1;
            }
        }
    }
    num_parabolas
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

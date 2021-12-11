use anyhow::{anyhow, Result};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn tick<const W: usize, const H: usize>(grid: &mut [[u8; W]; H]) -> usize {
    // Increment all squid timers by one
    grid.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|s| *s += 1));

    // Detect all squids that are about to flash
    let mut will_flash: VecDeque<_> = (0..H)
        .flat_map(|y| (0..W).map(move |x| (x, y)))
        .filter(|&(x, y)| grid[y][x] == 10)
        .collect();

    // While there are still squids to flash, do so
    let mut num_flashes = 0;
    while let Some((x, y)) = will_flash.pop_front() {
        // Iterate all neighboring locations
        let neighbors = [
            (Some(x), y.checked_sub(1)),
            (Some(x + 1), y.checked_sub(1)),
            (Some(x + 1), Some(y)),
            (Some(x + 1), Some(y + 1)),
            (Some(x), Some(y + 1)),
            (x.checked_sub(1), Some(y + 1)),
            (x.checked_sub(1), Some(y)),
            (x.checked_sub(1), y.checked_sub(1)),
        ];

        for n in neighbors {
            let (nx, ny, nv) = match n {
                (Some(x), Some(y)) => {
                    if let Some(v) = grid.get_mut(y).and_then(|row| row.get_mut(x)) {
                        (x, y, v)
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };
            *nv += 1;
            if *nv == 10 {
                will_flash.push_back((nx, ny));
            }
        }

        num_flashes += 1;
    }

    // When all reactions are complete we have to reset all the squids who flashed
    grid.iter_mut()
        .for_each(|row| row.iter_mut().filter(|s| **s > 9).for_each(|s| *s = 0));

    num_flashes
}

fn part_a<const W: usize, const H: usize>(mut grid: [[u8; W]; H]) -> usize {
    let mut num_flashes = 0;
    for _ in 0..100 {
        num_flashes += tick(&mut grid);
    }
    num_flashes
}

fn part_b<const W: usize, const H: usize>(mut grid: [[u8; W]; H]) -> usize {
    let mut num_steps = 0;
    loop {
        num_steps += 1;
        if tick(&mut grid) == W * H {
            break num_steps;
        }
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    // This will panic on invalid data. Would be nice to fail more gracefully
    let file = File::open(path)?;
    let mut grid: [[u8; 10]; 10] = Default::default();
    for (y, line) in io::BufReader::new(file).lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            grid[y][x] = c
                .to_digit(10)
                .ok_or_else(|| anyhow!("{} is not a digit", c))?
                .try_into()?;
        }
    }

    Ok((part_a(grid), Some(part_b(grid))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRID: [[u8; 10]; 10] = [
        [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];

    #[test]
    fn test_part_a() -> Result<()> {
        assert_eq!(part_a(GRID), 1656);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        assert_eq!(part_b(GRID), 195);
        Ok(())
    }
}

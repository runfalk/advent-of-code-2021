use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part_a(depths: &[usize]) -> usize {
    depths
        .iter()
        .copied()
        .skip(1)
        .zip(depths.iter().copied())
        .filter(|(c, p)| c > p)
        .count()
}

pub fn part_b(depths: &[usize]) -> usize {
    let windows = depths.windows(3);
    windows
        .clone()
        .skip(1)
        .zip(windows)
        .filter(|(c, p)| c.iter().sum::<usize>() > p.iter().sum::<usize>())
        .count()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let depths = io::BufReader::new(file)
        .lines()
        .map(|lr| Ok(lr?.parse::<usize>()?))
        .collect::<Result<Vec<usize>>>()?;
    Ok((part_a(&depths), Some(part_b(&depths))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() -> Result<()> {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part_a(&depths), 7);
        assert_eq!(part_b(&depths), 5);
        Ok(())
    }
}

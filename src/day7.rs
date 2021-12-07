use anyhow::Result;
use std::path::Path;

fn part_a(sorted_crabs: &[isize]) -> isize {
    let first = sorted_crabs[0];
    let last = sorted_crabs[sorted_crabs.len() - 1];

    (first..=last)
        .map(|target| {
            sorted_crabs
                .iter()
                .map(|crab| (crab - target).abs())
                .sum::<isize>()
        })
        .min()
        .unwrap_or(0)
}

fn part_b(sorted_crabs: &[isize]) -> isize {
    let first = sorted_crabs[0];
    let last = sorted_crabs[sorted_crabs.len() - 1];

    (first..=last)
        .map(|target| {
            sorted_crabs
                .iter()
                .map(|crab| (0..=(crab - target).abs()).sum::<isize>())
                .sum::<isize>()
        })
        .min()
        .unwrap_or(0)
}

pub fn main(path: &Path) -> Result<(isize, Option<isize>)> {
    let input = std::fs::read_to_string(path)?;
    let mut crabs = input
        .trim()
        .split(',')
        .map(|d| d.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()?;

    crabs.sort_unstable();

    Ok((part_a(&crabs), Some(part_b(&crabs))))
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

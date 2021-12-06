use anyhow::{anyhow, Result};
use std::path::Path;

type State = [usize; 9];

pub fn simulation(mut state: State, num_iterations: usize) -> usize {
    for _ in 0..num_iterations {
        let num_births = state[0];
        for i in 1..state.len() {
            state[i - 1] = state[i];
        }
        state[6] += num_births;
        state[8] = num_births;
    }
    state.into_iter().sum()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let timers = input
        .trim()
        .split(',')
        .map(|d| d.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut initial_state: State = Default::default();
    for timer in timers {
        if timer >= initial_state.len() {
            return Err(anyhow!("Invalid timer {}", timer));
        }
        initial_state[timer] += 1;
    }

    Ok((
        simulation(initial_state, 80),
        Some(simulation(initial_state, 256)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        assert_eq!(simulation([0, 1, 1, 2, 1, 0, 0, 0, 0], 80), 5934);
        assert_eq!(simulation([0, 1, 1, 2, 1, 0, 0, 0, 0], 256), 26984457539);
        Ok(())
    }
}

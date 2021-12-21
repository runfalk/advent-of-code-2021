use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default)]
struct UniverseSplitter {
    cache: HashMap<(usize, usize, usize, usize), (usize, usize)>,
}

impl UniverseSplitter {
    /// Number of outcomes where player 1 and 2 wins respectively given the starting conditions
    fn num_wins(
        &mut self,
        p1_pos: usize,
        p1_rem_score: usize,
        p2_pos: usize,
        p2_rem_score: usize,
    ) -> (usize, usize) {
        let mut num_p1_win = 0;
        let mut num_p2_win = 0;

        // Generate all possible dice roll combinations for 3 rolls of 3 sided dice
        let rolls =
            (1..=3).flat_map(|d1| (1..=3).flat_map(move |d2| (1..=3).map(move |d3| d1 + d2 + d3)));

        for roll in rolls {
            let p1_pos = (p1_pos + roll - 1) % 10 + 1;
            let p1_rem_score = p1_rem_score.saturating_sub(p1_pos);

            if p1_rem_score == 0 {
                num_p1_win += 1;
            } else {
                // If the current player has not won yet we need to recurse let the other player
                // try.  Since we can't possible try all paths we cache previous calls in case we
                // have already computed this exact scenario before. Note that we swap the players
                // in the argument list since it's the other player's turn now
                let args = (p2_pos, p2_rem_score, p1_pos, p1_rem_score);
                let (n_p2, n_p1) = self.cache.get(&args).copied().unwrap_or_else(|| {
                    let outcomes = self.num_wins(args.0, args.1, args.2, args.3);
                    self.cache.insert(args, outcomes);
                    outcomes
                });

                num_p1_win += n_p1;
                num_p2_win += n_p2;
            }
        }
        (num_p1_win, num_p2_win)
    }
}

fn part_a(mut player1_pos: usize, mut player2_pos: usize) -> usize {
    let mut is_player1s_turn = true;
    let mut player1_score = 0;
    let mut player2_score = 0;

    let mut dice = 1..;

    while player1_score < 1000 && player2_score < 1000 {
        let (pos, score) = if is_player1s_turn {
            (&mut player1_pos, &mut player1_score)
        } else {
            (&mut player2_pos, &mut player2_score)
        };

        let roll = dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap();
        *pos = (*pos + roll - 1) % 10 + 1;
        *score += *pos;

        is_player1s_turn = !is_player1s_turn;
    }
    (dice.next().unwrap() - 1) * player1_score.min(player2_score)
}

fn part_b(player1_pos: usize, player2_pos: usize) -> usize {
    let mut universe_splitter = UniverseSplitter::default();
    let (p1_wins, p2_wins) = universe_splitter.num_wins(player1_pos, 21, player2_pos, 21);
    p1_wins.max(p2_wins)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let (player1_str, player2_str) = input
        .split_once("\n")
        .ok_or_else(|| anyhow!("Invalid input"))?;

    let player1 = match player1_str.split_once(": ") {
        Some(("Player 1 starting position", pos)) => pos
            .parse::<usize>()
            .map_err(|_| anyhow!("Player 1 starting positon must be a number, got {:?}", pos)),
        _ => Err(anyhow!("Invalid starting position for player 1")),
    }?;
    let player2 = match player2_str.trim_end().split_once(": ") {
        Some(("Player 2 starting position", pos)) => pos
            .parse::<usize>()
            .map_err(|_| anyhow!("Player 2 starting positon must be a number, got {:?}", pos)),
        _ => Err(anyhow!("Invalid starting position for player 2")),
    }?;

    Ok((part_a(player1, player2), Some(part_b(player1, player2))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(4, 8), 739785);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(4, 8), 444_356_092_776_315);
    }
}

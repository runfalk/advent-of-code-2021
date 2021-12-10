use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum SyntaxError {
    BracketMismatch(char),
    UnmatchedBrackets(Vec<char>),
    InvalidCharacter(char),
}

fn validate_line(l: &str) -> Result<(), SyntaxError> {
    let mut bracket_stack = Vec::new();
    for c in l.chars() {
        if "([{<".contains(c) {
            bracket_stack.push(match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => unreachable!(),
            });
        } else if ">}])".contains(c) {
            match bracket_stack.pop() {
                Some(s) if c == s => (),
                _ => return Err(SyntaxError::BracketMismatch(c)),
            }
        } else {
            return Err(SyntaxError::InvalidCharacter(c));
        }
    }

    if !bracket_stack.is_empty() {
        return Err(SyntaxError::UnmatchedBrackets(
            bracket_stack.into_iter().rev().collect(),
        ));
    }

    Ok(())
}

fn part_a<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    let mut penalty = 0;
    for line in lines {
        match validate_line(line.as_ref()) {
            Err(SyntaxError::BracketMismatch(c)) => match c {
                ')' => penalty += 3,
                ']' => penalty += 57,
                '}' => penalty += 1197,
                '>' => penalty += 25137,
                _ => unreachable!(),
            },
            Err(SyntaxError::UnmatchedBrackets(_)) => (),
            Err(SyntaxError::InvalidCharacter(c)) => {
                return Err(anyhow!("Invalid character {}", c))
            }
            Ok(()) => return Err(anyhow!("Got a line that was OK?!")),
        }
    }
    Ok(penalty)
}

fn part_b<S: AsRef<str>>(lines: &[S]) -> Result<usize> {
    let mut penalties = Vec::new();
    for line in lines {
        let mut penalty = 0;
        let unmatched_brackets = match validate_line(line.as_ref()) {
            Err(SyntaxError::UnmatchedBrackets(ub)) => ub,
            Err(SyntaxError::BracketMismatch(_)) => continue,
            Err(SyntaxError::InvalidCharacter(c)) => {
                return Err(anyhow!("Invalid character {}", c))
            }
            Ok(()) => return Err(anyhow!("Got a line that was OK?!")),
        };

        for c in unmatched_brackets {
            penalty = 5 * penalty
                + match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                }
        }
        penalties.push(penalty);
    }
    penalties.sort_unstable();
    Ok(penalties[penalties.len() / 2])
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    Ok((part_a(&lines)?, Some(part_b(&lines)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINES: &'static [&str] = &[
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn test_part_a() -> Result<()> {
        assert_eq!(part_a(&LINES)?, 26397);
        assert_eq!(part_b(&LINES)?, 288957);
        Ok(())
    }
}

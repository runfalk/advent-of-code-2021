use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Display {
    patterns: Vec<Segments>,
    output: Vec<Segments>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Segments(u8);

impl Segments {
    fn from_str(s: &str) -> Result<Self> {
        let mut segments = 0;
        for c in s.chars() {
            segments |= match c {
                'a' => 0b0000001,
                'b' => 0b0000010,
                'c' => 0b0000100,
                'd' => 0b0001000,
                'e' => 0b0010000,
                'f' => 0b0100000,
                'g' => 0b1000000,
                c => return Err(anyhow!("Got unknown segment {}", c)),
            }
        }
        Ok(Self(segments))
    }

    fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    fn contains(self, other: Self) -> bool {
        (self.0 | other.0) == self.0
    }
}

fn part_a(displays: &[Display]) -> usize {
    displays
        .iter()
        .flat_map(|d| d.output.iter())
        .filter(|o| o.len() == 2 || o.len() == 3 || o.len() == 4 || o.len() == 7)
        .count()
}

fn part_b(displays: &[Display]) -> Result<usize> {
    let mut sum = 0;
    for display in displays {
        let patterns = display.patterns.iter().copied();
        let mut map = [Segments(0); 10];

        for pattern in patterns.clone() {
            match pattern.len() {
                2 => map[1] = pattern,
                4 => map[4] = pattern,
                3 => map[7] = pattern,
                7 => map[8] = pattern,
                _ => (),
            }
        }

        if map[1].len() == 0 || map[4].len() == 0 || map[7].len() == 0 || map[8].len() == 0 {
            return Err(anyhow!("Couldn't find 1, 4, 7 and 8 in pattern"));
        }

        map[3] = patterns
            .clone()
            .find(|&p| p.len() == 5 && p.contains(map[7]))
            .ok_or_else(|| anyhow!("Unable to find segments for 3"))?;

        map[6] = patterns
            .clone()
            .find(|&p| p.len() == 6 && !p.contains(map[1]))
            .ok_or_else(|| anyhow!("Unable to find segments for 6"))?;
        map[9] = patterns
            .clone()
            .find(|&p| p.len() == 6 && p.contains(map[3]))
            .ok_or_else(|| anyhow!("Unable to find segments for 9"))?;
        map[0] = patterns
            .clone()
            .find(|&p| p.len() == 6 && p != map[6] && p != map[9])
            .ok_or_else(|| anyhow!("Unable to find segments for 0"))?;

        map[5] = patterns
            .clone()
            .find(|&p| p.len() == 5 && map[6].contains(p))
            .ok_or_else(|| anyhow!("Unable to find segments for 5"))?;
        map[2] = patterns
            .clone()
            .find(|&p| p.len() == 5 && p != map[3] && p != map[5])
            .ok_or_else(|| anyhow!("Unable to find segments for 2"))?;

        // Use map to convert the output into a four digit number and add it to the total sum
        for (pow, output) in display.output.iter().copied().rev().enumerate() {
            for (digit, segments) in map.into_iter().enumerate() {
                if segments != output {
                    continue;
                }
                sum += 10usize.pow(pow as u32) * digit;
                break;
            }
        }
    }
    Ok(sum)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let displays = io::BufReader::new(file)
        .lines()
        .map(|lr| {
            let line = lr?;
            let (patterns_str, output_str) = line
                .split_once(" | ")
                .ok_or_else(|| anyhow!("No display delimiter found"))?;
            Ok(Display {
                patterns: patterns_str
                    .split_whitespace()
                    .map(Segments::from_str)
                    .collect::<Result<Vec<_>>>()?,
                output: output_str
                    .split_whitespace()
                    .map(Segments::from_str)
                    .collect::<Result<Vec<_>>>()?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((part_a(&displays), Some(part_b(&displays)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = vec![
            (
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb",
                "fdgacbe cefdb cefbgd gcbe",
            ),
            (
                "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec",
                "fcgedb cgb dgebacf gc",
            ),
            (
                "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef",
                "cg cg fdcagb cbg",
            ),
            (
                "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega",
                "efabcd cedba gadfec cb",
            ),
            (
                "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga",
                "gecf egdcabf bgf bfgea",
            ),
            (
                "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf",
                "gebdcfa ecba ca fadegcb",
            ),
            (
                "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf",
                "cefg dcbef fcge gbcadfe",
            ),
            (
                "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd",
                "ed bcgafe cdgba cbgef",
            ),
            (
                "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg",
                "gbdfcae bgc cg cgb",
            ),
            (
                "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc",
                "fgae cfgab fg bagce",
            ),
        ];
        let displays = input
            .into_iter()
            .map(|(patterns_str, output_str)| {
                Ok(Display {
                    patterns: patterns_str
                        .split_whitespace()
                        .map(Segments::from_str)
                        .collect::<Result<_>>()?,
                    output: output_str
                        .split_whitespace()
                        .map(Segments::from_str)
                        .collect::<Result<_>>()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        assert_eq!(part_a(&displays), 26);
        assert_eq!(part_b(&displays)?, 61229);

        Ok(())
    }
}

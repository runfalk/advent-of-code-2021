use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Display {
    patterns: Vec<u8>,
    output: Vec<u8>,
}

fn part_a(displays: &[Display]) -> usize {
    displays
        .iter()
        .flat_map(|d| d.output.iter())
        .filter(|o| {
            o.count_ones() == 2 || o.count_ones() == 3 || o.count_ones() == 4 || o.count_ones() == 7
        })
        .count()
}

fn part_b(displays: &[Display]) -> Result<usize> {
    let mut sum = 0;
    for display in displays {
        let patterns = display.patterns.iter().copied();
        let one = patterns
            .clone()
            .find(|s| s.count_ones() == 2)
            .ok_or_else(|| anyhow!("Unable to find segments for one"))?;
        let four = patterns
            .clone()
            .find(|s| s.count_ones() == 4)
            .ok_or_else(|| anyhow!("Unable to find segments for four"))?;
        let seven = patterns
            .clone()
            .find(|s| s.count_ones() == 3)
            .ok_or_else(|| anyhow!("Unable to find segments for seven"))?;
        let eight = patterns
            .clone()
            .find(|s| s.count_ones() == 7)
            .ok_or_else(|| anyhow!("Unable to find segments for eight"))?;

        let aeg = eight ^ four;
        let two = patterns
            .clone()
            .find(|s| s.count_ones() == 5 && (aeg & s) == aeg)
            .ok_or_else(|| anyhow!("Unable to find segments for two"))?;
        let three = patterns
            .clone()
            .find(|s| s.count_ones() == 5 && (seven & s) == seven)
            .ok_or_else(|| anyhow!("Unable to find segments for three"))?;
        let five = patterns
            .clone()
            .find(|&s| s.count_ones() == 5 && s != two && s != three)
            .ok_or_else(|| anyhow!("Unable to find segments for five"))?;

        let six = patterns
            .clone()
            .find(|&s| s.count_ones() == 6 && (s & one).count_ones() == 1)
            .ok_or_else(|| anyhow!("Unable to find segments for six"))?;
        let nine = patterns
            .clone()
            .find(|&s| s.count_ones() == 6 && (s ^ three).count_ones() == 1)
            .ok_or_else(|| anyhow!("Unable to find segments for nine"))?;
        let zero = patterns
            .clone()
            .find(|&s| s.count_ones() == 6 && ((six ^ nine) & s).count_ones() == 2)
            .ok_or_else(|| anyhow!("Unable to find segments for zero"))?;

        for (pow, &output) in display.output.iter().rev().enumerate() {
            let digit = if output == zero {
                0
            } else if output == one {
                1
            } else if output == two {
                2
            } else if output == three {
                3
            } else if output == four {
                4
            } else if output == five {
                5
            } else if output == six {
                6
            } else if output == seven {
                7
            } else if output == eight {
                8
            } else if output == nine {
                9
            } else {
                return Err(anyhow!("Unknown output"));
            };

            sum += 10usize.pow(pow as u32) * digit
        }
    }
    Ok(sum)
}

fn str_to_segments(s: &str) -> Result<u8> {
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
            _ => return Err(anyhow!("Got unknown segment")),
        }
    }
    Ok(segments)
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
                    .map(str_to_segments)
                    .collect::<Result<Vec<_>>>()?,
                output: output_str
                    .split_whitespace()
                    .map(str_to_segments)
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
                        .map(str_to_segments)
                        .collect::<Result<_>>()?,
                    output: output_str
                        .split_whitespace()
                        .map(str_to_segments)
                        .collect::<Result<_>>()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        assert_eq!(part_a(&displays), 26);
        assert_eq!(part_b(&displays)?, 61229);

        Ok(())
    }
}

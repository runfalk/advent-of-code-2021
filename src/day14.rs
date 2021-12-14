use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;

struct PolymerExpander {
    rules: HashMap<(char, char), char>,
    cache: HashMap<(char, char, usize), HashMap<char, usize>>,
}

impl PolymerExpander {
    fn new(rules: &HashMap<(char, char), char>) -> Self {
        Self {
            rules: rules.clone(),
            cache: rules
                .keys()
                .map(|&(a, b)| {
                    let mut counts = HashMap::new();
                    counts.insert(b, 1);
                    ((a, b, 0), counts)
                })
                .collect(),
        }
    }

    fn expand_pair(&mut self, a: char, b: char, depth: usize) -> HashMap<char, usize> {
        // Use cached value if we can
        if let Some(cached) = self.cache.get(&(a, b, depth)) {
            return cached.clone();
        }

        // Find which element that should be inserted between a and b
        let insertion = self.rules.get(&(a, b)).cloned().unwrap();

        // Recursively find the count of all polymer pairs
        let left = self.expand_pair(a, insertion, depth - 1);
        let right = self.expand_pair(insertion, b, depth - 1);

        let mut counts = left;
        right
            .into_iter()
            .for_each(|(k, v)| *counts.entry(k).or_default() += v);

        // Update cache before returning
        self.cache.insert((a, b, depth), counts.clone());
        counts
    }

    fn expand_template(&mut self, template: &str, depth: usize) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        counts.insert(template.chars().next().unwrap(), 1);

        for (p, c) in template.chars().zip(template.chars().skip(1)) {
            self.expand_pair(p, c, depth)
                .into_iter()
                .for_each(|(k, v)| *counts.entry(k).or_default() += v);
        }
        counts
    }
}

fn part_a(template: &str, rules: &HashMap<(char, char), char>) -> usize {
    let mut polymer_expander = PolymerExpander::new(rules);
    let counts = polymer_expander.expand_template(template, 10);

    let most_common = counts.values().copied().max().unwrap();
    let least_common = counts.values().copied().min().unwrap();
    most_common - least_common
}

fn part_b(template: &str, rules: &HashMap<(char, char), char>) -> usize {
    let mut polymer_expander = PolymerExpander::new(rules);
    let counts = polymer_expander.expand_template(template, 40);

    let most_common = counts.values().copied().max().unwrap();
    let least_common = counts.values().copied().min().unwrap();
    most_common - least_common
}

fn parse_insertion_rule(rule: &str) -> Option<((char, char), char)> {
    let (pair, insertion) = rule.split_once(" -> ")?;
    if pair.len() != 2 || insertion.len() != 1 {
        return None;
    }
    Some((
        (pair.chars().next()?, pair.chars().nth(1)?),
        insertion.chars().next()?,
    ))
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let (template, rules_str) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Unable to find insertion rules"))?;

    let rules = rules_str
        .lines()
        .map(|l| parse_insertion_rule(l).ok_or_else(|| anyhow!("{:?} is not a valid rule", l)))
        .collect::<Result<HashMap<(char, char), char>>>()?;

    Ok((part_a(template, &rules), Some(part_b(template, &rules))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let template = "NNCB";
        let mut rules = HashMap::new();
        rules.insert(('C', 'H'), 'B');
        rules.insert(('H', 'H'), 'N');
        rules.insert(('C', 'B'), 'H');
        rules.insert(('N', 'H'), 'C');
        rules.insert(('H', 'B'), 'C');
        rules.insert(('H', 'C'), 'B');
        rules.insert(('H', 'N'), 'C');
        rules.insert(('N', 'N'), 'C');
        rules.insert(('B', 'H'), 'H');
        rules.insert(('N', 'C'), 'B');
        rules.insert(('N', 'B'), 'B');
        rules.insert(('B', 'N'), 'B');
        rules.insert(('B', 'B'), 'N');
        rules.insert(('B', 'C'), 'B');
        rules.insert(('C', 'C'), 'N');
        rules.insert(('C', 'N'), 'C');

        assert_eq!(part_a(template, &rules), 1588);
        assert_eq!(part_b(template, &rules), 2188189693529);

        Ok(())
    }
}

use anyhow::{anyhow, Result};
use core::fmt;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{map, map_res, recognize};
use nom::multi::many1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailfishNumber {
    Nested(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Literal(usize),
}

#[derive(Debug)]
enum Explode {
    Unchanged(SnailfishNumber),
    Changed(SnailfishNumber),
    ApplyBoth(usize, usize),
    ApplyLeft(usize, SnailfishNumber),
    ApplyRight(SnailfishNumber, usize),
}

#[derive(Debug)]
enum Split {
    Unchanged(SnailfishNumber),
    Changed(SnailfishNumber),
}

impl SnailfishNumber {
    fn literal_pair(left: usize, right: usize) -> Self {
        Self::nested(Self::Literal(left), Self::Literal(right))
    }

    fn nested(left: Self, right: Self) -> Self {
        Self::Nested(Box::new(left), Box::new(right))
    }

    /// Find the leftmost literal and add the given number
    fn apply_left(&self, v: usize) -> Self {
        match self {
            Self::Nested(l, r) => Self::nested(l.apply_left(v), *r.clone()),
            Self::Literal(n) => Self::Literal(n + v),
        }
    }

    /// Find the rightmost literal and add the given number
    fn apply_right(&self, v: usize) -> Self {
        match self {
            Self::Nested(l, r) => Self::nested(*l.clone(), r.apply_right(v)),
            Self::Literal(n) => Self::Literal(n + v),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Nested(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
            Self::Literal(n) => *n,
        }
    }

    fn explode(&self) -> Option<Self> {
        match self.explode_inner(0) {
            Explode::Changed(n) | Explode::ApplyLeft(_, n) | Explode::ApplyRight(n, _) => Some(n),
            Explode::Unchanged(_) => None,
            Explode::ApplyBoth(_, _) => panic!(),
        }
    }

    fn explode_inner(&self, depth: usize) -> Explode {
        match self {
            Self::Nested(l, r) => {
                match (l.as_ref(), r.as_ref()) {
                    // If we encounter a literal pair we need to check the depth and potentially
                    // explode it
                    (Self::Literal(left), Self::Literal(right)) => {
                        if depth >= 4 {
                            Explode::ApplyBoth(*left, *right)
                        } else {
                            Explode::Unchanged(self.clone())
                        }
                    }
                    (left, right) => {
                        match left.explode_inner(depth + 1) {
                            Explode::ApplyBoth(l, r) => Explode::ApplyLeft(
                                l,
                                Self::nested(Self::Literal(0), right.apply_left(r)),
                            ),
                            Explode::ApplyLeft(l, n) => {
                                Explode::ApplyLeft(l, Self::nested(n, right.clone()))
                            }
                            Explode::ApplyRight(n, r) => {
                                Explode::Changed(Self::nested(n, right.apply_left(r)))
                            }
                            Explode::Changed(n) => Explode::Changed(Self::nested(n, right.clone())),
                            Explode::Unchanged(_) => {
                                // Since the left side was unchanged we go down the right
                                match right.explode_inner(depth + 1) {
                                    Explode::ApplyBoth(l, r) => Explode::ApplyRight(
                                        Self::nested(left.apply_right(l), Self::Literal(0)),
                                        r,
                                    ),
                                    Explode::ApplyLeft(l, n) => {
                                        Explode::Changed(Self::nested(left.apply_right(l), n))
                                    }
                                    Explode::ApplyRight(n, r) => {
                                        Explode::ApplyRight(Self::nested(left.clone(), n), r)
                                    }
                                    Explode::Changed(n) => {
                                        Explode::Changed(Self::nested(left.clone(), n))
                                    }
                                    Explode::Unchanged(_) => Explode::Unchanged(Self::nested(
                                        left.clone(),
                                        right.clone(),
                                    )),
                                }
                            }
                        }
                    }
                }
            }
            n => Explode::Unchanged(n.clone()),
        }
    }

    fn split(&self) -> Option<Self> {
        match self.split_inner() {
            Split::Changed(n) => Some(n),
            Split::Unchanged(_) => None,
        }
    }

    fn split_inner(&self) -> Split {
        match self {
            Self::Nested(left, right) => {
                if let Split::Changed(n) = left.split_inner() {
                    return Split::Changed(Self::nested(n, right.as_ref().clone()));
                }
                if let Split::Changed(n) = right.split_inner() {
                    return Split::Changed(Self::nested(left.as_ref().clone(), n));
                }
                Split::Unchanged(Self::nested(left.as_ref().clone(), right.as_ref().clone()))
            }
            Self::Literal(n) => {
                if *n >= 10 {
                    Split::Changed(Self::literal_pair(n / 2, n - n / 2))
                } else {
                    Split::Unchanged(Self::Literal(*n))
                }
            }
        }
    }

    fn reduce(&self) -> Self {
        let mut num = self.clone();
        loop {
            if let Some(n) = num.explode() {
                println!("explode");
                num = n;
                continue;
            }
            if let Some(n) = num.split() {
                println!("split");
                num = n;
                continue;
            }
            break;
        }
        num
    }

    fn from_str(input: &str) -> Result<Self> {
        parse_snailfish_number(input)
            .map(|(_, n)| n)
            .map_err(|_| anyhow!("Invalid snailfish number"))
    }

    fn add(&self, other: &Self) -> Self {
        Self::nested(self.clone(), other.clone())
    }

    fn sum(nums: &[Self]) -> Self {
        nums.iter()
            .cloned()
            .reduce(|num, add| num.add(&add).reduce())
            .unwrap()
    }
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailfishNumber::Nested(l, r) => write!(f, "[{},{}]", l, r),
            SnailfishNumber::Literal(n) => write!(f, "{}", n),
        }
    }
}

fn parse_snailfish_number(input: &str) -> IResult<&str, SnailfishNumber> {
    delimited(
        tag("["),
        map(
            separated_pair(parse_snailfish_part, tag(","), parse_snailfish_part),
            |(a, b)| SnailfishNumber::nested(a, b),
        ),
        tag("]"),
    )(input)
}

fn parse_snailfish_literal(input: &str) -> IResult<&str, usize> {
    map_res(recognize(many1(one_of("0123456789"))), |n: &str| {
        n.parse::<usize>()
    })(input)
}

fn parse_snailfish_part(input: &str) -> IResult<&str, SnailfishNumber> {
    alt((
        map(parse_snailfish_literal, SnailfishNumber::Literal),
        parse_snailfish_number,
    ))(input)
}

fn part_a(nums: &[SnailfishNumber]) -> usize {
    SnailfishNumber::sum(nums).magnitude()
}

fn part_b(nums: &[SnailfishNumber]) -> usize {
    let mut max = 0;
    for a in nums {
        for b in nums {
            max = max.max(a.add(b).reduce().magnitude());
        }
    }
    max
}

pub fn main(path: &Path) -> Result<(usize, Option<u128>)> {
    let nums = io::BufReader::new(File::open(path)?)
        .lines()
        .map(|lr| Ok(SnailfishNumber::from_str(&lr?)?))
        .collect::<Result<Vec<SnailfishNumber>>>()?;

    dbg!(part_a(&nums));
    dbg!(part_b(&nums));

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<()> {
        let nums = &[
            "[1,2]",
            "[[1,2],3]",
            "[9,[8,7]]",
            "[[1,9],[8,5]]",
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
        ];
        for n in nums {
            assert_eq!(&format!("{}", SnailfishNumber::from_str(n)?), n);
        }
        Ok(())
    }

    #[test]
    fn test_magnitude() -> Result<()> {
        assert_eq!(
            SnailfishNumber::from_str("[[1,2],[[3,4],5]]")?.magnitude(),
            143
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?.magnitude(),
            1384
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]")?.magnitude(),
            445
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]")?.magnitude(),
            791
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]")?.magnitude(),
            1137
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")?
                .magnitude(),
            3488
        );
        Ok(())
    }
    #[test]
    fn test_explode() -> Result<()> {
        assert_eq!(
            SnailfishNumber::from_str("[[[[[9,8],1],2],3],4]")?.explode(),
            Some(SnailfishNumber::from_str("[[[[0,9],2],3],4]")?)
        );
        assert_eq!(
            SnailfishNumber::from_str("[7,[6,[5,[4,[3,2]]]]]")?.explode(),
            Some(SnailfishNumber::from_str("[7,[6,[5,[7,0]]]]")?)
        );
        assert_eq!(
            SnailfishNumber::from_str("[[6,[5,[4,[3,2]]]],1]")?.explode(),
            Some(SnailfishNumber::from_str("[[6,[5,[7,0]]],3]")?)
        );
        assert_eq!(
            SnailfishNumber::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")?.explode(),
            Some(SnailfishNumber::from_str(
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            )?)
        );
        assert_eq!(
            SnailfishNumber::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")?.explode(),
            Some(SnailfishNumber::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")?)
        );
        Ok(())
    }

    #[test]
    fn test_split() -> Result<()> {
        assert_eq!(
            SnailfishNumber::from_str("[[[[0,7],4],[15,[0,13]]],[1,1]]")?.split(),
            Some(SnailfishNumber::from_str(
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
            )?)
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")?.split(),
            Some(SnailfishNumber::from_str(
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"
            )?)
        );
        Ok(())
    }

    #[test]
    fn test_reduce() -> Result<()> {
        assert_eq!(
            SnailfishNumber::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")?.reduce(),
            SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?,
        );
        Ok(())
    }

    #[test]
    fn test_add() -> Result<()> {
        assert_eq!(
            SnailfishNumber::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]")?
                .add(&SnailfishNumber::from_str("[1,1]")?),
            SnailfishNumber::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")?,
        );
        Ok(())
    }

    #[test]
    fn test_sum() -> Result<()> {
        let input = &[
            SnailfishNumber::from_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")?,
            SnailfishNumber::from_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")?,
            SnailfishNumber::from_str("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]")?,
            SnailfishNumber::from_str("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]")?,
            SnailfishNumber::from_str("[7,[5,[[3,8],[1,4]]]]")?,
            SnailfishNumber::from_str("[[2,[2,2]],[8,[8,1]]]")?,
            SnailfishNumber::from_str("[2,9]")?,
            SnailfishNumber::from_str("[1,[[[9,3],9],[[9,0],[0,7]]]]")?,
            SnailfishNumber::from_str("[[[5,[7,4]],7],1]")?,
            SnailfishNumber::from_str("[[[[4,2],2],6],[8,7]]")?,
        ];
        assert_eq!(
            SnailfishNumber::sum(input),
            SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")?
        );

        let input = &[
            SnailfishNumber::from_str("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]")?,
            SnailfishNumber::from_str("[[[5,[2,8]],4],[5,[[9,9],0]]]")?,
            SnailfishNumber::from_str("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]")?,
            SnailfishNumber::from_str("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]")?,
            SnailfishNumber::from_str("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]")?,
            SnailfishNumber::from_str("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]")?,
            SnailfishNumber::from_str("[[[[5,4],[7,7]],8],[[8,3],8]]")?,
            SnailfishNumber::from_str("[[9,3],[[9,9],[6,[4,9]]]]")?,
            SnailfishNumber::from_str("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]")?,
            SnailfishNumber::from_str("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]")?,
        ];
        assert_eq!(
            SnailfishNumber::sum(input),
            SnailfishNumber::from_str(
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            )?
        );
        Ok(())
    }

    #[test]
    fn test_part_ab() -> Result<()> {
        let input = &[
            SnailfishNumber::from_str("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]")?,
            SnailfishNumber::from_str("[[[5,[2,8]],4],[5,[[9,9],0]]]")?,
            SnailfishNumber::from_str("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]")?,
            SnailfishNumber::from_str("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]")?,
            SnailfishNumber::from_str("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]")?,
            SnailfishNumber::from_str("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]")?,
            SnailfishNumber::from_str("[[[[5,4],[7,7]],8],[[8,3],8]]")?,
            SnailfishNumber::from_str("[[9,3],[[9,9],[6,[4,9]]]]")?,
            SnailfishNumber::from_str("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]")?,
            SnailfishNumber::from_str("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]")?,
        ];
        assert_eq!(part_a(input), 4140);
        assert_eq!(part_b(input), 3993);
        Ok(())
    }
}

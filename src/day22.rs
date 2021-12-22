use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{map, map_res, opt, recognize, value};
use nom::multi::many1;
use nom::sequence::{pair, preceded, separated_pair, tuple};
use nom::IResult;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::path::Path;

#[derive(Debug)]
struct RebootStep {
    turn_on: bool,
    cube: CubeSelection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CubeSelection {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

impl CubeSelection {
    fn new(
        x: RangeInclusive<isize>,
        y: RangeInclusive<isize>,
        z: RangeInclusive<isize>,
    ) -> Option<Self> {
        if !x.is_empty() && !y.is_empty() && !z.is_empty() {
            Some(Self { x, y, z })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.x.clone().count() * self.y.clone().count() * self.z.clone().count()
    }

    fn bounding_box(&self, other: &Self) -> Self {
        let x = (*self.x.start()).min(*other.x.start())..=(*self.x.end()).max(*other.x.end());
        let y = (*self.y.start()).min(*other.y.start())..=(*self.y.end()).max(*other.y.end());
        let z = (*self.z.start()).min(*other.z.start())..=(*self.z.end()).max(*other.z.end());
        Self { x, y, z }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        Self::new(
            (*self.x.start()).max(*other.x.start())..=(*self.x.end()).min(*other.x.end()),
            (*self.y.start()).max(*other.y.start())..=(*self.y.end()).min(*other.y.end()),
            (*self.z.start()).max(*other.z.start())..=(*self.z.end()).min(*other.z.end()),
        )
    }

    /// Return a vector of cubes representing the volume of this selection that doesn't intersect the
    /// given other selection
    ///
    /// This function is not my proudest work :(
    fn difference(&self, other: &Self) -> Vec<Self> {
        // If the selections don't intersect we don't need to modify this one at all
        let inner = match self.intersection(other) {
            Some(i) => i,
            None => return vec![self.clone()],
        };
        let outer = self.bounding_box(other);

        // Split the bounding box into 27 separate selections with the intersection in the middle.
        // Then we can keep the bits which intersects with this selection but not with the other.
        let mut out = Vec::new();
        for z in [
            *outer.z.start()..=*inner.z.start() - 1,
            *inner.z.start()..=*inner.z.end(),
            *inner.z.end() + 1..=*outer.z.end(),
        ] {
            for y in [
                *outer.y.start()..=*inner.y.start() - 1,
                *inner.y.start()..=*inner.y.end(),
                *inner.y.end() + 1..=*outer.y.end(),
            ] {
                for x in [
                    *outer.x.start()..=*inner.x.start() - 1,
                    *inner.x.start()..=*inner.x.end(),
                    *inner.x.end() + 1..=*outer.x.end(),
                ] {
                    if let Some(selection) =
                        Self::new(x, y.clone(), z.clone()).and_then(|s| self.intersection(&s))
                    {
                        if selection.intersection(other).is_none() {
                            out.push(selection);
                        }
                    }
                }
            }
        }

        out
    }
}

fn parse_number(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(pair(opt(tag("-")), many1(one_of("0123456789")))),
        |d: &str| d.parse::<isize>(),
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<isize>> {
    map(
        separated_pair(parse_number, tag(".."), parse_number),
        |(start, end)| start..=end,
    )(input)
}

fn parse_reboot_step(input: &str) -> Result<RebootStep, nom::Err<nom::error::Error<String>>> {
    map(
        tuple((
            alt((value(true, tag("on")), value(false, tag("off")))),
            preceded(tag(" x="), parse_range),
            preceded(tag(",y="), parse_range),
            preceded(tag(",z="), parse_range),
        )),
        |(state, x, y, z)| RebootStep {
            turn_on: state,
            cube: CubeSelection { x, y, z },
        },
    )(input)
    .map(|(_, step)| step)
    .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
}

fn part_a(reboot_steps: &[RebootStep]) -> usize {
    // Since we're only looking at one million cubes we can brute force
    let mut on = HashSet::new();
    for step in reboot_steps {
        for z in (-50).max(*step.cube.z.start())..=50.min(*step.cube.z.end()) {
            for y in (-50).max(*step.cube.y.start())..=50.min(*step.cube.y.end()) {
                for x in (-50).max(*step.cube.x.start())..=50.min(*step.cube.x.end()) {
                    if step.turn_on {
                        on.insert((x, y, z));
                    } else {
                        on.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    on.len()
}

fn part_b(reboot_steps: &[RebootStep]) -> usize {
    let mut on: Vec<CubeSelection> = Vec::new();
    for step in reboot_steps {
        on = on
            .into_iter()
            .flat_map(|c| c.difference(&step.cube).into_iter())
            .collect();
        if step.turn_on {
            on.push(step.cube.clone());
        }
    }
    on.iter().map(|c| c.len()).sum::<usize>()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let reboot_steps = io::BufReader::new(File::open(path)?)
        .lines()
        .map(|lr| Ok(parse_reboot_step(&lr?)?))
        .collect::<Result<Vec<_>>>()?;
    Ok((part_a(&reboot_steps), Some(part_b(&reboot_steps))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static [&str] = &[
        "on x=-5..47,y=-31..22,z=-19..33",
        "on x=-44..5,y=-27..21,z=-14..35",
        "on x=-49..-1,y=-11..42,z=-10..38",
        "on x=-20..34,y=-40..6,z=-44..1",
        "off x=26..39,y=40..50,z=-2..11",
        "on x=-41..5,y=-41..6,z=-36..8",
        "off x=-43..-33,y=-45..-28,z=7..25",
        "on x=-33..15,y=-32..19,z=-34..11",
        "off x=35..47,y=-46..-34,z=-11..5",
        "on x=-14..36,y=-6..44,z=-16..29",
        "on x=-57795..-6158,y=29564..72030,z=20435..90618",
        "on x=36731..105352,y=-21140..28532,z=16094..90401",
        "on x=30999..107136,y=-53464..15513,z=8553..71215",
        "on x=13528..83982,y=-99403..-27377,z=-24141..23996",
        "on x=-72682..-12347,y=18159..111354,z=7391..80950",
        "on x=-1060..80757,y=-65301..-20884,z=-103788..-16709",
        "on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856",
        "on x=-52752..22273,y=-49450..9096,z=54442..119054",
        "on x=-29982..40483,y=-108474..-28371,z=-24328..38471",
        "on x=-4958..62750,y=40422..118853,z=-7672..65583",
        "on x=55694..108686,y=-43367..46958,z=-26781..48729",
        "on x=-98497..-18186,y=-63569..3412,z=1232..88485",
        "on x=-726..56291,y=-62629..13224,z=18033..85226",
        "on x=-110886..-34664,y=-81338..-8658,z=8914..63723",
        "on x=-55829..24974,y=-16897..54165,z=-121762..-28058",
        "on x=-65152..-11147,y=22489..91432,z=-58782..1780",
        "on x=-120100..-32970,y=-46592..27473,z=-11695..61039",
        "on x=-18631..37533,y=-124565..-50804,z=-35667..28308",
        "on x=-57817..18248,y=49321..117703,z=5745..55881",
        "on x=14781..98692,y=-1341..70827,z=15753..70151",
        "on x=-34419..55919,y=-19626..40991,z=39015..114138",
        "on x=-60785..11593,y=-56135..2999,z=-95368..-26915",
        "on x=-32178..58085,y=17647..101866,z=-91405..-8878",
        "on x=-53655..12091,y=50097..105568,z=-75335..-4862",
        "on x=-111166..-40997,y=-71714..2688,z=5609..50954",
        "on x=-16602..70118,y=-98693..-44401,z=5197..76897",
        "on x=16383..101554,y=4615..83635,z=-44907..18747",
        "off x=-95822..-15171,y=-19987..48940,z=10804..104439",
        "on x=-89813..-14614,y=16069..88491,z=-3297..45228",
        "on x=41075..99376,y=-20427..49978,z=-52012..13762",
        "on x=-21330..50085,y=-17944..62733,z=-112280..-30197",
        "on x=-16478..35915,y=36008..118594,z=-7885..47086",
        "off x=-98156..-27851,y=-49952..43171,z=-99005..-8456",
        "off x=2032..69770,y=-71013..4824,z=7471..94418",
        "on x=43670..120875,y=-42068..12382,z=-24787..38892",
        "off x=37514..111226,y=-45862..25743,z=-16714..54663",
        "off x=25699..97951,y=-30668..59918,z=-15349..69697",
        "off x=-44271..17935,y=-9516..60759,z=49131..112598",
        "on x=-61695..-5813,y=40978..94975,z=8655..80240",
        "off x=-101086..-9439,y=-7088..67543,z=33935..83858",
        "off x=18020..114017,y=-48931..32606,z=21474..89843",
        "off x=-77139..10506,y=-89994..-18797,z=-80..59318",
        "off x=8476..79288,y=-75520..11602,z=-96624..-24783",
        "on x=-47488..-1262,y=24338..100707,z=16292..72967",
        "off x=-84341..13987,y=2429..92914,z=-90671..-1318",
        "off x=-37810..49457,y=-71013..-7894,z=-105357..-13188",
        "off x=-27365..46395,y=31009..98017,z=15428..76570",
        "off x=-70369..-16548,y=22648..78696,z=-1892..86821",
        "on x=-53470..21291,y=-120233..-33476,z=-44150..38147",
        "off x=-93533..-4276,y=-16170..68771,z=-104985..-24507",
    ];

    #[test]
    fn test_example() -> Result<()> {
        let steps = EXAMPLE
            .iter()
            .map(|l| parse_reboot_step(l))
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(part_a(&steps), 474140);
        assert_eq!(part_b(&steps), 2758514936282235);
        Ok(())
    }
}

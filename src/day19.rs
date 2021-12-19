use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;
use std::collections::{HashSet, VecDeque};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    fn manhattan_distance(self, other: Self) -> usize {
        let c = self.sub(other);
        (c.x.abs() + c.y.abs() + c.z.abs()) as usize
    }
}

#[derive(Debug)]
struct DetectionCube {
    scanners: HashSet<Coordinate>,
    beacons: HashSet<Coordinate>,
}

impl DetectionCube {
    fn new(beacons: HashSet<Coordinate>) -> Self {
        let mut scanners = HashSet::new();
        scanners.insert(Coordinate::new(0, 0, 0));
        Self { scanners, beacons }
    }

    fn from_cubes(mut detection_cubes: Vec<Self>) -> Self {
        // Select one detection cube to start with and try to merge it with the rest
        let mut detection_cube = detection_cubes.pop().unwrap();

        // NOTE: This will loop infinitely if there are scanners that don't share any beacons
        let mut unmerged_detection_cubes = detection_cubes.into_iter().collect::<VecDeque<_>>();
        while let Some(other_scanner) = unmerged_detection_cubes.pop_front() {
            if let Some(m) = detection_cube.try_merge(&other_scanner) {
                detection_cube = m;
            } else {
                unmerged_detection_cubes.push_back(other_scanner);
            }
        }
        detection_cube
    }

    fn rotations(&self) -> Vec<Self> {
        rotations(self.scanners.iter().copied())
            .into_iter()
            .zip(rotations(self.beacons.iter().copied()).into_iter())
            .map(|(scanners, beacons)| Self { scanners, beacons })
            .collect()
    }

    /// Move the origin to `origin`
    fn translate(&self, origin: Coordinate) -> Self {
        Self {
            scanners: self.scanners.iter().map(|c| c.sub(origin)).collect(),
            beacons: self.beacons.iter().map(|c| c.sub(origin)).collect(),
        }
    }

    fn translations(&'_ self) -> impl Iterator<Item = Self> + '_ {
        self.beacons
            .iter()
            .copied()
            .map(|new_origin| self.translate(new_origin))
    }

    fn try_merge(&self, other: &Self) -> Option<Self> {
        // Translate this scanner's origin to all points within the scanner
        for s in self.translations() {
            // We need to check all orientations for the given
            for rotated_other in other.rotations() {
                // For every new origin we need to check that against the other scanner
                for o in rotated_other.translations() {
                    if o.beacons.intersection(&s.beacons).count() >= 12 {
                        return Some(Self {
                            scanners: o.scanners.union(&s.scanners).copied().collect(),
                            beacons: o.beacons.union(&s.beacons).copied().collect(),
                        });
                    }
                }
            }
        }
        None
    }
}

fn rotations<I: Iterator<Item = Coordinate> + Clone>(it: I) -> Vec<HashSet<Coordinate>> {
    vec![
        // All four rotations when original X faces X
        it.clone().collect(),
        it.clone()
            .map(|c| Coordinate::new(c.x, -c.y, -c.z))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(c.x, -c.z, c.y))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(c.x, c.z, -c.y))
            .collect(),
        // All four rotations when original X faces Y
        it.clone()
            .map(|c| Coordinate::new(-c.y, c.x, c.z))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(-c.z, c.x, -c.y))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(c.y, c.x, -c.z))
            .collect(),
        it.clone().map(|c| Coordinate::new(c.z, c.x, c.y)).collect(),
        // All four rotations when original X faces Z
        it.clone()
            .map(|c| Coordinate::new(-c.y, -c.z, c.x))
            .collect(),
        it.clone().map(|c| Coordinate::new(c.y, c.z, c.x)).collect(),
        it.clone()
            .map(|c| Coordinate::new(c.z, -c.y, c.x))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(-c.z, c.y, c.x))
            .collect(),
        // All four rotations when original X faces -X
        it.clone()
            .map(|c| Coordinate::new(-c.x, -c.y, c.z))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(-c.x, -c.z, -c.y))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(-c.x, c.y, -c.z))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(-c.x, c.z, c.y))
            .collect(),
        // All four rotations when original X faces -Y
        it.clone()
            .map(|c| Coordinate::new(c.y, -c.x, c.z))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(-c.z, -c.x, c.y))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(-c.y, -c.x, -c.z))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(c.z, -c.x, -c.y))
            .collect(),
        // All four rotations when original X faces -Z
        it.clone()
            .map(|c| Coordinate::new(c.y, -c.z, -c.x))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(c.z, c.y, -c.x))
            .collect(),
        it.clone()
            .map(|c| Coordinate::new(-c.y, c.z, -c.x))
            .collect(),
        it.map(|c| Coordinate::new(-c.z, -c.y, -c.x)).collect(),
    ]
}

fn parse_number(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(pair(opt(tag("-")), many1(one_of("0123456789")))),
        |d: &str| d.parse::<isize>(),
    )(input)
}

fn parse_scanners(input: &str) -> Result<Vec<DetectionCube>, nom::Err<nom::error::Error<String>>> {
    separated_list1(
        tag("\n\n"),
        map(
            pair(
                delimited(tag("--- scanner "), parse_number, tag(" ---\n")),
                separated_list1(
                    tag("\n"),
                    map(
                        tuple((
                            parse_number,
                            preceded(tag(","), parse_number),
                            preceded(tag(","), parse_number),
                        )),
                        |(x, y, z)| Coordinate::new(x, y, z),
                    ),
                ),
            ),
            |(_, beacons)| DetectionCube::new(beacons.into_iter().collect()),
        ),
    )(input)
    .map(|(_, scanners)| scanners)
    .map_err(|e: nom::Err<nom::error::Error<&str>>| e.to_owned())
}

fn part_a(detection_cube: &DetectionCube) -> usize {
    detection_cube.beacons.len()
}

fn part_b(detection_cube: &DetectionCube) -> Option<usize> {
    detection_cube
        .scanners
        .iter()
        .copied()
        .flat_map(|s1| {
            detection_cube
                .scanners
                .iter()
                .copied()
                .filter(move |s2| s1 != *s2)
                .map(move |s2| s1.manhattan_distance(s2))
        })
        .max()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let detection_cube = DetectionCube::from_cubes(parse_scanners(&input)?);
    Ok((part_a(&detection_cube), part_b(&detection_cube)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<()> {
        let mut example = String::new();
        example.push_str("--- scanner 0 ---\n");
        example.push_str("404,-588,-901\n");
        example.push_str("528,-643,409\n");
        example.push_str("-838,591,734\n");
        example.push_str("390,-675,-793\n");
        example.push_str("-537,-823,-458\n");
        example.push_str("-485,-357,347\n");
        example.push_str("-345,-311,381\n");
        example.push_str("-661,-816,-575\n");
        example.push_str("-876,649,763\n");
        example.push_str("-618,-824,-621\n");
        example.push_str("553,345,-567\n");
        example.push_str("474,580,667\n");
        example.push_str("-447,-329,318\n");
        example.push_str("-584,868,-557\n");
        example.push_str("544,-627,-890\n");
        example.push_str("564,392,-477\n");
        example.push_str("455,729,728\n");
        example.push_str("-892,524,684\n");
        example.push_str("-689,845,-530\n");
        example.push_str("423,-701,434\n");
        example.push_str("7,-33,-71\n");
        example.push_str("630,319,-379\n");
        example.push_str("443,580,662\n");
        example.push_str("-789,900,-551\n");
        example.push_str("459,-707,401\n");
        example.push_str("\n");
        example.push_str("--- scanner 1 ---\n");
        example.push_str("686,422,578\n");
        example.push_str("605,423,415\n");
        example.push_str("515,917,-361\n");
        example.push_str("-336,658,858\n");
        example.push_str("95,138,22\n");
        example.push_str("-476,619,847\n");
        example.push_str("-340,-569,-846\n");
        example.push_str("567,-361,727\n");
        example.push_str("-460,603,-452\n");
        example.push_str("669,-402,600\n");
        example.push_str("729,430,532\n");
        example.push_str("-500,-761,534\n");
        example.push_str("-322,571,750\n");
        example.push_str("-466,-666,-811\n");
        example.push_str("-429,-592,574\n");
        example.push_str("-355,545,-477\n");
        example.push_str("703,-491,-529\n");
        example.push_str("-328,-685,520\n");
        example.push_str("413,935,-424\n");
        example.push_str("-391,539,-444\n");
        example.push_str("586,-435,557\n");
        example.push_str("-364,-763,-893\n");
        example.push_str("807,-499,-711\n");
        example.push_str("755,-354,-619\n");
        example.push_str("553,889,-390\n");
        example.push_str("\n");
        example.push_str("--- scanner 2 ---\n");
        example.push_str("649,640,665\n");
        example.push_str("682,-795,504\n");
        example.push_str("-784,533,-524\n");
        example.push_str("-644,584,-595\n");
        example.push_str("-588,-843,648\n");
        example.push_str("-30,6,44\n");
        example.push_str("-674,560,763\n");
        example.push_str("500,723,-460\n");
        example.push_str("609,671,-379\n");
        example.push_str("-555,-800,653\n");
        example.push_str("-675,-892,-343\n");
        example.push_str("697,-426,-610\n");
        example.push_str("578,704,681\n");
        example.push_str("493,664,-388\n");
        example.push_str("-671,-858,530\n");
        example.push_str("-667,343,800\n");
        example.push_str("571,-461,-707\n");
        example.push_str("-138,-166,112\n");
        example.push_str("-889,563,-600\n");
        example.push_str("646,-828,498\n");
        example.push_str("640,759,510\n");
        example.push_str("-630,509,768\n");
        example.push_str("-681,-892,-333\n");
        example.push_str("673,-379,-804\n");
        example.push_str("-742,-814,-386\n");
        example.push_str("577,-820,562\n");
        example.push_str("\n");
        example.push_str("--- scanner 3 ---\n");
        example.push_str("-589,542,597\n");
        example.push_str("605,-692,669\n");
        example.push_str("-500,565,-823\n");
        example.push_str("-660,373,557\n");
        example.push_str("-458,-679,-417\n");
        example.push_str("-488,449,543\n");
        example.push_str("-626,468,-788\n");
        example.push_str("338,-750,-386\n");
        example.push_str("528,-832,-391\n");
        example.push_str("562,-778,733\n");
        example.push_str("-938,-730,414\n");
        example.push_str("543,643,-506\n");
        example.push_str("-524,371,-870\n");
        example.push_str("407,773,750\n");
        example.push_str("-104,29,83\n");
        example.push_str("378,-903,-323\n");
        example.push_str("-778,-728,485\n");
        example.push_str("426,699,580\n");
        example.push_str("-438,-605,-362\n");
        example.push_str("-469,-447,-387\n");
        example.push_str("509,732,623\n");
        example.push_str("647,635,-688\n");
        example.push_str("-868,-804,481\n");
        example.push_str("614,-800,639\n");
        example.push_str("595,780,-596\n");
        example.push_str("\n");
        example.push_str("--- scanner 4 ---\n");
        example.push_str("727,592,562\n");
        example.push_str("-293,-554,779\n");
        example.push_str("441,611,-461\n");
        example.push_str("-714,465,-776\n");
        example.push_str("-743,427,-804\n");
        example.push_str("-660,-479,-426\n");
        example.push_str("832,-632,460\n");
        example.push_str("927,-485,-438\n");
        example.push_str("408,393,-506\n");
        example.push_str("466,436,-512\n");
        example.push_str("110,16,151\n");
        example.push_str("-258,-428,682\n");
        example.push_str("-393,719,612\n");
        example.push_str("-211,-452,876\n");
        example.push_str("808,-476,-593\n");
        example.push_str("-575,615,604\n");
        example.push_str("-485,667,467\n");
        example.push_str("-680,325,-822\n");
        example.push_str("-627,-443,-432\n");
        example.push_str("872,-547,-609\n");
        example.push_str("833,512,582\n");
        example.push_str("807,604,487\n");
        example.push_str("839,-516,451\n");
        example.push_str("891,-625,532\n");
        example.push_str("-652,-548,-490\n");
        example.push_str("30,-46,-14\n");

        let detection_cube = DetectionCube::from_cubes(parse_scanners(&example)?);
        assert_eq!(part_a(&detection_cube), 79);
        assert_eq!(part_b(&detection_cube), Some(3621));

        Ok(())
    }
}

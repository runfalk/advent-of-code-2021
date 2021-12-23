use anyhow::{anyhow, Result};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt;
use std::path::Path;

/// We need this exotic data structure since we can't store types that don't implement Ord directly
/// in a BinaryHeap
#[derive(Debug, Clone)]
struct PriorityQueue<P, T> {
    heap: BinaryHeap<(P, Reverse<usize>)>,
    values: HashMap<usize, T>,
    next_index: usize,
}

impl<P, T> PriorityQueue<P, T>
where
    P: Ord,
{
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            values: HashMap::new(),
            next_index: 0,
        }
    }

    fn push(&mut self, v: T, p: P) {
        self.heap.push((p, Reverse(self.next_index)));
        self.values.insert(self.next_index, v);
        self.next_index += 1;
    }

    fn pop(&mut self) -> Option<(T, P)> {
        self.heap
            .pop()
            .map(|(p, Reverse(k))| (self.values.remove(&k).unwrap(), p))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Amphipod(Amphipod),
    Wall,
    Space,
    Empty,
}

impl Amphipod {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Self::Amber),
            'B' => Ok(Self::Bronze),
            'C' => Ok(Self::Copper),
            'D' => Ok(Self::Desert),
            c => Err(anyhow!("Invalid amphipod {:?}", c)),
        }
    }

    const fn energy(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

impl Cell {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Empty),
            ' ' => Ok(Self::Space),
            c => Ok(Self::Amphipod(Amphipod::from_char(c)?)),
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Burrow {
    // We can't use HashMap here since it doesn't implement Hash
    cells: Vec<Vec<Cell>>,
}

impl Burrow {
    fn target() -> Self {
        let mut target_str = String::new();
        target_str.push_str("#############\n");
        target_str.push_str("#...........#\n");
        target_str.push_str("###A#B#C#D###\n");
        target_str.push_str("  #A#B#C#D#\n");
        target_str.push_str("  #########\n");

        Self::from_str(&target_str).unwrap()
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(y).and_then(|row| row.get(x)).copied()
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y][x] = cell;
    }

    fn take(&mut self, x: usize, y: usize) -> Option<Cell> {
        self.cells
            .get_mut(y)
            .and_then(|row| row.get_mut(x).map(|v| std::mem::replace(v, Cell::Empty)))
    }

    fn find_amphipods(&self) -> impl Iterator<Item = (usize, usize, Amphipod)> + '_ {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter().copied().enumerate().filter_map(move |(x, c)| {
                if let Cell::Amphipod(a) = c {
                    Some((x, y, a))
                } else {
                    None
                }
            })
        })
    }

    fn is_room(x: usize, y: usize) -> bool {
        matches!(
            (x, y),
            (3, 2) | (3, 3) | (5, 2) | (5, 3) | (7, 2) | (7, 3) | (9, 2) | (9, 3)
        )
    }

    fn is_hallway(x: usize, y: usize) -> bool {
        // We exclude the cells right outside a room as we're not allowed to stop there
        matches!(
            (x, y),
            (1, 1) | (2, 1) | (4, 1) | (6, 1) | (8, 1) | (10, 1) | (11, 1)
        )
    }

    /// Return a list of all reachable cells from the current position and the number of steps to
    /// get there
    fn find_reachable_cells(&self, x: usize, y: usize) -> Vec<(usize, usize, usize)> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((x, y, 0));
        visited.insert((x, y));

        // Explore cells using BFS
        let mut reachable_cells = Vec::new();
        while let Some((x, y, steps)) = queue.pop_front() {
            let up = y
                .checked_sub(1)
                .and_then(|y| self.get(x, y).filter(Cell::is_empty).map(move |_| (x, y)));
            let right = self
                .get(x + 1, y)
                .filter(Cell::is_empty)
                .map(move |_| (x + 1, y));
            let down = self
                .get(x, y + 1)
                .filter(Cell::is_empty)
                .map(move |_| (x, y + 1));
            let left = x
                .checked_sub(1)
                .and_then(|x| self.get(x, y).filter(Cell::is_empty).map(move |_| (x, y)));

            for (nx, ny) in [up, right, down, left].into_iter().flatten() {
                if !visited.contains(&(nx, ny)) {
                    queue.push_back((nx, ny, steps + 1));
                    visited.insert((nx, ny));

                    // We push here since we don't want to include the starting cell
                    reachable_cells.push((nx, ny, steps + 1));
                }
            }
        }
        reachable_cells
    }

    fn from_str(input: &str) -> Result<Self> {
        let cells = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(Cell::from_char)
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { cells })
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for l in self.cells.iter() {
            for cell in l {
                let c = match cell {
                    Cell::Amphipod(Amphipod::Amber) => 'A',
                    Cell::Amphipod(Amphipod::Bronze) => 'B',
                    Cell::Amphipod(Amphipod::Copper) => 'C',
                    Cell::Amphipod(Amphipod::Desert) => 'D',
                    Cell::Wall => '#',
                    Cell::Empty => '.',
                    Cell::Space => ' ',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_a(burrow: Burrow) -> Option<usize> {
    let target = Burrow::target();

    // We use this exotic priority queue instead of binary heap since Burrow can't implement Ord
    let mut queue = PriorityQueue::new();
    let mut visited = HashSet::new();
    queue.push(burrow.clone(), Reverse(0usize));
    visited.insert(burrow);

    while let Some((burrow, Reverse(energy))) = queue.pop() {
        println!("{}{}\n", &burrow, energy);
        if burrow == target {
            return Some(energy);
        }
        visited.insert(burrow.clone());

        // Find all amphipods and explore what paths they can take
        for (x, y, amphipod) in burrow.find_amphipods() {
            // Check which room this amphipod belongs in
            let (outer_target, inner_target) = match amphipod {
                Amphipod::Amber => ((3, 2), (3, 3)),
                Amphipod::Bronze => ((5, 2), (5, 3)),
                Amphipod::Copper => ((7, 2), (7, 3)),
                Amphipod::Desert => ((9, 2), (9, 3)),
            };

            // If we have already reached the inner position we shouldn't go back out again
            if (x, y) == inner_target {
                continue;
            }
            let inner_target_done = matches!(
                burrow.get(inner_target.0, inner_target.1),
                Some(Cell::Amphipod(a)) if a == amphipod,
            );

            if inner_target_done && (x, y) == outer_target {
                continue;
            }

            // Generate all new burrow configurations based on
            for (nx, ny, steps) in burrow.find_reachable_cells(x, y) {
                // If we are currently in a room we can only step out into the hallway
                if Burrow::is_room(x, y) && !Burrow::is_hallway(nx, ny) {
                    continue;
                }

                // If we are in the hallway we must go inside the right room in the right spot
                if Burrow::is_hallway(x, y)
                    && ((!inner_target_done && (nx, ny) != inner_target)
                        || (inner_target_done && (nx, ny) != outer_target))
                {
                    continue;
                }

                let mut new_burrow = burrow.clone();
                let cell = new_burrow.take(x, y).unwrap();
                new_burrow.set(nx, ny, cell);

                if visited.contains(&new_burrow) {
                    continue;
                }

                queue.push(
                    new_burrow.clone(),
                    Reverse(energy + steps * amphipod.energy()),
                );
            }
        }
    }
    None
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let burrow = Burrow::from_str(&input)?;
    Ok((
        part_a(burrow).ok_or_else(|| anyhow!("Can't find a solution for part A"))?,
        None,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        Ok(())
    }
}

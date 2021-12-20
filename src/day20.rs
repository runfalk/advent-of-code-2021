use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn iter_3x3(self) -> impl Iterator<Item = Self> {
        [
            Coordinate::new(self.x - 1, self.y - 1),
            Coordinate::new(self.x, self.y - 1),
            Coordinate::new(self.x + 1, self.y - 1),
            Coordinate::new(self.x - 1, self.y),
            Coordinate::new(self.x, self.y),
            Coordinate::new(self.x + 1, self.y),
            Coordinate::new(self.x - 1, self.y + 1),
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x + 1, self.y + 1),
        ]
        .into_iter()
    }
}

struct SparseImage {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    rest_is_light: bool,
    light_pixels: HashSet<Coordinate>,
}

impl SparseImage {
    fn new(light_pixels: HashSet<Coordinate>) -> Self {
        Self {
            min_x: light_pixels.iter().copied().map(|c| c.x).min().unwrap_or(0),
            max_x: light_pixels.iter().copied().map(|c| c.x).max().unwrap_or(0),
            min_y: light_pixels.iter().copied().map(|c| c.y).min().unwrap_or(0),
            max_y: light_pixels.iter().copied().map(|c| c.y).max().unwrap_or(0),
            rest_is_light: false,
            light_pixels,
        }
    }

    fn is_light(&self, c: Coordinate) -> bool {
        if (self.min_x..=self.max_x).contains(&c.x) && (self.min_y..=self.max_y).contains(&c.y) {
            self.light_pixels.contains(&c)
        } else {
            self.rest_is_light
        }
    }

    fn enhance(&mut self, iea: &[bool; 512]) {
        let mut light_pixels = HashSet::new();

        // We search an area just outside the image as well since the pixels inside the current
        // image may affect them
        let image_coordinates = (self.min_y - 1..=self.max_y + 1)
            .flat_map(|y| (self.min_x - 1..=self.max_x + 1).map(move |x| Coordinate::new(x, y)));
        for c in image_coordinates {
            // Find the correct lookup location by converting the area around the pixel to an
            // integer that we use to lookup the correct location in the image enhancement algorithm
            let i: usize = c
                .iter_3x3()
                .enumerate()
                .filter(|(_, c)| self.is_light(*c))
                .fold(0usize, |i, (b, _)| i | 1 << (8 - b));
            if iea[i] {
                light_pixels.insert(c);
            }
        }

        self.light_pixels = light_pixels;

        // The rest of the pixels may or may not toggle based on the enhancement algorithm
        if self.rest_is_light {
            self.rest_is_light = iea[511];
        } else {
            self.rest_is_light = iea[0];
        }

        // Since we have checked pixels just outside the current image we must expand the image
        // size as well
        self.min_x -= 1;
        self.max_x += 1;
        self.min_y -= 1;
        self.max_y += 1;
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let (enhancement_str, image_str) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Invalid input"))?;

    let iea: [bool; 512] = enhancement_str
        .chars()
        .map(|c| match c {
            '#' => Ok(true),
            '.' => Ok(false),
            c => Err(anyhow!(
                "Invalid character in image enhancment algorithm {:?}",
                c
            )),
        })
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .map_err(|_| anyhow!("Image enhancment algorithm must be 512 long"))?;

    let light_pixels = image_str
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Coordinate::new(x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>();

    let mut image = SparseImage::new(light_pixels);
    for _ in 0..2 {
        image.enhance(&iea);
    }
    let a = image.light_pixels.len();

    for _ in 2..50 {
        image.enhance(&iea);
    }
    let b = image.light_pixels.len();

    Ok((a, Some(b)))
}

use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::path::Path;

struct SparseImage {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    rest_is_light: bool,
    light_pixels: HashSet<(isize, isize)>,
}

impl SparseImage {
    fn new(light_pixels: HashSet<(isize, isize)>) -> Self {
        Self {
            min_x: light_pixels.iter().map(|(x, _)| *x).min().unwrap_or(0),
            max_x: light_pixels.iter().map(|(x, _)| *x).max().unwrap_or(0),
            min_y: light_pixels.iter().map(|(_, y)| *y).min().unwrap_or(0),
            max_y: light_pixels.iter().map(|(_, y)| *y).max().unwrap_or(0),
            rest_is_light: false,
            light_pixels,
        }
    }

    fn is_light(&self, (x, y): (isize, isize)) -> bool {
        if (self.min_x..=self.max_x).contains(&x) && (self.min_y..=self.max_y).contains(&y) {
            self.light_pixels.contains(&(x, y))
        } else {
            self.rest_is_light
        }
    }

    fn enhance(&mut self, image_enhancement_algorithm: &[bool; 512]) {
        let mut light_pixels = HashSet::new();

        // We search an area just outside the image as well since the pixels inside the current
        // image may affect them
        let image_coordinates = (self.min_y - 1..=self.max_y + 1)
            .flat_map(|y| (self.min_x - 1..=self.max_x + 1).map(move |x| (x, y)));
        for (x, y) in image_coordinates {
            // Find the correct lookup location by converting the area around the pixel to an
            // integer that we use to lookup the correct location in the image enhancement algorithm
            let mut index = 0;
            let mut bit = 8;
            for ny in y - 1..=y + 1 {
                for nx in x - 1..=x + 1 {
                    if self.is_light((nx, ny)) {
                        index |= 1 << bit;
                    }
                    bit -= 1;
                }
            }
            if image_enhancement_algorithm[index] {
                light_pixels.insert((x, y));
            }
        }

        self.light_pixels = light_pixels;

        // The rest of the pixels may or may not toggle based on the enhancement algorithm
        if self.rest_is_light {
            self.rest_is_light = image_enhancement_algorithm[511];
        } else {
            self.rest_is_light = image_enhancement_algorithm[0];
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

    let image_enhancement_algorithm: [bool; 512] = enhancement_str
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
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>();

    let mut image = SparseImage::new(light_pixels);
    for _ in 0..2 {
        image.enhance(&image_enhancement_algorithm);
    }
    let a = image.light_pixels.len();

    for _ in 2..50 {
        image.enhance(&image_enhancement_algorithm);
    }
    let b = image.light_pixels.len();

    Ok((a, Some(b)))
}

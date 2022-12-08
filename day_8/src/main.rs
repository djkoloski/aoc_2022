use std::{io::BufRead, fmt::{Display, self}};

use anyhow::Result;
use solve::Input;

struct Grid<T> {
    width: usize,
    height: usize,
    values: Vec<T>,
}

impl<T> Grid<T> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.values[x + y * self.width]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.values[x + y * self.width]
    }
}

impl<T: Default> Grid<T> {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            values: (0..width * height).map(|_| T::default()).collect(),
        }
    }
}

impl Input for Grid<u8> {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut width = 0;
        let mut height = 0;
        let mut values = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let bytes = line.as_bytes();
            width = bytes.len();
            height += 1;
            values.extend(bytes.iter().map(|x| x - b'0'));
        }
        Ok(Self {
            width,
            height,
            values,
        })
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
struct Visibility {
    north: u8,
    south: u8,
    east: u8,
    west: u8,
}

impl Visibility {
    fn min(&self) -> u8 {
        u8::min(u8::min(self.north, self.south), u8::min(self.east, self.west))
    }
}

impl Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.min())
    }
}

fn solve_part_one(input: &Grid<u8>) -> usize {
    let width = input.width();
    let height = input.height();

    let mut visibility = Grid::<Visibility>::new(width, height);

    // north / south
    for x in 0..width {
        visibility.get_mut(x, 0).north = 0;
        visibility.get_mut(x, height - 1).south = 0;
    }
    for y in 1..height {
        for x in 0..width {
            visibility.get_mut(x, y).north = u8::max(*input.get(x, y - 1), visibility.get(x, y - 1).north);
            visibility.get_mut(x, height - y - 1).south = u8::max(*input.get(x, height - y), visibility.get(x, height - y).south);
        }
    }
    // east / west
    for y in 0..height {
        visibility.get_mut(0, y).west = 0;
        visibility.get_mut(width - 1, y).east = 0;
    }
    for x in 1..width {
        for y in 0..height {
            visibility.get_mut(x, y).west = u8::max(*input.get(x - 1, y), visibility.get(x - 1, y).west);
            visibility.get_mut(width - x - 1, y).east = u8::max(*input.get(width - x, y), visibility.get(width - x, y).east);
        }
    }

    let mut visible = 2 * width + 2 * height - 4;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if *input.get(x, y) > visibility.get(x, y).min() {
                visible += 1;
            }
        }
    }
    visible
}

fn solve_part_two(input: &Grid<u8>) -> usize {
    let width = input.width();
    let height = input.height();

    fn score_dir(input: &Grid<u8>, mut x: isize, mut y: isize, dx: isize, dy: isize) -> usize {
        let width = input.width() as isize;
        let height = input.height() as isize;

        let cap = *input.get(x as usize, y as usize);
        let mut count = 0;
        x += dx;
        y += dy;
        while x >= 0 && x < width && y >= 0 && y < height {
            count += 1;
            let next = *input.get(x as usize, y as usize);
            if next >= cap {
                break;
            }
            x += dx;
            y += dy;
        }

        count
    }

    fn score(input: &Grid<u8>, x: isize, y: isize) -> usize {
        let east = score_dir(input, x, y, 1, 0);
        let west = score_dir(input, x, y, -1, 0);
        let south = score_dir(input, x, y, 0, 1);
        let north = score_dir(input, x, y, 0, -1);
        east * west * south * north
    }

    let mut max_score = 0;
    for y in 0..height as isize {
        for x in 0..width as isize {
            max_score = usize::max(max_score, score(input, x, y));
        }
    }
    max_score
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

use std::{
    collections::HashSet,
    ops::{Add, Sub},
    str::FromStr,
};

use anyhow::{Context, Error};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Vec3i {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3i {
    fn min(self, other: Self) -> Self {
        Self {
            x: i32::min(self.x, other.x),
            y: i32::min(self.y, other.y),
            z: i32::min(self.z, other.z),
        }
    }

    fn max(self, other: Self) -> Self {
        Self {
            x: i32::max(self.x, other.x),
            y: i32::max(self.y, other.y),
            z: i32::max(self.z, other.z),
        }
    }

    fn volume(&self) -> i32 {
        self.x * self.y * self.z
    }

    fn index(&self, size: Vec3i) -> usize {
        (self.x + size.x * (self.y + size.y * self.z)) as usize
    }
}

impl Add for Vec3i {
    type Output = Vec3i;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3i {
    type Output = Vec3i;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FromStr for Vec3i {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(',');
        Ok(Self {
            x: pieces.next().context("missing x coordinate")?.parse()?,
            y: pieces.next().context("missing y coordinate")?.parse()?,
            z: pieces.next().context("missing z coordinate")?.parse()?,
        })
    }
}

const NEIGHBORS: [Vec3i; 6] = [
    Vec3i { x: -1, y: 0, z: 0 },
    Vec3i { x: 1, y: 0, z: 0 },
    Vec3i { x: 0, y: -1, z: 0 },
    Vec3i { x: 0, y: 1, z: 0 },
    Vec3i { x: 0, y: 0, z: -1 },
    Vec3i { x: 0, y: 0, z: 1 },
];

fn solve_part_one(input: &Vec<Vec3i>) -> usize {
    let mut total = 0;

    let mut positions = HashSet::new();
    for position in input {
        positions.insert(position);
        total += 6;
        for neighbor in NEIGHBORS {
            if positions.contains(&(*position + neighbor)) {
                total -= 2;
            }
        }
    }

    total
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    None,
    Lava,
    Steam,
}

fn solve_part_two(input: &Vec<Vec3i>) -> i32 {
    let lower = input.iter().cloned().reduce(Vec3i::min).unwrap();
    let upper = input.iter().cloned().reduce(Vec3i::max).unwrap();
    let size = upper - lower + Vec3i { x: 1, y: 1, z: 1 };

    let mut grid = vec![Cell::None; size.volume() as usize];

    for i in input {
        let index = (*i - lower).index(size);
        grid[index] = Cell::Lava;
    }

    let mut done = false;
    while !done {
        done = true;

        for x in 0..size.x {
            for y in 0..size.y {
                for z in 0..size.z {
                    let local_pos = Vec3i { x, y, z };
                    let index = local_pos.index(size);
                    if grid[index] == Cell::None {
                        for neighbor in NEIGHBORS {
                            let neighbor_pos = local_pos + neighbor;
                            if neighbor_pos.x < 0
                                || neighbor_pos.x >= size.x
                                || neighbor_pos.y < 0
                                || neighbor_pos.y >= size.y
                                || neighbor_pos.z < 0
                                || neighbor_pos.z >= size.z
                                || grid[neighbor_pos.index(size)] == Cell::Steam
                            {
                                grid[index] = Cell::Steam;
                                done = false;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut total = 0;
    for pos in input {
        let local_pos = *pos - lower;
        for neighbor in NEIGHBORS {
            let neighbor_pos = local_pos + neighbor;
            if neighbor_pos.x < 0
                || neighbor_pos.x >= size.x
                || neighbor_pos.y < 0
                || neighbor_pos.y >= size.y
                || neighbor_pos.z < 0
                || neighbor_pos.z >= size.z
                || grid[neighbor_pos.index(size)] == Cell::Steam
            {
                total += 1;
            }
        }
    }

    total
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

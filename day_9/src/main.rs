use anyhow::{bail, Context, Error, Result};
use std::{collections::HashSet, ops::Sub, str::FromStr};

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Self::Right,
            "U" => Self::Up,
            "L" => Self::Left,
            "D" => Self::Down,
            x => bail!("Invalid direction: {}", x),
        })
    }
}

struct Instruction {
    direction: Direction,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        Ok(Self {
            direction: pieces.next().context("Expected direction")?.parse()?,
            amount: pieces.next().context("Expected amount")?.parse()?,
        })
    }
}

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn add(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Down => self.y -= 1,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Rope<const N: usize> {
    knots: [Vec2; N],
}

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self {
            knots: [Vec2 { x: 0, y: 0 }; N],
        }
    }
}

impl<const N: usize> Rope<N> {
    fn move_head(&mut self, direction: Direction) {
        self.knots[0].add(direction);

        for i in 1..N {
            let delta = self.knots[i - 1] - self.knots[i];
            if delta.x.abs() > 1 || delta.y.abs() > 1 {
                self.knots[i].x += delta.x.signum();
                self.knots[i].y += delta.y.signum();
            }
        }
    }

    fn tail(&self) -> Vec2 {
        self.knots[N - 1]
    }
}

fn simulate<const N: usize>(instructions: &[Instruction]) -> usize {
    let mut rope = Rope::<N>::default();
    let mut visited = HashSet::new();

    visited.insert(rope.tail());

    for instruction in instructions {
        for _ in 0..instruction.amount {
            rope.move_head(instruction.direction);
            visited.insert(rope.tail());
        }
    }

    visited.len()
}

fn solve_part_one(input: &Vec<Instruction>) -> usize {
    simulate::<2>(input.as_slice())
}

fn solve_part_two(input: &Vec<Instruction>) -> usize {
    simulate::<10>(input.as_slice())
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

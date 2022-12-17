use std::{collections::HashMap, io::BufRead};

use anyhow::{bail, Result};
use solve::Input;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn push(self) -> i64 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}

struct Pattern {
    directions: Vec<Direction>,
}

impl Input for Pattern {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut directions = Vec::new();
        for b in reader.bytes() {
            directions.push(match b? {
                b'<' => Direction::Left,
                b'>' => Direction::Right,
                b'\n' => break,
                x => bail!("expected > or <, found {x}"),
            });
        }
        Ok(Self { directions })
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum RockKind {
    Dash,
    Plus,
    L,
    I,
    Square,
}

impl RockKind {
    fn width(self) -> i64 {
        match self {
            Self::Dash => 4,
            Self::Plus => 3,
            Self::L => 3,
            Self::I => 1,
            Self::Square => 2,
        }
    }

    fn height(self) -> i64 {
        match self {
            Self::Dash => 1,
            Self::Plus => 3,
            Self::L => 3,
            Self::I => 4,
            Self::Square => 2,
        }
    }

    fn is_set(self, x: i64, y: i64) -> bool {
        match self {
            Self::Dash | Self::I | Self::Square => true,
            Self::Plus => x == 1 || y == 1,
            Self::L => x == 2 || y == 2,
        }
    }
}

struct Rock {
    x: i64,
    y: i64,
    kind: RockKind,
}

impl Rock {
    fn overlap(&self, grid: &Grid) -> bool {
        for x in 0..self.kind.width() {
            for y in 0..self.kind.height() {
                if self.kind.is_set(x, y) && grid.get(self.x + x, self.y - y) {
                    return true;
                }
            }
        }
        false
    }

    fn blit(&self, grid: &mut Grid) {
        for x in 0..self.kind.width() {
            for y in 0..self.kind.height() {
                if self.kind.is_set(x, y) {
                    grid.set(self.x + x, self.y - y);
                }
            }
        }
    }
}

struct Grid {
    cells: [u8; Self::BUFFER_SIZE as usize],
    floor: u64,
}

impl Grid {
    const BUFFER_SIZE: u64 = 64;

    fn new() -> Self {
        Grid {
            cells: [0u8; Self::BUFFER_SIZE as usize],
            floor: 0,
        }
    }

    fn get(&self, x: i64, y: i64) -> bool {
        if x < 0 || x >= 7 || y < self.floor as i64 {
            true
        } else {
            let index = y as u64 % Self::BUFFER_SIZE as u64;
            self.cells[index as usize] & (1 << x) != 0
        }
    }

    fn set(&mut self, x: i64, y: i64) {
        let index = y as u64 % Self::BUFFER_SIZE as u64;
        self.cells[index as usize] |= 1 << x;
    }

    fn reserve_to(&mut self, height: i64) {
        let height = height as u64;
        if self.floor + Self::BUFFER_SIZE < height {
            for y in self.floor..height - Self::BUFFER_SIZE {
                let index = y % Self::BUFFER_SIZE as u64;
                self.cells[index as usize] = 0;
            }
            self.floor = height - Self::BUFFER_SIZE;
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    cells: [u8; Grid::BUFFER_SIZE as usize],
    pattern_index: usize,
    kind: RockKind,
}

fn simulate(pattern: &Pattern, count: u64) -> i64 {
    let mut grid = Grid::new();
    let mut kind = RockKind::Dash;
    let mut pattern_index = 0;

    let mut seen = HashMap::new();

    let mut highest_rock_pos = -1;
    let mut i = 0;
    while i < count {
        let state = State {
            cells: grid.cells,
            pattern_index,
            kind,
        };
        if let Some((prev_highest_rock_pos, prev_i)) = seen.insert(state, (highest_rock_pos, i)) {
            let rocks_dropped = i - prev_i;
            let height_change = highest_rock_pos - prev_highest_rock_pos;

            let advance = (count - i) / rocks_dropped;

            i += advance * rocks_dropped;
            highest_rock_pos += advance as i64 * height_change;
            grid.floor += advance * height_change as u64;
        }

        let mut rock = Rock {
            x: 2,
            y: highest_rock_pos + kind.height() + 3,
            kind,
        };

        kind = match kind {
            RockKind::Dash => RockKind::Plus,
            RockKind::Plus => RockKind::L,
            RockKind::L => RockKind::I,
            RockKind::I => RockKind::Square,
            RockKind::Square => RockKind::Dash,
        };

        loop {
            let push = pattern.directions[pattern_index].push();
            pattern_index = (pattern_index + 1) % pattern.directions.len();

            rock.x += push;
            if rock.overlap(&grid) {
                rock.x -= push;
            }
            rock.y -= 1;
            if rock.overlap(&grid) {
                rock.y += 1;
                break;
            }
        }

        rock.blit(&mut grid);
        highest_rock_pos = i64::max(rock.y, highest_rock_pos);
        grid.reserve_to(highest_rock_pos + kind.height() + 4);
        i += 1;
    }

    highest_rock_pos + 1
}

fn solve_part_one(pattern: &Pattern) -> i64 {
    simulate(pattern, 2022)
}

fn solve_part_two(pattern: &Pattern) -> i64 {
    simulate(pattern, 1_000_000_000_000)
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

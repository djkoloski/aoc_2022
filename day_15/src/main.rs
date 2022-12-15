use std::{collections::HashSet, env, ops::Sub, str::FromStr};

use anyhow::{Context, Error};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Vec2i {
    x: i32,
    y: i32,
}

impl Vec2i {
    fn manhattan_len(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Sub for Vec2i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl FromStr for Vec2i {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(", ");
        Ok(Self {
            x: pieces
                .next()
                .context("expected x coordinate")?
                .strip_prefix("x=")
                .context("expected x prefix")?
                .parse()?,
            y: pieces
                .next()
                .context("expected y coordinate")?
                .strip_prefix("y=")
                .context("expected y prefix")?
                .parse()?,
        })
    }
}

struct Sensor {
    position: Vec2i,
    nearest_beacon: Vec2i,
}

impl Sensor {
    fn covered_range_at(&self, y: i32) -> Option<(i32, i32)> {
        let covered_radius = (self.nearest_beacon - self.position).manhattan_len();
        let distance = (y - self.position.y).abs();
        let range_radius = covered_radius - distance;
        if range_radius < 0 {
            None
        } else {
            Some((
                self.position.x - range_radius,
                self.position.x + range_radius,
            ))
        }
    }
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s
            .strip_prefix("Sensor at ")
            .context("expected prefix")?
            .split(": closest beacon is at ");
        Ok(Self {
            position: pieces.next().context("expected sensor position")?.parse()?,
            nearest_beacon: pieces.next().context("expected closest beacon")?.parse()?,
        })
    }
}

fn solve_part_one(input: &Vec<Sensor>) -> i32 {
    let row = env::args()
        .nth(2)
        .as_deref()
        .map(FromStr::from_str)
        .transpose()
        .unwrap()
        .unwrap_or(2_000_000);

    let beacons = input
        .iter()
        .filter_map(|s| {
            if s.nearest_beacon.y == row {
                Some(s.nearest_beacon.x)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    let mut covered_ranges = input
        .iter()
        .filter_map(|s| s.covered_range_at(row))
        .collect::<Vec<_>>();
    covered_ranges.sort();

    let mut total = 0;
    let mut active_range = Option::<(i32, i32)>::None;
    for range in covered_ranges {
        if let Some(active) = active_range {
            if range.0 <= active.1 + 1 {
                active_range = Some((active.0, i32::max(active.1, range.1)));
            } else {
                total += active.1 - active.0 + 1
                    - beacons
                        .iter()
                        .filter(|&&x| x >= active.0 && x <= active.1)
                        .count() as i32;
                active_range = Some(range);
            }
        } else {
            active_range = Some(range);
        }
    }
    if let Some(active) = active_range {
        total += active.1 - active.0 + 1
            - beacons
                .iter()
                .filter(|&&x| x >= active.0 && x <= active.1)
                .count() as i32;
    }
    total
}

fn solve_part_two(input: &Vec<Sensor>) -> u64 {
    let search_space = env::args()
        .nth(3)
        .as_deref()
        .map(FromStr::from_str)
        .transpose()
        .unwrap()
        .unwrap_or(4_000_000);

    for row in 0..search_space {
        let mut covered_ranges = input
            .iter()
            .filter_map(|s| s.covered_range_at(row))
            .filter(|r| r.1 >= 0 && r.0 <= search_space)
            .collect::<Vec<_>>();
        covered_ranges.sort();

        let mut last_range = None;
        let mut active_range = Option::<(i32, i32)>::None;
        for range in covered_ranges {
            if let Some(active) = active_range {
                if range.0 <= active.1 + 1 {
                    active_range = Some((active.0, i32::max(active.1, range.1)));
                } else {
                    last_range = Some(active);
                    active_range = Some(range);
                }
            } else {
                active_range = Some(range);
            }
        }
        if last_range.is_some() {
            return (last_range.unwrap().1 + 1) as u64 * 4_000_000u64 + row as u64;
        }
    }

    0
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

#[cfg(test)]
mod tests {
    use crate::{Sensor, Vec2i};

    #[test]
    fn test_covered_range() {
        let sensor = Sensor {
            position: Vec2i { x: 8, y: 7 },
            nearest_beacon: Vec2i { x: 2, y: 10 },
        };
        assert_eq!(sensor.covered_range_at(-3), None);
        assert_eq!(sensor.covered_range_at(-2), Some((8, 8)));
        assert_eq!(sensor.covered_range_at(7), Some((-1, 17)));
        assert_eq!(sensor.covered_range_at(10), Some((2, 14)));
        assert_eq!(sensor.covered_range_at(14), Some((6, 10)));
        assert_eq!(sensor.covered_range_at(22), None);
    }
}

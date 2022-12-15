use std::{collections::HashSet, ops::Add, str::FromStr};

use anyhow::{Context, Error};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const DOWN: Self = Point { x: 0, y: 1 };
    const DOWN_LEFT: Self = Point { x: -1, y: 1 };
    const DOWN_RIGHT: Self = Point { x: 1, y: 1 };

    pub fn min(a: Self, b: Self) -> Point {
        Point {
            x: i32::min(a.x, b.x),
            y: i32::min(a.y, b.y),
        }
    }

    pub fn max(a: Self, b: Self) -> Point {
        Point {
            x: i32::max(a.x, b.x),
            y: i32::max(a.y, b.y),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(",").map(|p| p.parse());
        Ok(Self {
            x: pieces.next().context("expected X coordinate")??,
            y: pieces.next().context("expected Y coordinate")??,
        })
    }
}

struct Rect {
    lower: Point,
    upper: Point,
}

impl Rect {
    fn contains(&self, point: Point) -> bool {
        self.could_contain(point) && self.lower.y <= point.y
    }

    fn could_contain(&self, point: Point) -> bool {
        self.lower.x <= point.x && self.upper.x >= point.x && self.upper.y >= point.y
    }

    fn merge(self, other: Rect) -> Rect {
        Rect {
            lower: Point::min(self.lower, other.lower),
            upper: Point::max(self.upper, other.upper),
        }
    }
}

struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn bounding_box(&self) -> Option<Rect> {
        Some(Rect {
            lower: self.points.iter().cloned().reduce(Point::min)?,
            upper: self.points.iter().cloned().reduce(Point::max)?,
        })
    }

    fn overlaps(&self, point: Point) -> bool {
        self.points
            .iter()
            .zip(self.points.iter().skip(1))
            .any(|(a, b)| {
                let bounding_box = Rect {
                    lower: Point::min(*a, *b),
                    upper: Point::max(*a, *b),
                };
                bounding_box.contains(point)
            })
    }
}

impl FromStr for Polyline {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            points: s
                .split(" -> ")
                .map(|p| p.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn try_move(
    point: Point,
    sand: &HashSet<Point>,
    lines: &[Polyline],
    floor: Option<i32>,
) -> Option<Point> {
    if sand.contains(&point) {
        None
    } else if lines.iter().any(|l| l.overlaps(point)) {
        None
    } else if floor.is_some() && point.y >= floor.unwrap() {
        None
    } else {
        Some(point)
    }
}

fn solve_part_one(input: &Vec<Polyline>) -> usize {
    let bounds = input
        .iter()
        .map(|p| Polyline::bounding_box(p).unwrap())
        .reduce(Rect::merge)
        .unwrap();
    let mut sand = HashSet::new();

    loop {
        let mut drop = Point { x: 500, y: 0 };
        while bounds.could_contain(drop) {
            if let Some(next) = try_move(drop + Point::DOWN, &sand, &input, None) {
                drop = next;
            } else if let Some(next) = try_move(drop + Point::DOWN_LEFT, &sand, &input, None) {
                drop = next;
            } else if let Some(next) = try_move(drop + Point::DOWN_RIGHT, &sand, &input, None) {
                drop = next;
            } else {
                sand.insert(drop);
                break;
            }
        }
        if !bounds.could_contain(drop) {
            break;
        }
    }

    sand.len()
}

fn solve_part_two(input: &Vec<Polyline>) -> usize {
    let bounds = input
        .iter()
        .map(|p| Polyline::bounding_box(p).unwrap())
        .reduce(Rect::merge)
        .unwrap();
    let floor = Some(bounds.upper.y + 2);
    let mut sand = HashSet::new();

    while !sand.contains(&Point { x: 500, y: 0 }) {
        let mut drop = Point { x: 500, y: 0 };
        loop {
            if let Some(next) = try_move(drop + Point::DOWN, &sand, &input, floor) {
                drop = next;
            } else if let Some(next) = try_move(drop + Point::DOWN_LEFT, &sand, &input, floor) {
                drop = next;
            } else if let Some(next) = try_move(drop + Point::DOWN_RIGHT, &sand, &input, floor) {
                drop = next;
            } else {
                sand.insert(drop);
                break;
            }
        }
    }

    sand.len()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

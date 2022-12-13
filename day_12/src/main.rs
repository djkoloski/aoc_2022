use std::{collections::VecDeque, io::BufRead};

use anyhow::{Context, Result};

struct Input {
    start: (i32, i32),
    end: (i32, i32),
    grid: Vec<u8>,
    width: i32,
    height: i32,
}

impl Input {
    pub fn get(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            let index = x + y * self.width;
            Some(self.grid[index as usize])
        }
    }
}

impl solve::Input for Input {
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut start = None;
        let mut end = None;
        let mut grid = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in reader.lines() {
            let line = line?;
            let bytes = line.as_bytes();

            for (i, c) in bytes.iter().enumerate() {
                let elevation = match c {
                    b'S' => {
                        start = Some((i as i32, height));
                        0
                    }
                    b'E' => {
                        end = Some((i as i32, height));
                        25
                    }
                    _ => c - b'a',
                };
                grid.push(elevation)
            }

            width = bytes.len() as i32;
            height += 1;
        }

        Ok(Self {
            start: start.context("missing start")?,
            end: end.context("missing end")?,
            grid,
            width,
            height,
        })
    }
}

const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn solve_part_one(input: &Input) -> usize {
    let mut distance = vec![usize::MAX; input.grid.len()];

    let mut queue = VecDeque::new();
    queue.push_back((input.start, 0));

    while let Some(((x, y), dist)) = queue.pop_front() {
        let index = (x + y * input.width) as usize;
        if dist < distance[index] {
            distance[index] = dist;
            for (dx, dy) in NEIGHBORS {
                let nx = x + dx;
                let ny = y + dy;
                if let Some(e) = input.get(x + dx, y + dy) {
                    if e <= input.get(x, y).unwrap() + 1 {
                        queue.push_back(((nx, ny), dist + 1));
                    }
                }
            }
        }
    }

    let end_index = (input.end.0 + input.end.1 * input.width) as usize;
    distance[end_index]
}

fn solve_part_two(input: &Input) -> usize {
    let mut distance = vec![usize::MAX; input.grid.len()];

    let mut queue = VecDeque::new();
    queue.push_back((input.end, 0));

    while let Some(((x, y), dist)) = queue.pop_front() {
        let index = (x + y * input.width) as usize;
        if dist < distance[index] {
            distance[index] = dist;
            for (dx, dy) in NEIGHBORS {
                let nx = x + dx;
                let ny = y + dy;
                if let Some(e) = input.get(x + dx, y + dy) {
                    if e + 1 >= input.get(x, y).unwrap() {
                        queue.push_back(((nx, ny), dist + 1));
                    }
                }
            }
        }
    }

    *input
        .grid
        .iter()
        .zip(distance.iter())
        .filter_map(|(e, d)| (*e == 0).then(|| d))
        .min()
        .unwrap()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

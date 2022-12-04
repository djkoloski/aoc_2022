use std::str::FromStr;

use anyhow::{anyhow, Error};

struct Range {
    lower: i32,
    upper: i32,
}

impl Range {
    pub fn contains(&self, other: &Self) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.lower <= other.upper && self.upper >= other.lower
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split('-');
        Ok(Self {
            lower: values
                .next()
                .ok_or_else(|| anyhow!("Missing lower bound on range"))?
                .parse()?,
            upper: values
                .next()
                .ok_or_else(|| anyhow!("Missing upper bound on range"))?
                .parse()?,
        })
    }
}

struct RangePair {
    first: Range,
    second: Range,
}

impl FromStr for RangePair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = s.split(',');
        Ok(Self {
            first: pairs
                .next()
                .ok_or_else(|| anyhow!("Missing first range in pair"))?
                .parse()?,
            second: pairs
                .next()
                .ok_or_else(|| anyhow!("Missing second range in pair"))?
                .parse()?,
        })
    }
}

fn solve_part_one(input: &Vec<RangePair>) -> usize {
    input
        .iter()
        .filter(|pair| pair.first.contains(&pair.second) || pair.second.contains(&pair.first))
        .count()
}

fn solve_part_two(input: &Vec<RangePair>) -> usize {
    input
        .iter()
        .filter(|pair| pair.first.overlaps(&pair.second))
        .count()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

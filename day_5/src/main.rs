use std::str::FromStr;

use anyhow::{anyhow, Error};

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        assert_eq!(
            pieces
                .next()
                .ok_or_else(|| anyhow!("Missing \"move\" in instruction"))?,
            "move"
        );
        let amount = pieces
            .next()
            .ok_or_else(|| anyhow!("Missing amount in instruction"))?
            .parse()?;
        assert_eq!(
            pieces
                .next()
                .ok_or_else(|| anyhow!("Missing \"from\" in instruction"))?,
            "from"
        );
        let from = pieces
            .next()
            .ok_or_else(|| anyhow!("Missing from in instruction"))?
            .parse()?;
        assert_eq!(
            pieces
                .next()
                .ok_or_else(|| anyhow!("Missing \"to\" in instruction"))?,
            "to"
        );
        let to = pieces
            .next()
            .ok_or_else(|| anyhow!("Missing to in instruction"))?
            .parse()?;
        Ok(Self { amount, from: from, to })
    }
}

struct Input {
    stacks: Vec<Vec<u8>>,
    instructions: Vec<Instruction>,
}

impl solve::Input for Input {
    fn parse<R: std::io::BufRead>(reader: R) -> anyhow::Result<Self> {
        let mut lines = reader.lines();

        let mut stacks = Vec::new();
        'parse_stacks: loop {
            let line = lines.next().ok_or_else(|| anyhow!("Expected stack line"))??;
            let bytes = line.as_bytes();

            let stack_count = (bytes.len() + 1) / 4;
            for _ in stacks.len()..stack_count {
                stacks.push(Vec::new());
            }

            for i in 0..stack_count {
                let segment = &bytes[4 * i..4 * i + 3];
                if segment[0] == '[' as u8 {
                    stacks[i].push(segment[1]);
                } else if segment[1] != ' ' as u8 {
                    break 'parse_stacks;
                }
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }

        assert!(lines.next().ok_or_else(|| anyhow!("Expected empty line between stacks and instructions"))??.is_empty());

        let mut instructions = Vec::new();

        while let Some(line) = lines.next() {
            instructions.push(line?.parse()?);
        }

        Ok(Self {
            stacks,
            instructions,
        })
    }
}

fn solve_part_one(input: &Input) -> String {
    let mut stacks = input.stacks.clone();

    for instruction in input.instructions.iter() {
        for _ in 0..instruction.amount {
            let moved = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(moved);
        }
    }

    stacks.iter().map(|s| *s.last().unwrap() as char).collect()
}

fn solve_part_two(input: &Input) -> String {
    let mut stacks = input.stacks.clone();

    for instruction in input.instructions.iter() {
        let mut moved = Vec::new();
        for _ in 0..instruction.amount {
            moved.push(stacks[instruction.from - 1].pop().unwrap());
        }
        for _ in 0..instruction.amount {
            stacks[instruction.to - 1].push(moved.pop().unwrap());
        }
    }

    stacks.iter().map(|s| *s.last().unwrap() as char).collect()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

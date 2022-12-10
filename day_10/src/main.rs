use anyhow::{bail, Context, Error, Result};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        Ok(match pieces.next().context("Expected instruction")? {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(pieces.next().context("Expected amount")?.parse()?),
            _ => bail!("Unknown instruction"),
        })
    }
}

struct Machine {
    x: i32,
    pending: Option<i32>,
}

impl Machine {
    fn new() -> Self {
        Machine {
            x: 1,
            pending: None,
        }
    }

    fn step(&mut self, instructions: &mut impl Iterator<Item = Instruction>) {
        if let Some(n) = self.pending.take() {
            self.x += n;
        } else {
            if let Some(instruction) = instructions.next() {
                match instruction {
                    Instruction::Noop => (),
                    Instruction::Addx(n) => self.pending = Some(n),
                }
            }
        }
    }
}

fn solve_part_one(input: &Vec<Instruction>) -> i32 {
    let mut machine = Machine::new();
    let mut instructions = input.iter().cloned();

    let mut total = 0;
    for i in 0..220 {
        if (i + 21) % 40 == 0 {
            total += machine.x * (i as i32 + 1);
        }
        machine.step(&mut instructions);
    }
    total
}

fn solve_part_two(input: &Vec<Instruction>) -> String {
    let mut machine = Machine::new();
    let mut instructions = input.iter().cloned();

    let mut display = ['.'; 40 * 6];

    for i in 0..240 {
        let px = i % 40;
        if px >= machine.x - 1 && px <= machine.x + 1 {
            display[i as usize] = '#';
        }
        machine.step(&mut instructions);
    }

    let mut result = String::new();
    for i in 0..6 {
        result += "\n";
        result.extend(&display[40 * i..40 * (i + 1)]);
    }
    result
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

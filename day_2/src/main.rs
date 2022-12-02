use std::str::FromStr;

use anyhow::anyhow;

#[derive(Clone, Copy)]
enum Action {
    A,
    B,
    C,
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Action::A),
            "B" => Ok(Action::B),
            "C" => Ok(Action::C),
            _ => Err(anyhow!("Invalid action \"{}\"", s)),
        }
    }
}

#[derive(Clone, Copy)]
enum Response {
    X,
    Y,
    Z,
}

impl FromStr for Response {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Response::X),
            "Y" => Ok(Response::Y),
            "Z" => Ok(Response::Z),
            _ => Err(anyhow!("Invalid response \"{}\"", s)),
        }
    }
}

#[derive(Clone, Copy)]
struct Strategy {
    opponent: Action,
    response: Response,
}

impl Strategy {
    fn value_1(&self) -> i32 {
        const OUTCOME: [[i32; 3]; 3] = [
            [4, 8, 3],
            [1, 5, 9],
            [7, 2, 6],
        ];
        OUTCOME[self.opponent as usize][self.response as usize]
    }

    fn value_2(&self) -> i32 {
        const OUTCOME: [[i32; 3]; 3] = [
            [3, 4, 8],
            [1, 5, 9],
            [2, 6, 7],
        ];
        OUTCOME[self.opponent as usize][self.response as usize]
    }
}

impl FromStr for Strategy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut actions = s.split(' ');
        Ok(Self {
            opponent: actions.next().ok_or_else(|| anyhow!("Missing opponent action"))?.parse()?,
            response: actions.next().ok_or_else(|| anyhow!("Missing response action"))?.parse()?,
        })
    }
}

fn solve_part_one(input: &Vec<Strategy>) -> i32 {
    input.iter().map(|s| s.value_1()).sum()
}

fn solve_part_two(input: &Vec<Strategy>) -> i32 {
    input.iter().map(|s| s.value_2()).sum()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

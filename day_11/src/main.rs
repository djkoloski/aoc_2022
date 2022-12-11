use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};
use solve::Input;

type Int = u64;

#[derive(Clone)]
enum Operand {
    Old,
    Imm(Int),
}

impl FromStr for Operand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Self::Old,
            x => Self::Imm(x.parse()?),
        })
    }
}

impl Operand {
    fn evaluate(&self, old: Int) -> Int {
        match self {
            Self::Old => old,
            Self::Imm(x) => *x,
        }
    }
}

#[derive(Clone)]
enum Operator {
    Add,
    Mul,
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            x => bail!("unexpected operator: {}", x),
        })
    }
}

impl Operator {
    fn evaluate(&self, left: Int, right: Int) -> Int {
        match self {
            Self::Add => left + right,
            Self::Mul => left * right,
        }
    }
}

#[derive(Clone)]
struct Expression {
    left: Operand,
    op: Operator,
    right: Operand,
}

impl FromStr for Expression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        Ok(Self {
            left: pieces.next().context("expected left operand")?.parse()?,
            op: pieces.next().context("expected operator")?.parse()?,
            right: pieces.next().context("expected right operand")?.parse()?,
        })
    }
}

impl Expression {
    fn evaluate(&self, old: Int) -> Int {
        self.op
            .evaluate(self.left.evaluate(old), self.right.evaluate(old))
    }
}

#[derive(Clone, Copy)]
enum Reduction {
    Divide,
    Modulo,
}

impl Reduction {
    fn apply(&self, value: Int, amount: Int) -> Int {
        match self {
            Reduction::Divide => value / amount,
            Reduction::Modulo => value % amount,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Int>,
    expression: Expression,
    test_divisible_by: Int,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    fn parse(lines: &mut impl Iterator<Item = std::io::Result<String>>) -> Result<Self> {
        let items = lines
            .next()
            .context("unexpected end of input")??
            .strip_prefix("  Starting items: ")
            .context("mismatched starting items prefix")?
            .split(", ")
            .map(Int::from_str)
            .collect::<Result<_, _>>()?;
        let expression = lines
            .next()
            .context("unexpected end of input")??
            .strip_prefix("  Operation: new = ")
            .context("mismatched operation prefix")?
            .parse()?;
        let test_divisible_by = lines
            .next()
            .context("unexpected end of input")??
            .strip_prefix("  Test: divisible by ")
            .context("mismatched test prefix")?
            .parse()?;
        let test_true = lines
            .next()
            .context("unexpected end of input")??
            .strip_prefix("    If true: throw to monkey ")
            .context("mismatched if true prefix")?
            .parse()?;
        let test_false = lines
            .next()
            .context("unexpected end of input")??
            .strip_prefix("    If false: throw to monkey ")
            .context("mismatched if false prefix")?
            .parse()?;

        Ok(Self {
            items,
            expression,
            test_divisible_by,
            test_true,
            test_false,
        })
    }

    fn step(&mut self, reduction: Reduction, amount: Int) -> Option<(Int, usize)> {
        self.items.pop().map(|old| {
            let value = reduction.apply(self.expression.evaluate(old), amount);
            let target = if value % self.test_divisible_by == 0 {
                self.test_true
            } else {
                self.test_false
            };
            (value, target)
        })
    }
}

#[derive(Clone)]
struct State {
    monkeys: Vec<Monkey>,
}

impl Input for State {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut monkeys = Vec::new();
        let mut lines = reader.lines();
        while let Some(line) = lines.next() {
            let id = line?
                .strip_prefix("Monkey ")
                .context("mismatched monkey id prefix")?
                .strip_suffix(':')
                .context("mismatched monkey id suffix")?
                .parse::<usize>()?;
            if id != monkeys.len() {
                bail!("expected id {}, found id {}", monkeys.len(), id);
            }
            monkeys.push(Monkey::parse(&mut lines)?);
            let _ = lines.next();
        }

        Ok(Self { monkeys })
    }
}

fn gcd(mut a: Int, mut b: Int) -> Int {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: Int, b: Int) -> Int {
    a * b / gcd(a, b)
}

impl State {
    fn step(&mut self, reduction: Reduction) -> Vec<usize> {
        let amount = match reduction {
            Reduction::Divide => 3,
            Reduction::Modulo => self
                .monkeys
                .iter()
                .fold(1, |x, monkey| lcm(x, monkey.test_divisible_by)),
        };

        let mut stats = Vec::new();
        for i in 0..self.monkeys.len() {
            let mut inspections = 0;
            while let Some((item, to)) = self.monkeys[i].step(reduction, amount) {
                inspections += 1;
                self.monkeys[to].items.push(item);
            }
            stats.push(inspections);
        }
        stats
    }
}

fn simulate(input: &State, reduction: Reduction, rounds: usize) -> usize {
    let mut state = input.clone();
    let mut stats = vec![0; input.monkeys.len()];

    for _ in 0..rounds {
        let new_stats = state.step(reduction);
        for (stat, new_stat) in stats.iter_mut().zip(new_stats.iter()) {
            *stat += new_stat;
        }
    }

    stats.sort();
    stats[stats.len() - 2..].iter().product()
}

fn solve_part_one(input: &State) -> usize {
    simulate(input, Reduction::Divide, 20)
}

fn solve_part_two(input: &State) -> usize {
    simulate(input, Reduction::Modulo, 10000)
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

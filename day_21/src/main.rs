use std::{str::FromStr, collections::HashMap, ops::{Add, Sub, Mul, Div}};

use anyhow::{Error, Context};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Name([u8; 4]);

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Name(s.as_bytes().try_into()?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rational {
    n: i64,
    d: u64,
}

impl Rational {
    fn reduce(n: i128, d: u128) -> Self {
        let gcd = gcd(n.abs() as u128, d);
        Self {
            n: (n / gcd as i128) as i64,
            d: (d / gcd) as u64,
        }
    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::reduce(
            self.n as i128 * rhs.d as i128 + rhs.n as i128 * self.d as i128,
            self.d as u128 * rhs.d as u128,
        )
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::reduce(
            self.n as i128 * rhs.d as i128 - rhs.n as i128 * self.d as i128,
            self.d as u128 * rhs.d as u128,
        )
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::reduce(
            self.n as i128 * rhs.n as i128,
            self.d as u128 * rhs.d as u128,
        )
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::reduce(
            self.n as i128 * rhs.d as i128 * rhs.n.signum() as i128,
            self.d as u128 * rhs.n.abs() as u128,
        )
    }
}

#[derive(Clone)]
enum Expr {
    Value(i64),
    Add(Name, Name),
    Sub(Name, Name),
    Mul(Name, Name),
    Div(Name, Name),
}

impl FromStr for Expr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('+') {
            let mut pieces = s.split(" + ");
            Ok(Expr::Add(
                pieces.next().context("expected first operand")?.parse()?,
                pieces.next().context("expected second operand")?.parse()?,
            ))
        } else if s.contains('-') {
            let mut pieces = s.split(" - ");
            Ok(Expr::Sub(
                pieces.next().context("expected first operand")?.parse()?,
                pieces.next().context("expected second operand")?.parse()?,
            ))
        } else if s.contains('*') {
            let mut pieces = s.split(" * ");
            Ok(Expr::Mul(
                pieces.next().context("expected first operand")?.parse()?,
                pieces.next().context("expected second operand")?.parse()?,
            ))
        } else if s.contains('/') {
            let mut pieces = s.split(" / ");
            Ok(Expr::Div(
                pieces.next().context("expected first operand")?.parse()?,
                pieces.next().context("expected second operand")?.parse()?,
            ))
        } else {
            return Ok(Expr::Value(s.parse()?));
        }
    }
}

impl Expr {
    fn eval(&self, monkeys: &HashMap<Name, Expr>) -> i64 {
        match self {
            Expr::Value(x) => *x,
            Expr::Add(x, y) => monkeys[x].eval(monkeys) + monkeys[y].eval(monkeys),
            Expr::Sub(x, y) => monkeys[x].eval(monkeys) - monkeys[y].eval(monkeys),
            Expr::Mul(x, y) => monkeys[x].eval(monkeys) * monkeys[y].eval(monkeys),
            Expr::Div(x, y) => monkeys[x].eval(monkeys) / monkeys[y].eval(monkeys),
        }
    }

    fn eval_or_humn(monkeys: &HashMap<Name, Expr>, name: &Name) -> Linear {
        if &name.0 == b"humn" {
            Linear {
                m: Rational { n: 1, d: 1 },
                b: Rational { n: 0, d: 1 },
            }
        } else {
            monkeys[&name].eval_linear(monkeys)
        }
    }

    fn eval_linear(&self, monkeys: &HashMap<Name, Expr>) -> Linear {
        match self {
            Expr::Value(x) => Linear { m: Rational { n: 0, d: 1 }, b: Rational { n: *x, d: 1 } },
            Expr::Add(x, y) => Self::eval_or_humn(monkeys, x) + Self::eval_or_humn(monkeys, y),
            Expr::Sub(x, y) => Self::eval_or_humn(monkeys, x) - Self::eval_or_humn(monkeys, y),
            Expr::Mul(x, y) => Self::eval_or_humn(monkeys, x) * Self::eval_or_humn(monkeys, y),
            Expr::Div(x, y) => Self::eval_or_humn(monkeys, x) / Self::eval_or_humn(monkeys, y),
        }
    }
}

#[derive(Debug)]
struct Linear {
    m: Rational,
    b: Rational,
}

impl Add for Linear {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            m: self.m + rhs.m,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Linear {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            m: self.m - rhs.m,
            b: self.b - rhs.b,
        }
    }
}

impl Mul for Linear {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.m.n == 0 || rhs.m.n == 0);
        Self {
            m: self.m * rhs.b + self.b * rhs.m,
            b: self.b * rhs.b,
        }
    }
}

impl Div for Linear {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs.m.n == 0);
        Self {
            m: self.m / rhs.b,
            b: self.b / rhs.b,
        }
    }
}

struct Monkey {
    name: Name,
    expr: Expr,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(": ");
        Ok(Monkey {
            name: pieces.next().context("expected monkey name")?.parse()?,
            expr: pieces.next().context("expected monkey expression")?.parse()?,
        })
    }
}

fn solve_part_one(input: &Vec<Monkey>) -> i64 {
    let mut monkeys = HashMap::new();

    for monkey in input {
        monkeys.insert(monkey.name, monkey.expr.clone());
    }

    monkeys[&Name(*b"root")].eval(&monkeys)
}

fn solve_part_two(input: &Vec<Monkey>) -> i64 {
    let mut monkeys = HashMap::new();

    for monkey in input {
        monkeys.insert(monkey.name, monkey.expr.clone());
    }

    let (left, right) = match monkeys[&Name(*b"root")] {
        Expr::Add(x, y) => (x, y),
        Expr::Sub(x, y) => (x, y),
        Expr::Mul(x, y) => (x, y),
        Expr::Div(x, y) => (x, y),
        _ => panic!("expected the root monkey to have a binary operation"),
    };

    let l = monkeys[&left].eval_linear(&monkeys);
    let r = monkeys[&right].eval_linear(&monkeys);

    assert_eq!(r.m.n, 0);
    let result = (r.b - l.b) / l.m;
    assert_eq!(result.d, 1);

    result.n
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

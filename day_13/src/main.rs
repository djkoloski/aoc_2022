use std::{cmp::Ordering, io::BufRead, iter::Peekable, slice};

use anyhow::{bail, Context, Error};

trait Munch: Sized {
    type Error;

    fn munch<I>(chars: &mut Peekable<I>) -> Result<Self, Self::Error>
    where
        I: Iterator<Item = char>;

    fn munch_str(str: &str) -> Result<Self, Self::Error> {
        Self::munch(&mut str.chars().peekable())
    }
}

impl Munch for i32 {
    type Error = Error;

    fn munch<I>(chars: &mut Peekable<I>) -> Result<Self, Self::Error>
    where
        I: Iterator<Item = char>,
    {
        let mut result = 0;
        let mut parsed = false;
        while let Some(c @ '0'..='9') = chars.peek() {
            let c = *c;
            chars.next();
            result = result * 10 + (c as i32 - '0' as i32);
            parsed = true;
        }
        if !parsed {
            bail!("failed to parse integer");
        }
        Ok(result)
    }
}

#[derive(Clone, Eq, Ord, PartialEq)]
enum Value<T> {
    Literal(T),
    List(Vec<Value<T>>),
}

impl<T: Munch> Munch for Value<T>
where
    Error: From<T::Error>,
{
    type Error = Error;

    fn munch<I>(chars: &mut Peekable<I>) -> Result<Self, Self::Error>
    where
        I: Iterator<Item = char>,
    {
        match chars
            .peek()
            .context("expected chars to parse a `Value` from")?
        {
            '[' => {
                chars.next();
                let mut values = Vec::new();
                loop {
                    if *chars.peek().context("expected closed list")? == ']' {
                        chars.next();
                        break;
                    }
                    values.push(Value::munch(chars)?);
                    if *chars.peek().context("expected separator char")? == ',' {
                        chars.next();
                    }
                }
                Ok(Self::List(values))
            }
            _ => Ok(Self::Literal(T::munch(chars)?)),
        }
    }
}

impl<T: PartialOrd> PartialOrd for Value<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Literal(l), Value::Literal(r)) => l.partial_cmp(r),
            (Value::Literal(_), Value::List(r)) => slice::from_ref(self).partial_cmp(&**r),
            (Value::List(l), Value::Literal(_)) => (&**l).partial_cmp(slice::from_ref(other)),
            (Value::List(l), Value::List(r)) => l.partial_cmp(r),
        }
    }
}

struct Input {
    pairs: Vec<(Value<i32>, Value<i32>)>,
}

impl solve::Input for Input {
    fn parse<R: BufRead>(reader: R) -> Result<Self, Error> {
        let mut lines = reader.lines();
        let mut pairs = Vec::new();

        while let Some(left) = lines.next() {
            let right = lines.next().context("expected pair")?;
            pairs.push((Value::munch_str(&left?)?, Value::munch_str(&right?)?));
            lines.next();
        }

        Ok(Input { pairs })
    }
}

fn solve_part_one(input: &Input) -> usize {
    input
        .pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (l, r))| (l < r).then(|| i + 1))
        .sum()
}

fn solve_part_two(input: &Input) -> usize {
    let mut packets = input
        .pairs
        .iter()
        .cloned()
        .fold(Vec::new(), |mut packets, (l, r)| {
            packets.push(l);
            packets.push(r);
            packets
        });
    let dividers = [
        Value::List(vec![Value::List(vec![Value::Literal(2)])]),
        Value::List(vec![Value::List(vec![Value::Literal(6)])]),
    ];
    for divider in dividers.iter() {
        packets.push(divider.clone());
    }
    packets.sort();

    dividers
        .iter()
        .map(|d| packets.iter().position(|p| p == d).unwrap() + 1)
        .product()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

use anyhow::{anyhow, Context, Error, Result};
use std::{
    env,
    fmt::{self, Display},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::{from_utf8, FromStr},
    time::{Duration, Instant},
};

pub trait Input: Sized {
    fn parse<R: BufRead>(reader: R) -> Result<Self>;
}

impl Input for String {
    fn parse<R: BufRead>(mut reader: R) -> Result<Self> {
        let mut result = String::new();
        reader.read_to_string(&mut result)?;
        Ok(result)
    }
}

impl<T: FromStr> Input for Vec<T>
where
    T::Err: Display,
{
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        reader
            .lines()
            .enumerate()
            .map(|(line_number, line)| {
                T::from_str(&line.context("Failed to read line")?)
                    .map_err(|e| anyhow!("Failed to parse line {}: {}", line_number + 1, e))
            })
            .collect()
    }
}

pub struct Unimplemented;

impl Display for Unimplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "unimplemented")
    }
}

pub struct CSV<T> {
    pub values: Vec<T>,
}

impl<T: FromStr> FromStr for CSV<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s.as_bytes())
    }
}

impl<T: FromStr> Input for CSV<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let values = reader
            .split(b',')
            .map(|x| Ok(from_utf8(&x?)?.parse()?))
            .collect::<Result<_, Error>>()?;
        Ok(Self { values })
    }
}

pub struct Grouped<T> {
    pub groups: Vec<Vec<T>>,
}

impl<T: FromStr> Input for Grouped<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        let mut groups = vec![Vec::new()];
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                groups.push(Vec::new());
            } else {
                groups.last_mut().unwrap().push(line.parse()?);
            }
        }
        Ok(Grouped { groups })
    }
}

pub struct Solution<T> {
    result: T,
    duration: Duration,
}

impl<T: Display> Display for Solution<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Solution: {}", self.result)?;
        writeln!(f, "  Elapsed:  {} seconds", self.duration.as_secs_f64())?;
        Ok(())
    }
}

fn time_solve<F: FnOnce() -> T, T>(f: F) -> Solution<T> {
    let start = Instant::now();
    let result = f();
    let duration = Instant::now().duration_since(start);
    Solution { result, duration }
}

pub type SolveResult<P1, P2> = Result<(Solution<P1>, Solution<P2>)>;

pub fn solve<I, P1, P2>(
    path: &Path,
    solve_part_one: impl FnOnce(&I) -> P1,
    solve_part_two: impl FnOnce(&I) -> P2,
) -> SolveResult<P1, P2>
where
    I: Input,
    P1: Display,
    P2: Display,
{
    let input_file = BufReader::new(File::open(path).context("Failed to open input file")?);
    let input = Input::parse(input_file).context("Failed to parse input")?;

    Ok((
        time_solve(|| solve_part_one(&input)),
        time_solve(|| solve_part_two(&input)),
    ))
}

pub fn main<I, P1, P2>(solve_part_one: impl FnOnce(&I) -> P1, solve_part_two: impl FnOnce(&I) -> P2)
where
    I: Input,
    P1: Display,
    P2: Display,
{
    let bin_path = env::args().nth(0).expect("missing binary path");
    let path = env::args().nth(1).unwrap_or_else(|| {
        let path = Path::new(&bin_path);
        let day = path.file_stem().unwrap().to_str().unwrap();
        format!("{day}/test.input")
    });
    println!("opening {path}");
    let (part_one, part_two) =
        solve(path.as_ref(), solve_part_one, solve_part_two).expect("failed to solve problem");

    println!("Part one:");
    println!("{}", part_one);
    println!("Part two:");
    println!("{}", part_two);
}

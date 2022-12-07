use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Context, Error};

pub enum Line {
    Cd(String),
    CdOut,
    CdRoot,
    Ls,
    Dir(String),
    File(usize, String),
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        let first = pieces.next().context("Unexpected empty line")?;
        match first {
            "$" => {
                let command = pieces.next().context("Expected command after '$'")?;
                match command {
                    "cd" => {
                        let target = pieces.next().context("Expected cd target")?;
                        match target {
                            "/" => Ok(Line::CdRoot),
                            ".." => Ok(Line::CdOut),
                            x => Ok(Line::Cd(x.to_string())),
                        }
                    }
                    "ls" => Ok(Line::Ls),
                    x => Err(anyhow!("Unrecognized command '{}'", x)),
                }
            }
            "dir" => Ok(Line::Dir(
                pieces
                    .next()
                    .context("Expected directory name after 'dir'")?
                    .to_string(),
            )),
            size => Ok(Line::File(
                size.parse()?,
                pieces
                    .next()
                    .context("Expected file name after size")?
                    .to_string(),
            )),
        }
    }
}

pub struct FileSystem {
    directories: Vec<Directory>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            directories: vec![Directory::new(0)],
        }
    }

    pub fn from_lines<'a>(lines: impl Iterator<Item = &'a Line>) -> Self {
        let mut result = Self::new();
        let mut dir = 0;

        for line in lines {
            match line {
                Line::Cd(path) => dir = result.directories[dir].subdirs[path],
                Line::CdOut => dir = result.directories[dir].parent,
                Line::CdRoot => dir = 0,
                Line::Ls => (),
                Line::Dir(path) => {
                    let new_dir = result.directories.len();
                    result.directories.push(Directory::new(dir));
                    result.directories[dir]
                        .subdirs
                        .insert(path.clone(), new_dir);
                }
                Line::File(size, path) => {
                    result.directories[dir].files.insert(path.clone(), *size);
                }
            }
        }

        result
    }

    pub fn recurse_on_size(&self, dir: usize, visitor: &mut impl FnMut(usize)) -> usize {
        let directory = &self.directories[dir];
        let size = directory.files.values().cloned().sum::<usize>()
            + directory
                .subdirs
                .values()
                .map(|d| self.recurse_on_size(*d, visitor))
                .sum::<usize>();
        visitor(size);
        size
    }
}

pub struct Directory {
    subdirs: HashMap<String, usize>,
    files: HashMap<String, usize>,
    parent: usize,
}

impl Directory {
    pub fn new(parent: usize) -> Self {
        Directory {
            subdirs: HashMap::new(),
            files: HashMap::new(),
            parent,
        }
    }
}

fn solve_part_one(input: &Vec<Line>) -> usize {
    let fs = FileSystem::from_lines(input.iter());
    let mut result = 0;
    fs.recurse_on_size(0, &mut |size| {
        if size <= 100000 {
            result += size
        }
    });
    result
}

fn solve_part_two(input: &Vec<Line>) -> usize {
    let fs = FileSystem::from_lines(input.iter());
    let total_size = fs.recurse_on_size(0, &mut |_| {});
    let free_space = 70000000 - total_size;
    let needed_space = 30000000 - free_space;
    let mut delete_size = None;
    fs.recurse_on_size(0, &mut |size| {
        if size >= needed_space && (delete_size.is_none() || size < delete_size.unwrap()) {
            delete_size = Some(size);
        }
    });
    delete_size.unwrap()
}

fn main() {
    solve::main(solve_part_one, solve_part_two);
}

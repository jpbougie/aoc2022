use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::complete::line_ending,
    combinator::{map_res, opt},
    multi::many0,
    AsChar, IResult,
};

enum Command {
    Ls,
    Cd(String),
}

enum Output {
    Dir(String),
    File { size: u64, name: String },
}

fn output(input: &str) -> IResult<&str, Output> {
    if let Ok((input, _)) = tag::<&str, _, nom::error::Error<&str>>("dir ")(input) {
        let (input, dir) = take_till(|c| c == '\n')(input)?;
        let (input, _) = opt(line_ending)(input)?;

        return Ok((input, Output::Dir(dir.into())));
    }

    let (input, size) = map_res(take_while(|c: char| c.is_dec_digit()), FromStr::from_str)(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, file) = take_till(|c| c == '\n')(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((
        input,
        Output::File {
            size,
            name: file.into(),
        },
    ))
}

fn command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    alt((ls, cd))(input)
}

fn cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("cd ")(input)?;
    let (input, dir) = take_till(|c| c == '\n')(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, Command::Cd(dir.into())))
}

fn ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("ls")(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, Command::Ls))
}

type Program = Vec<(Command, Vec<Output>)>;

fn program(input: &str) -> IResult<&str, Program> {
    many0(call)(input)
}

fn call(input: &str) -> IResult<&str, (Command, Vec<Output>)> {
    let (input, cmd) = command(input)?;
    let (input, outputs) = many0(output)(input)?;
    Ok((input, (cmd, outputs)))
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/07.txt");
    let (_, prog) = program(input)?;

    let mut root = Folder::default();
    let mut cur_path: Vec<String> = Vec::new();

    for (cmd, output) in prog.iter() {
        match cmd {
            Command::Ls => {
                let mut entry = &mut root;
                for path in cur_path.iter() {
                    let ent = entry.folders.entry(path.clone()).or_default();
                    entry = ent;
                }
                for out in output.iter() {
                    match out {
                        Output::Dir(d) => {
                            entry.folders.entry(d.into()).or_default();
                        }
                        Output::File { size, name } => {
                            entry.files.insert(name.into(), *size);
                        }
                    };
                }
            }
            Command::Cd(path) => {
                if path == ".." {
                    cur_path.pop();
                } else if path == "/" {
                    cur_path.clear();
                } else {
                    cur_path.push(path.into())
                }
            }
        }
    }

    let mut sizes = sizes(&root);
    let total = root.size();
    println!("{total}");

    let current_free = 70000000 - total;
    let min = 30000000;
    let to_free = min - current_free;

    sizes.sort();

    let part2 = sizes.iter().find(|sz| **sz >= to_free).unwrap();

    let part1: u64 = sizes.iter().filter(|s| **s <= 100000).sum();

    println!("Part 1: {part1} {}", root.part1());
    println!("Part 2: {part2}");

    //root.show(0);

    Ok(())
}

// Bottom-up efficient way
fn sizes(folder: &Folder) -> Vec<u64> {
    let mut self_size = folder.files.values().sum::<u64>();
    let mut out_sizes = Vec::new();
    for folder in folder.folders.values() {
        let mut szs = sizes(folder);
        self_size += szs.last().unwrap();
        out_sizes.append(&mut szs);
    }

    out_sizes.push(self_size);

    out_sizes
}

#[derive(Default, Debug)]
struct Folder {
    files: HashMap<String, u64>,
    folders: HashMap<String, Folder>,
}

impl Folder {
    fn size(&self) -> u64 {
        self.files.values().sum::<u64>() + self.folders.values().map(Folder::size).sum::<u64>()
    }

    // Lazy way
    fn part1(&self) -> u64 {
        let own_size = self.size();
        let size = self.folders.values().map(Folder::part1).sum::<u64>();

        if own_size > 100000 {
            size
        } else {
            own_size + size
        }
    }

    // fn show(&self, indent: usize) {
    //     for (name, folder) in self.folders.iter() {
    //         println!("{:indent$} {} (dir) {}", "", name, folder.size());
    //         folder.show(indent + 2);
    //     }

    //     for (name, size) in self.files.iter() {
    //         println!("{:indent$} {} ({size})", "", name);
    //     }
    // }
}

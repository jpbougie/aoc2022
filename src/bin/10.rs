use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::line_ending,
    multi::many1,
    sequence::terminated,
    IResult,
};

#[derive(Debug)]
enum Cmd {
    Noop,
    Addx(i32),
}

fn noop(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = terminated(tag("noop"), line_ending)(input)?;

    Ok((input, Cmd::Noop))
}

fn addx(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("addx ")(input)?;
    let (input, val) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, Cmd::Addx(val.parse().unwrap())))
}

fn parse(input: &str) -> IResult<&str, Vec<Cmd>> {
    many1(alt((noop, addx)))(input)
}

struct Prgm {
    x: i32,
    effects: Vec<(usize, i32)>,
    next: usize,
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/10.txt");
    let (_, cmds) = parse(input).unwrap();

    let mut prog = Prgm {
        x: 1,
        effects: Vec::new(),
        next: 0,
    };

    let mut next = 1;
    for cmd in cmds.iter() {
        match cmd {
            Cmd::Noop => next += 1,
            Cmd::Addx(val) => {
                next += 2;
                prog.effects.push((next, *val));
            }
        };
    }

    let mut part1 = 0;
    for _ in 0..4 {
        for i in 0..10 {
            print!("{i}");
        }
    }
    println!();
    for i in 1..=240usize {
        let col = (i as i32 - 1) % 40;

        if (i - 1) % 40 == 0 {
            println!();
        }

        if prog.next < prog.effects.len() && prog.effects[prog.next].0 == i {
            let val = prog.effects[prog.next].1;
            prog.next += 1;
            prog.x += val;
        }

        if prog.x == col - 1 || prog.x == col || prog.x == col + 1 {
            print!("#");
        } else {
            print!(".");
        }

        if (i as i32 - 20) % 40 == 0 {
            part1 += i as i32 * prog.x;
        }
    }

    println!();

    println!("Part 1: {part1}");

    Ok(())
}

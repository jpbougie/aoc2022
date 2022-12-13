use std::str::FromStr;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Op,
    test: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspections: usize,
}

#[derive(Debug)]
struct Op {
    operator: Operator,
    lhs: Operand,
    rhs: Operand,
}

impl Op {
    fn compute(&self, old: u64) -> u64 {
        let lhs = self.lhs.compute(old);
        let rhs = self.rhs.compute(old);

        self.operator.compute(lhs, rhs)
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn compute(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Val(u64),
}

impl Operand {
    fn compute(&self, old: u64) -> u64 {
        match self {
            Operand::Old => old,
            Operand::Val(v) => *v,
        }
    }
}

fn operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(tag("old"), |_| Operand::Old),
        map_res(digit1, |val: &str| val.parse().map(Operand::Val)),
    ))(input)
}

fn op(input: &str) -> IResult<&str, Op> {
    let (input, lhs) = terminated(operand, tag(" "))(input)?;
    let (input, operator) = terminated(
        alt((
            map(tag("+"), |_| Operator::Add),
            map(tag("*"), |_| Operator::Mul),
        )),
        tag(" "),
    )(input)?;
    let (input, rhs) = operand(input)?;

    Ok((input, Op { operator, lhs, rhs }))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, val) = delimited(tag("Monkey "), digit1, tag(":\n"))(input)?;
    let id = val.parse::<usize>().unwrap();
    let (input, items) = map_res(
        delimited(
            tag("  Starting items: "),
            separated_list1(tag(", "), digit1::<_, nom::error::Error<_>>),
            tag("\n"),
        ),
        |items| {
            items
                .into_iter()
                .map(|s: &str| s.parse::<u64>())
                .collect::<std::result::Result<Vec<u64>, _>>()
        },
    )(input)?;

    let (input, operation) = delimited(tag("  Operation: new = "), op, tag("\n"))(input)?;

    let (input, test) = delimited(
        tag("  Test: divisible by "),
        map_res(digit1, FromStr::from_str),
        tag("\n"),
    )(input)?;

    let (input, true_monkey) = delimited(
        tag("    If true: throw to monkey "),
        map_res(digit1, FromStr::from_str),
        tag("\n"),
    )(input)?;

    let (input, false_monkey) = delimited(
        tag("    If false: throw to monkey "),
        map_res(digit1, FromStr::from_str),
        tag("\n"),
    )(input)?;

    Ok((
        input,
        Monkey {
            id,
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
            inspections: 0,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n"), monkey)(input)
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/11.txt");
    let (_, mut monkeys) = parse(input)?;
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();
            for item in items {
                monkeys[i].inspections += 1;
                let mut new = monkeys[i].operation.compute(item);
                new /= 3;
                if new % monkeys[i].test == 0 {
                    let x = monkeys[i].true_monkey;
                    monkeys[x].items.push(new);
                } else {
                    let x = monkeys[i].false_monkey;
                    monkeys[x].items.push(new);
                }
            }
        }
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    println!("Part 1: {}", inspections.iter().take(2).product::<usize>());

    let input = include_str!("../../inputs/11.txt");
    let (_, mut monkeys) = parse(input)?;
    let m = monkeys.iter().map(|m| m.test).product::<u64>();
    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();
            for item in items {
                monkeys[i].inspections += 1;
                let mut new = monkeys[i].operation.compute(item);
                new %= m;
                if new % monkeys[i].test == 0 {
                    let x = monkeys[i].true_monkey;
                    monkeys[x].items.push(new);
                } else {
                    let x = monkeys[i].false_monkey;
                    monkeys[x].items.push(new);
                }
            }
        }
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    println!("Part 1: {}", inspections.iter().take(2).product::<usize>());
    Ok(())
}

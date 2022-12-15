use std::{cmp::Ordering, ops::Index};

use anyhow::Result;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum L {
    List(Vec<L>),
    Val(u32),
}

impl L {
    fn as_list(&self) -> Self {
        match self {
            L::List(_) => self.clone(),
            L::Val(_) => L::List(vec![self.clone()]),
        }
    }

    fn val(&self) -> u32 {
        match self {
            L::List(_) => todo!(),
            L::Val(v) => *v,
        }
    }

    fn list(&self) -> &[L] {
        match self {
            L::List(l) => l,
            L::Val(_) => todo!(),
        }
    }

    fn is_val(&self) -> bool {
        matches!(self, L::Val(_))
    }

    fn is_list(&self) -> bool {
        matches!(self, L::List(_))
    }
}

impl Ord for L {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is_val() && other.is_val() {
            return self.val().cmp(&other.val());
        }

        let other = if self.is_list() {
            other.as_list()
        } else {
            other.clone()
        };

        let this = if other.is_list() {
            self.as_list()
        } else {
            self.clone()
        };

        let other_list = other.list();
        let this_list = this.list();

        for (a, b) in this_list.iter().zip(other_list.iter()) {
            match a.cmp(b) {
                x @ Ordering::Less => return x,
                Ordering::Equal => {}
                x @ Ordering::Greater => return x,
            };
        }

        this_list.len().cmp(&other_list.len())
    }
}

impl PartialOrd for L {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

mod parser {

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::map_res,
        multi::separated_list0,
        sequence::{delimited, pair, terminated},
        IResult,
    };

    use super::L;

    pub fn parse(input: &str) -> IResult<&str, Vec<(L, L)>> {
        separated_list0(tag("\n"), pair(line, line))(input)
    }

    fn line(input: &str) -> IResult<&str, L> {
        terminated(list, tag("\n"))(input)
    }

    fn list(input: &str) -> IResult<&str, L> {
        let (input, vals) = delimited(
            tag("["),
            separated_list0(
                tag(","),
                alt((
                    list,
                    map_res(digit1, |digits: &str| digits.parse::<u32>().map(L::Val)),
                )),
            ),
            tag("]"),
        )(input)?;

        Ok((input, L::List(vals)))
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/13.txt");
    let (_input, lists) = parser::parse(input)?;

    let part1 = lists
        .iter()
        .enumerate()
        .filter(|(_x, (a, b))| a <= b)
        .map(|x| x.0 + 1)
        .sum::<usize>();

    println!("Part 1: {part1}");

    let mut all_lists = lists
        .into_iter()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<L>>();

    all_lists.push(L::List(vec![L::List(vec![L::Val(2)])]));
    all_lists.push(L::List(vec![L::List(vec![L::Val(6)])]));

    all_lists.sort();

    let a_packet = L::List(vec![L::List(vec![L::Val(2)])]);
    let a = all_lists.iter().position(|x| x == &a_packet).unwrap() + 1;
    let b_packet = L::List(vec![L::List(vec![L::Val(6)])]);
    let b = all_lists.iter().position(|x| x == &b_packet).unwrap() + 1;

    println!("Part 2: {}", a * b);

    Ok(())
}

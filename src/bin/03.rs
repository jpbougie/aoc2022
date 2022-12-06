use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/03.txt");
    let rucksacks = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            Rucksack {
                a: a.chars().collect(),
                b: b.chars().collect(),
            }
        })
        .collect::<Vec<Rucksack>>();

    let part1: u32 = rucksacks
        .iter()
        .map(|r| priority(r.shared().unwrap()))
        .sum();
    println!("Part 1: {}", part1);

    let part2: u32 = rucksacks.chunks(3).map(|g| priority(common(g))).sum();
    println!("Part 2: {}", part2);
    Ok(())
}

struct Rucksack {
    a: HashSet<char>,
    b: HashSet<char>,
}

impl Rucksack {
    fn shared(&self) -> Option<char> {
        self.a.intersection(&self.b).cloned().next()
    }

    fn all(&self) -> HashSet<char> {
        self.a.union(&self.b).cloned().collect()
    }
}

fn all() -> HashSet<char> {
    ('a'..='z').chain('A'..='Z').collect()
}

fn common(x: &[Rucksack]) -> char {
    *x.iter()
        .fold(all(), |h, s| {
            h.intersection(&s.all()).cloned().collect::<HashSet<char>>()
        })
        .iter()
        .next()
        .unwrap()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => 1 + (c as u32) - ('a' as u32),
        'A'..='Z' => 27 + (c as u32) - ('A' as u32),
        _ => unreachable!(),
    }
}

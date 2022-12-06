use std::str::FromStr;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("../../inputs/02.txt");
    let matches: Vec<Match> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let them = parts.next().unwrap();
            let us = parts.next().unwrap();
            Ok(Match {
                them: them.parse()?,
                our_result: us.parse()?,
                us: us.parse()?,
            })
        })
        .collect::<Result<Vec<Match>>>()?;

    let part_1 = matches.iter().map(Match::score).sum::<usize>();
    println!("Part 1: {part_1}");

    let part_2 = matches.iter().map(Match::fixed_score).sum::<usize>();
    println!("Part 2: {part_2}");

    Ok(())
}

#[derive(PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn play(&self, other: &Self) -> MatchResult {
        match self {
            RPS::Rock => match other {
                RPS::Rock => MatchResult::Draw,
                RPS::Paper => MatchResult::Loss,
                RPS::Scissors => MatchResult::Win,
            },
            RPS::Paper => match other {
                RPS::Rock => MatchResult::Win,
                RPS::Paper => MatchResult::Draw,
                RPS::Scissors => MatchResult::Loss,
            },
            RPS::Scissors => match other {
                RPS::Rock => MatchResult::Loss,
                RPS::Paper => MatchResult::Win,
                RPS::Scissors => MatchResult::Draw,
            },
        }
    }
}

impl FromStr for RPS {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('A') | Some('X') => Ok(Self::Rock),
            Some('B') | Some('Y') => Ok(Self::Paper),
            Some('C') | Some('Z') => Ok(Self::Scissors),
            _ => Err(anyhow!("No match")),
        }
    }
}

struct Match {
    them: RPS,
    our_result: MatchResult,
    us: RPS,
}

impl Match {
    fn score(&self) -> usize {
        self.us.score() + self.us.play(&self.them).score()
    }

    fn fixed_score(&self) -> usize {
        self.our_result.score() + self.our_result.required_for(&self.them).score()
    }
}

enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl MatchResult {
    fn score(&self) -> usize {
        match self {
            MatchResult::Win => 6,
            MatchResult::Loss => 0,
            MatchResult::Draw => 3,
        }
    }

    fn required_for(&self, them: &RPS) -> RPS {
        match them {
            RPS::Rock => match self {
                MatchResult::Win => RPS::Paper,
                MatchResult::Loss => RPS::Scissors,
                MatchResult::Draw => RPS::Rock,
            },
            RPS::Paper => match self {
                MatchResult::Win => RPS::Scissors,
                MatchResult::Loss => RPS::Rock,
                MatchResult::Draw => RPS::Paper,
            },
            RPS::Scissors => match self {
                MatchResult::Win => RPS::Rock,
                MatchResult::Loss => RPS::Paper,
                MatchResult::Draw => RPS::Scissors,
            },
        }
    }
}

impl FromStr for MatchResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('X') => Ok(Self::Loss),
            Some('Y') => Ok(Self::Draw),
            Some('Z') => Ok(Self::Win),
            _ => Err(anyhow!("No match")),
        }
    }
}

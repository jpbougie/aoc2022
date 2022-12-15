use std::collections::{BinaryHeap, HashSet};

use anyhow::Result;

struct Candidate {
    pos: (usize, usize),
    path: Vec<(usize, usize)>,
    dest: (usize, usize),
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.path == other.path
    }
}

impl Candidate {
    fn dist(&self) -> i32 {
        (self.pos.0 as i32 - self.dest.0 as i32).abs()
            + (self.pos.1 as i32 - self.dest.1 as i32).abs()
    }
}

impl Eq for Candidate {}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .path
            .len()
            .cmp(&self.path.len())
            .then_with(|| other.dist().cmp(&self.dist()))
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/12.txt");
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = find_char(&grid, 'S').unwrap();
    let end = find_char(&grid, 'E').unwrap();

    let part1 = solve(&grid, start, end);

    println!("Part 1: {}", part1.unwrap().path.len());

    let starts = find_mapped_chars(&grid, 'a');

    let part2 = starts
        .iter()
        .filter_map(|start| solve(&grid, *start, end).map(|c| c.path.len()))
        .min()
        .unwrap();

    println!("Part 2: {part2}");

    Ok(())
}

fn solve(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> Option<Candidate> {
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    to_visit.push(Candidate {
        pos: start,
        path: Vec::new(),
        dest: end,
    });

    while let Some(candidate) = to_visit.pop() {
        if visited.contains(&candidate.pos) {
            continue;
        }

        visited.insert(candidate.pos);
        let mut new_path = candidate.path.clone();
        new_path.push(candidate.pos);

        if candidate.pos == end {
            return Some(candidate);
        }
        let ch = mapped_char(grid[candidate.pos.0][candidate.pos.1]);

        // up
        if candidate.pos.0 > 0 {
            let newch = mapped_char(grid[candidate.pos.0 - 1][candidate.pos.1]);
            if (newch as i32 - ch as i32) <= 1 {
                to_visit.push(Candidate {
                    pos: (candidate.pos.0 - 1, candidate.pos.1),
                    path: new_path.clone(),
                    dest: end,
                });
            }
        }

        // down
        if candidate.pos.0 < grid.len() - 1 {
            let newch = mapped_char(grid[candidate.pos.0 + 1][candidate.pos.1]);
            if (newch as i32 - ch as i32) <= 1 {
                to_visit.push(Candidate {
                    pos: (candidate.pos.0 + 1, candidate.pos.1),
                    path: new_path.clone(),
                    dest: end,
                });
            }
        }

        // left
        if candidate.pos.1 > 0 {
            let newch = mapped_char(grid[candidate.pos.0][candidate.pos.1 - 1]);
            if (newch as i32 - ch as i32) <= 1 {
                to_visit.push(Candidate {
                    pos: (candidate.pos.0, candidate.pos.1 - 1),
                    path: new_path.clone(),
                    dest: end,
                });
            }
        }

        // right
        if candidate.pos.1 < grid[0].len() - 1 {
            let newch = mapped_char(grid[candidate.pos.0][candidate.pos.1 + 1]);
            if (newch as i32 - ch as i32) <= 1 {
                to_visit.push(Candidate {
                    pos: (candidate.pos.0, candidate.pos.1 + 1),
                    path: new_path.clone(),
                    dest: end,
                });
            }
        }
    }

    None
}

fn find_char(grid: &[Vec<char>], needle: char) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == needle {
                return Some((i, j));
            }
        }
    }

    None
}

fn find_mapped_chars(grid: &[Vec<char>], needle: char) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if mapped_char(*cell) == needle {
                res.push((i, j));
            }
        }
    }

    res
}

fn mapped_char(ch: char) -> char {
    match ch {
        'E' => 'z',
        'S' => 'a',
        _ => ch,
    }
}

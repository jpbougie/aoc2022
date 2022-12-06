use std::ops::RangeInclusive;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/04.txt");
    let tasks: Vec<Vec<RangeInclusive<usize>>> = input
        .lines()
        .map(|l| {
            let parts = l.split(',');
            parts
                .map(|p| {
                    let mut idxs = p.split('-');
                    let start = idxs.next().unwrap().parse::<usize>().unwrap();
                    let end = idxs.next().unwrap().parse::<usize>().unwrap();
                    RangeInclusive::new(start, end)
                })
                .collect()
        })
        .collect();

    let included = tasks
        .iter()
        .filter(|grp| {
            let x = &grp[0];
            let y = &grp[1];
            (x.contains(y.start()) && x.contains(y.end()))
                || (y.contains(x.start())) && y.contains(x.end())
        })
        .count();
    println!("Part 1: {included}");

    let overlapping = tasks
        .iter()
        .filter(|grp| {
            let x = &grp[0];
            let y = &grp[1];
            (x.contains(y.start()) || x.contains(y.end()))
                || (y.contains(x.start()))
                || y.contains(x.end())
        })
        .count();
    println!("Part 2: {overlapping}");
    Ok(())
}

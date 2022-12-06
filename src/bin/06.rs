use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/06.txt");
    let chars: Vec<char> = input.chars().collect();
    let first = 4 + chars
        .windows(4)
        .enumerate()
        .find(|(_i, sl)| sl.iter().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        .0;

    println!("Part 1: {first}");

    let second = 14
        + chars
            .windows(14)
            .enumerate()
            .find(|(_i, sl)| sl.iter().collect::<HashSet<_>>().len() == 14)
            .unwrap()
            .0;

    println!("Part 2: {second}");
    Ok(())
}

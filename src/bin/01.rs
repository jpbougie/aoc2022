use std::io::Result;
fn main() -> Result<()> {
    let input = include_str!("../../inputs/01.txt");
    let elves = input.split("\n\n");

    let mut cals = elves
        .map(|elf| elf.lines().map(|line| line.parse::<u64>().unwrap()).sum())
        .collect::<Vec<u64>>();

    cals.sort();
    cals.reverse();
    let max = cals.iter().cloned().max().unwrap_or(0);
    println!("Part 1: {max}");

    let top_3 = cals.iter().take(3).sum::<u64>();
    println!("Part 2: {top_3}");

    Ok(())
}

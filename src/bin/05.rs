use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/05.txt");
    let mut parts = input.split("\n\n");

    let state_str = parts.next().unwrap();
    let mut orig_state: Vec<Vec<char>> = Vec::new();

    for _i in 1..=9 {
        orig_state.push(Vec::new());
    }

    for line in state_str.lines() {
        if line.starts_with(" 1") {
            break;
        }

        let chars = line.chars().collect::<Vec<_>>();

        for i in 0..9 {
            let ch = chars[1 + 4 * i];
            if ch != ' ' {
                orig_state[i].insert(0, ch);
            }
        }
    }

    let commands_str = parts.next().unwrap();

    let re = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();

    let commands = re
        .captures_iter(commands_str)
        .map(|caps| {
            (
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>();

    let mut state = orig_state.to_vec();
    for (times, from, to) in commands.iter() {
        for _i in 0..*times {
            let x = state[*from - 1].pop().unwrap();
            state[*to - 1].push(x);
        }
    }

    let word = state.iter().map(|s| s.last().unwrap()).collect::<String>();
    println!("Part 1: {word}");

    let mut state = orig_state.to_vec();
    for (times, from, to) in commands.iter() {
        let mut buf = Vec::new();
        for _i in 0..*times {
            let x = state[*from - 1].pop().unwrap();
            buf.push(x);
        }
        while let Some(x) = buf.pop() {
            state[*to - 1].push(x);
        }
    }

    let word = state.iter().map(|s| s.last().unwrap()).collect::<String>();
    println!("Part 2: {word}");
    Ok(())
}

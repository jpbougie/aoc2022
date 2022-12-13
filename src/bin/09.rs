use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/09.txt");
    let commands: Vec<Command> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            Command {
                dir: parts.next().unwrap().chars().next().unwrap(),
                dist: parts.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect();
    let mut visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail);

    for command in commands.iter() {
        // println!(" == {} {} ==", command.dir, command.dist);
        for _d in 0..command.dist {
            apply_move(&mut head, command.dir);
            if !valid_tail(head, tail) {
                adjust(&mut tail, head);
                visited.insert(tail);
            }

            // println!("H {:?}", head);
            // println!("T {:?}", tail);
            // println!("");
        }
    }

    println!("Part 1: {}", visited.len());

    let mut visited = HashSet::new();
    let mut knots = vec![(0i32, 0i32); 10];
    for command in commands.iter() {
        // println!(" == {} {} ==", command.dir, command.dist);
        for _d in 0..command.dist {
            apply_move(&mut knots[0], command.dir);
            for i in 1..10 {
                let head = knots[i - 1];
                if !valid_tail(head, knots[i]) {
                    adjust(&mut knots[i], head);
                }
            }
            visited.insert(knots[9]);
        }
    }

    println!("Part 2: {}", visited.len());

    Ok(())
}

fn valid_tail(head: (i32, i32), tail: (i32, i32)) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

struct Command {
    dir: char,
    dist: i32,
}

fn apply_move(pos: &mut (i32, i32), dir: char) {
    match dir {
        'L' => pos.0 -= 1,
        'R' => pos.0 += 1,
        'U' => pos.1 -= 1,
        'D' => pos.1 += 1,
        _ => unreachable!(),
    };
}

fn adjust(tail: &mut (i32, i32), head: (i32, i32)) {
    let offset_x = {
        let diff = head.0 - tail.0;
        if diff == 0 {
            0
        } else {
            diff / diff.abs()
        }
    };

    let offset_y = {
        let diff = head.1 - tail.1;
        if diff == 0 {
            0
        } else {
            diff / diff.abs()
        }
    };

    tail.0 += offset_x;
    tail.1 += offset_y;
}

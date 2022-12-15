use std::collections::HashSet;

use anyhow::Result;

pub type Grid = HashSet<(usize, usize)>;
pub type Path = Vec<(usize, usize)>;

fn points(path: &Path) -> Vec<(usize, usize)> {
    path.windows(2)
        .flat_map(|path| {
            if path[0].0 == path[1].0 {
                let r = if path[0].1 <= path[1].1 {
                    (path[0].1)..=(path[1].1)
                } else {
                    (path[1].1)..=(path[0].1)
                };
                r.into_iter().map(|y| (path[0].0, y)).collect::<Vec<_>>()
            } else {
                let r = if path[0].0 <= path[1].0 {
                    (path[0].0)..=(path[1].0)
                } else {
                    (path[1].0)..=(path[0].0)
                };
                r.into_iter().map(|x| (x, path[0].1)).collect::<Vec<_>>()
            }
        })
        .collect()
}

mod parser {
    use std::str::FromStr;

    use nom::{
        bytes::complete::tag, character::complete::digit1, combinator::map_res,
        multi::separated_list1, sequence::separated_pair, IResult,
    };

    use crate::Path;

    pub fn parse(input: &str) -> IResult<&str, Vec<Path>> {
        separated_list1(tag("\n"), path)(input)
    }

    fn path(input: &str) -> IResult<&str, Path> {
        separated_list1(
            tag(" -> "),
            separated_pair(
                map_res(digit1, FromStr::from_str),
                tag(","),
                map_res(digit1, FromStr::from_str),
            ),
        )(input)
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/14.txt");
    let (_input, paths) = parser::parse(input)?;
    {
        let mut grid = Grid::new();
        for path in paths.iter() {
            for point in points(path) {
                grid.insert(point);
            }
        }

        let floor_level = grid.iter().map(|(_x, y)| *y).max().unwrap();
        let mut sands = 0;
        'a: loop {
            let mut pos = (500, 0);

            loop {
                if pos.1 >= floor_level {
                    break 'a;
                }

                // try to go down one
                if !grid.contains(&(pos.0, pos.1 + 1)) {
                    pos = (pos.0, pos.1 + 1);
                    continue;
                }

                // down-left
                if !grid.contains(&(pos.0 - 1, pos.1 + 1)) {
                    pos = (pos.0 - 1, pos.1 + 1);
                    continue;
                }

                // down-right
                if !grid.contains(&(pos.0 + 1, pos.1 + 1)) {
                    pos = (pos.0 + 1, pos.1 + 1);
                    continue;
                }

                break;
            }

            grid.insert(pos);

            sands += 1;
        }
        println!("Part 1: {}", sands);
    }
    {
        let mut grid = Grid::new();
        for path in paths.iter() {
            for point in points(path) {
                grid.insert(point);
            }
        }

        let floor_level = grid.iter().map(|(_x, y)| *y).max().unwrap() + 2;
        let mut sands = 0;
        loop {
            let mut pos = (500, 0);

            loop {
                if pos.1 == floor_level - 1 {
                    break;
                }

                // try to go down one
                if !grid.contains(&(pos.0, pos.1 + 1)) {
                    pos = (pos.0, pos.1 + 1);
                    continue;
                }

                // down-left
                if !grid.contains(&(pos.0 - 1, pos.1 + 1)) {
                    pos = (pos.0 - 1, pos.1 + 1);
                    continue;
                }

                // down-right
                if !grid.contains(&(pos.0 + 1, pos.1 + 1)) {
                    pos = (pos.0 + 1, pos.1 + 1);
                    continue;
                }

                break;
            }

            grid.insert(pos);
            sands += 1;

            if pos == (500, 0) {
                break;
            }
        }
        println!("Part 2: {}", sands);
    }
    Ok(())
}

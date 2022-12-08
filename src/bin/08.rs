use anyhow::Result;

type Grid = Vec<Vec<i8>>;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/08.txt");
    let grid: Grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let t = top(&grid);
    let b = bottom(&grid);
    let l = left(&grid);
    let r = right(&grid);

    let mut visible = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let h = grid[i][j];
            if h > t[i][j] || h > b[i][j] || h > l[i][j] || h > r[i][j] {
                visible += 1;
            }
        }
    }

    println!("Part 1: {visible}");

    let len = grid.len();
    let mut max = 0;
    for i in 0..len {
        for j in 0..len {
            let s = score(&grid, i, j);
            if s > max {
                max = s;
            }
        }
    }

    println!("Part 2: {max}");

    Ok(())
}

fn top(grid: &Grid) -> Grid {
    let mut new = grid.to_vec();

    for i in 0..new.len() {
        for j in 0..new[i].len() {
            if i == 0 {
                new[i][j] = -1;
            } else {
                let prev_max = new[i - 1][j];
                let prev = grid[i - 1][j];
                new[i][j] = if prev > prev_max { prev } else { prev_max };
            }
        }
    }
    new
}

fn bottom(grid: &Grid) -> Grid {
    let mut new = grid.to_vec();
    let l = new.len();

    for i in (0..l).rev() {
        for j in 0..l {
            if i == l - 1 {
                new[i][j] = -1;
            } else {
                let prev_max = new[i + 1][j];
                let prev = grid[i + 1][j];
                new[i][j] = if prev > prev_max { prev } else { prev_max };
            }
        }
    }
    new
}

fn left(grid: &Grid) -> Grid {
    let mut new = grid.to_vec();
    let l = new.len();

    for j in 0..l {
        for i in 0..l {
            if j == 0 {
                new[i][j] = -1;
            } else {
                let prev_max = new[i][j - 1];
                let prev = grid[i][j - 1];
                new[i][j] = if prev > prev_max { prev } else { prev_max };
            }
        }
    }
    new
}

fn right(grid: &Grid) -> Grid {
    let mut new = grid.to_vec();
    let l = new.len();

    for j in (0..l).rev() {
        for i in 0..l {
            if j == l - 1 {
                new[i][j] = -1;
            } else {
                let prev_max = new[i][j + 1];
                let prev = grid[i][j + 1];
                new[i][j] = if prev > prev_max { prev } else { prev_max };
            }
        }
    }
    new
}

fn score(grid: &Grid, i: usize, j: usize) -> usize {
    let len = grid.len();
    if i == 0 || j == 0 || i == len - 1 || j == len - 1 {
        return 0;
    }

    let h = grid[i][j];

    let mut t = 0;
    for x in (0..i).rev() {
        t += 1;
        if grid[x][j] >= h {
            break;
        }
    }

    let mut b = 0;
    for x in (i + 1)..len {
        b += 1;
        if grid[x][j] >= h {
            break;
        }
    }

    let mut l = 0;
    for y in (0..j).rev() {
        l += 1;
        if grid[i][y] >= h {
            break;
        }
    }

    let mut r = 0;
    for y in (j + 1)..len {
        r += 1;
        if grid[i][y] >= h {
            break;
        }
    }

    t * b * l * r
}

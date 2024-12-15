use itertools::Itertools;
use std::collections::BTreeSet;

pub fn solve(input: &str, double: bool) -> usize {
    let mut lines = input.lines();
    let mut grid = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|row| {
            if double {
                row.bytes()
                    .flat_map(|b| match b {
                        b'#' => [b'#', b'#'],
                        b'O' => [b'[', b']'],
                        b'.' => [b'.', b'.'],
                        b'@' => [b'@', b'.'],
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            } else {
                row.as_bytes().to_vec()
            }
        })
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let (y, x) =
        (0..height).cartesian_product(0..width).find(|&(y, x)| grid[y][x] == b'@').unwrap();
    grid[y][x] = b'.';
    let (mut x, mut y) = (x as isize, y as isize);

    for mov in lines.flat_map(|line| line.as_bytes()).cloned().collect::<Vec<_>>() {
        let (dx, dy) = match mov {
            b'>' => (1, 0),
            b'<' => (-1, 0),
            b'v' => (0, 1),
            b'^' => (0, -1),
            _ => unreachable!(),
        };

        let mut possible = true;
        let mut front = BTreeSet::new();
        let mut fronts = Vec::new();
        front.insert((x, y));
        while possible && !front.is_empty() {
            fronts.push(std::mem::take(&mut front));
            for &(x, y) in fronts.last().unwrap() {
                match grid[(y + dy) as usize][(x + dx) as usize] {
                    b'#' => {
                        possible = false;
                    }
                    b @ (b'O' | b'[' | b']') => {
                        front.insert((x + dx, y + dy));
                        if dy != 0 && b == b'[' {
                            front.insert((x + dx + 1, y + dy));
                        }
                        if dy != 0 && b == b']' {
                            front.insert((x + dx - 1, y + dy));
                        }
                    }
                    b'.' => {}
                    _ => unreachable!(),
                }
            }
        }

        if possible {
            // move boxes
            for (i, (x, y)) in fronts.into_iter().skip(1).rev().flatten().enumerate() {
                if double {
                    grid[(y + dy) as usize][(x + dx) as usize] =
                        if i % 2 == usize::from(dx != 1) { b']' } else { b'[' };
                } else {
                    grid[(y + dy) as usize][(x + dx) as usize] = b'O';
                }
                grid[y as usize][x as usize] = b'.';
            }
            // move robot
            (x, y) = (x + dx, y + dy);
            grid[y as usize][x as usize] = b'.';
        }
    }

    let mut result = 0;
    for (y, x) in Itertools::cartesian_product(0..height, 0..width) {
        if grid[y][x] == b'O' || grid[y][x] == b'[' {
            result += 100 * y + x;
        }
    }
    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(10092, solve(sample, false));
    assert_eq!(2028, solve(include_str!("sample2.txt"), false));
    assert_eq!(9021, solve(sample, true));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1437174, solve(input, false));
    assert_eq!(1437468, solve(input, true));
}

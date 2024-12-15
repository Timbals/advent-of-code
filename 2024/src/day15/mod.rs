use itertools::Itertools;
use std::collections::BTreeSet;

pub fn solve_first(input: &str) -> usize {
    let mut lines = input.lines();
    let mut grid = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|row| row.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let moves = lines.flat_map(|line| line.as_bytes()).cloned().collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let (mut y, mut x) =
        (0..height).cartesian_product(0..width).find(|&(y, x)| grid[y][x] == b'@').unwrap();
    grid[y][x] = b'.';

    for mov in moves {
        let (dx, dy) = match mov {
            b'>' => (1, 0),
            b'<' => (-1, 0),
            b'v' => (0, 1),
            b'^' => (0, -1),
            _ => unreachable!(),
        };

        let possible;
        let mut steps = 1;
        loop {
            match grid[y.wrapping_add_signed(steps * dy)][x.wrapping_add_signed(steps * dx)] {
                b'#' => {
                    possible = false;
                    break;
                }
                b'.' => {
                    possible = true;
                    break;
                }
                b'O' => {}
                _ => unreachable!(),
            }
            steps += 1;
        }

        if possible {
            if steps > 1 {
                grid[y.wrapping_add_signed(dy * steps)][x.wrapping_add_signed(dx * steps)] = b'O';
            }
            (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            grid[y][x] = b'.';
        }
    }

    let mut result = 0;
    #[allow(clippy::needless_range_loop)]
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == b'O' {
                result += 100 * y + x;
            }
        }
    }
    result
}

pub fn solve_second(input: &str) -> usize {
    let mut lines = input.lines();
    let mut grid = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|row| {
            row.bytes()
                .flat_map(|b| match b {
                    b'#' => [b'#', b'#'],
                    b'O' => [b'[', b']'],
                    b'.' => [b'.', b'.'],
                    b'@' => [b'@', b'.'],
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let moves = lines.flat_map(|line| line.as_bytes()).cloned().collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let (mut y, mut x) =
        (0..height).cartesian_product(0..width).find(|&(y, x)| grid[y][x] == b'@').unwrap();
    grid[y][x] = b'.';

    for mov in moves {
        let (dx, dy) = match mov {
            b'>' => (1, 0),
            b'<' => (-1, 0),
            b'v' => (0, 1),
            b'^' => (0, -1),
            _ => unreachable!(),
        };

        let mut possible = true;
        let mut end = false;
        let mut front = BTreeSet::new();
        let mut fronts = Vec::new();
        front.insert((x, y));
        while !end {
            fronts.push(std::mem::take(&mut front));
            end |= fronts
                .last()
                .unwrap()
                .iter()
                .map(|&(x, y)| match grid[y.wrapping_add_signed(dy)][x.wrapping_add_signed(dx)] {
                    b'#' => {
                        possible = false;
                        end = true;
                        false
                    }
                    b'.' => true,
                    b'[' => {
                        front.insert((x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)));
                        if dy != 0 {
                            front
                                .insert((x.wrapping_add_signed(dx + 1), y.wrapping_add_signed(dy)));
                        }
                        false
                    }
                    b']' => {
                        front.insert((x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)));
                        if dy != 0 {
                            front
                                .insert((x.wrapping_add_signed(dx - 1), y.wrapping_add_signed(dy)));
                        }
                        false
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .into_iter()
                .all(|x| x);
        }

        if possible {
            for (i, (x2, y2)) in fronts.into_iter().skip(1).rev().flatten().enumerate() {
                if dx == 1 {
                    grid[y2.wrapping_add_signed(dy)][x2.wrapping_add_signed(dx)] =
                        if i % 2 == 0 { b']' } else { b'[' };
                } else {
                    grid[y2.wrapping_add_signed(dy)][x2.wrapping_add_signed(dx)] =
                        if i % 2 == 0 { b'[' } else { b']' };
                }
                grid[y2][x2] = b'.';
            }
            (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            grid[y][x] = b'.';
        }

        // println!("{dx} {dy}");
        // for y2 in 0..height {
        //     for x2 in 0..width {
        //         if x2 == x && y2 == y {
        //             print!("@");
        //         } else {
        //             print!("{}", char::from(grid[y2][x2]));
        //         }
        //     }
        //     println!()
        // }
    }

    let mut result = 0;
    #[allow(clippy::needless_range_loop)]
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == b'[' {
                result += 100 * y + x;
            }
        }
    }
    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(10092, solve_first(sample));
    assert_eq!(2028, solve_first(include_str!("sample2.txt")));
    assert_eq!(9021, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1437174, solve_first(input));
    assert_eq!(1437468, solve_second(input));
}

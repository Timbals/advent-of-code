use itertools::Itertools;
use std::collections::{BTreeMap, HashSet, VecDeque};

pub fn solve_first(input: &str, save: usize) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let width = grid[0].len();
    let height = grid.len();

    let (sy, sx) =
        (0..height).cartesian_product(0..width).find(|&(y, x)| grid[y][x] == b'S').unwrap();

    let mut stack = VecDeque::new();
    stack.push_back((0, sx, sy, None, None));
    let mut visited = HashSet::new();
    visited.insert((sx, sy, None, None));
    let mut times = Vec::new();
    let mut normal_time = usize::MAX;
    while let Some((steps, x, y, cheat_start, cheat_end)) = stack.pop_front() {
        if grid[y][x] == b'E' {
            if cheat_start.is_none() {
                normal_time = steps;
                break;
            }

            times.push(steps);
            continue;
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            if let Some(&b) = grid.get(ny).and_then(|row| row.get(nx)) {
                if b == b'#' {
                    if (cheat_start.is_none()) && visited.insert((nx, ny, Some(steps), cheat_end)) {
                        stack.push_back((steps + 1, nx, ny, Some(steps), None));
                    }
                } else if visited.insert((nx, ny, cheat_start, cheat_end)) {
                    if cheat_start.is_some() && cheat_end.is_none() {
                        stack.push_back((steps + 1, nx, ny, cheat_start, Some((nx, ny))));
                    } else {
                        stack.push_back((steps + 1, nx, ny, cheat_start, cheat_end));
                    }
                }
            }
        }
    }

    times.into_iter().filter(|&time| time <= normal_time - save).count()
}

pub fn solve_second(input: &str, save: usize, cheat: isize) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let width = grid[0].len();
    let height = grid.len();

    let (sy, sx) =
        (0..height).cartesian_product(0..width).find(|&(y, x)| grid[y][x] == b'S').unwrap();
    let mut path = vec![(sx, sy)];
    while grid[path.last().unwrap().1][path.last().unwrap().0] != b'E' {
        let &(x, y) = path.last().unwrap();
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            if grid[ny][nx] != b'#' && (path.len() == 1 || (nx, ny) != path[path.len() - 2]) {
                path.push((nx, ny));
                break;
            }
        }
    }
    let shortcut =
        path.iter().rev().enumerate().map(|(i, &coord)| (coord, i)).collect::<BTreeMap<_, _>>();
    // println!("{path:?} {}", path.len());
    // println!("{shortcut:?} {}", path.len());

    let mut result = 0;
    for (start_steps, &(x, y)) in path.iter().enumerate() {
        let mut end = Vec::new();
        for dy in (-cheat)..=cheat {
            let remaining = cheat - dy.abs();
            for dx in -remaining..=remaining {
                let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                if let Some(&b) = grid.get(ny).and_then(|row| row.get(nx)) {
                    if b != b'#' {
                        end.push((nx, ny));
                    }
                }
            }
        }

        for (nx, ny) in end {
            let steps = start_steps + x.abs_diff(nx) + y.abs_diff(ny) + shortcut[&(nx, ny)];
            if steps + save <= path.len() - 1 {
                // println!("({x} {y}) -> ({nx} {ny}) {}", path.len() - 1 - steps);
                result += 1;
            }
        }
    }

    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(44, solve_second(sample, 2, 2));
    assert_eq!(
        32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
        solve_second(sample, 50, 20)
    );
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1375, solve_second(input, 100, 2));
    assert_eq!(983054, solve_second(input, 100, 20));
}

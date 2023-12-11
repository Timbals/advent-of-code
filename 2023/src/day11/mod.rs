use itertools::iproduct;
use std::cmp::{max, min};
use std::collections::HashSet;

pub fn solve(input: &str, multiplier: usize) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(y, _)| y)
        .collect::<HashSet<_>>();
    let empty_columns = (0..grid[0].len())
        .filter(|&i| grid.iter().all(|row| row[i] == '.'))
        .collect::<HashSet<_>>();

    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();

    iproduct!(galaxies.iter(), galaxies.iter())
        .map(|((x1, y1), (x2, y2))| {
            let empties = (min(*x1, *x2)..max(*x1, *x2))
                .filter(|x| empty_columns.contains(x))
                .count()
                + (min(*y1, *y2)..max(*y1, *y2))
                    .filter(|y| empty_rows.contains(y))
                    .count();

            ((x1, y1), (x2, y2), empties)
        })
        .map(|((x1, y1), (x2, y2), empties)| {
            x1.abs_diff(*x2) + y1.abs_diff(*y2) + empties * (multiplier - 1)
        })
        .sum::<usize>()
        / 2
}

pub fn solve_first(input: &str) -> usize {
    solve(input, 2)
}

pub fn solve_second(input: &str) -> usize {
    solve(input, 1000000)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(374, solve_first(sample));
    assert_eq!(8410, solve(sample, 100));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(9418609, solve_first(input));
    assert_eq!(593821230983, solve_second(input));
}

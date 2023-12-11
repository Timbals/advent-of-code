use itertools::Itertools;
use std::cmp::{max, min};

pub fn solve(input: &str, multiplier: usize) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cum_sum = |acc: &mut usize, x| {
        *acc += x as usize;
        Some(*acc)
    };
    let empty_rows = grid
        .iter()
        .map(|row| row.iter().all(|&c| c == '.'))
        .scan(0, cum_sum)
        .collect::<Vec<_>>();
    let empty_columns = (0..grid[0].len())
        .map(|i| grid.iter().all(|row| row[i] == '.'))
        .scan(0, cum_sum)
        .collect::<Vec<_>>();

    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| (x, y))
        })
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            let empty_columns = empty_columns[max(x1, x2)] - empty_columns[min(x1, x2)];
            let empty_rows = empty_rows[max(y1, y2)] - empty_rows[min(y1, y2)];
            x1.abs_diff(x2) + y1.abs_diff(y2) + (empty_columns + empty_rows) * (multiplier - 1)
        })
        .sum()
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
    assert_eq!(1030, solve(sample, 10));
    assert_eq!(8410, solve(sample, 100));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(9418609, solve_first(input));
    assert_eq!(593821230983, solve_second(input));
}

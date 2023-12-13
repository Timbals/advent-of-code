use std::cmp::min;
use std::iter::zip;

pub fn solve(input: &str, defects: usize) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let rows = pattern
                .lines()
                .map(|line| line.chars().map(|x| x == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let find_mirror = |data: &Vec<Vec<bool>>| -> Option<usize> {
                for index in 1..data.len() {
                    let (left, right) = data.split_at(index);
                    let len = min(left.len(), right.len());

                    if zip(
                        left.iter().rev().take(len).flatten(),
                        right.iter().take(len).flatten(),
                    )
                    .filter(|(a, b)| a != b)
                    .count()
                        == defects
                    {
                        return Some(index);
                    }
                }
                None
            };

            if let Some(row) = find_mirror(&rows) {
                row * 100
            } else {
                let columns = (0..rows[0].len())
                    .map(|column| rows.iter().map(|row| row[column]).collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                find_mirror(&columns).unwrap()
            }
        })
        .sum()
}

pub fn solve_first(input: &str) -> usize {
    solve(input, 0)
}

pub fn solve_second(input: &str) -> usize {
    solve(input, 1)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(405, solve_first(sample));
    assert_eq!(400, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(34918, solve_first(input));
    assert_eq!(33054, solve_second(input));
}

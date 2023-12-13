use itertools::equal;
use std::cmp::min;

pub fn solve_first(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let rows = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            for row in 1..rows.len() {
                let (left, right) = rows.split_at(row);
                let len = min(left.len(), right.len());

                if equal(left.iter().rev().take(len), right.iter().take(len)) {
                    return 100 * row;
                }
            }

            let columns = (0..rows[0].len())
                .map(|column| rows.iter().map(|row| row[column]).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            for column in 1..columns.len() {
                let (left, right) = columns.split_at(column);
                let len = min(left.len(), right.len());

                if equal(left.iter().rev().take(len), right.iter().take(len)) {
                    return column;
                }
            }

            unreachable!()
        })
        .sum()
}

pub fn solve_second(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let mut rows = pattern
                .lines()
                .map(|line| line.chars().map(|x| x == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let mut original = 0;

            for row in 1..rows.len() {
                let (left, right) = rows.split_at(row);
                let len = min(left.len(), right.len());

                if equal(left.iter().rev().take(len), right.iter().take(len)) {
                    original = 100 * row;
                }
            }

            let columns = (0..rows[0].len())
                .map(|column| rows.iter().map(|row| row[column]).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            for column in 1..columns.len() {
                let (left, right) = columns.split_at(column);
                let len = min(left.len(), right.len());

                if equal(left.iter().rev().take(len), right.iter().take(len)) {
                    original = column;
                }
            }

            for row in 0..rows.len() {
                for column in 0..rows[0].len() {
                    rows[row][column] = !rows[row][column];

                    for row in 1..rows.len() {
                        let (left, right) = rows.split_at(row);
                        let len = min(left.len(), right.len());

                        if equal(left.iter().rev().take(len), right.iter().take(len)) {
                            if 100 * row != original {
                                return 100 * row;
                            }
                        }
                    }

                    let columns = (0..rows[0].len())
                        .map(|column| rows.iter().map(|row| row[column]).collect::<Vec<_>>())
                        .collect::<Vec<_>>();

                    for column in 1..columns.len() {
                        let (left, right) = columns.split_at(column);
                        let len = min(left.len(), right.len());

                        if equal(left.iter().rev().take(len), right.iter().take(len)) {
                            if column != original {
                                return column;
                            }
                        }
                    }

                    rows[row][column] = !rows[row][column];
                }
            }

            unreachable!()
        })
        .sum()
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

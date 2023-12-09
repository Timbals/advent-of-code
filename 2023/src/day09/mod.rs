use itertools::Itertools;
use std::ops::Add;

pub fn solve(input: &str, reverse: bool) -> isize {
    input
        .lines()
        .map(|line| {
            let sequence = line.split_whitespace().map(|x| x.parse::<isize>().unwrap());

            let sequence: Vec<_> = if reverse {
                sequence.rev().collect()
            } else {
                sequence.collect()
            };

            let mut sequences = vec![sequence];
            while !sequences.last().unwrap().iter().all(|&x| x == 0) {
                sequences.push(
                    sequences
                        .last()
                        .unwrap()
                        .iter()
                        .tuple_windows()
                        .map(|(x, y)| y - x)
                        .collect(),
                );
            }

            sequences
                .into_iter()
                .map(|x| *x.last().unwrap())
                .reduce(isize::add)
                .unwrap()
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(114, solve(sample, false));
    assert_eq!(2, solve(sample, true));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1762065988, solve(input, false));
    assert_eq!(1066, solve(input, true));
}

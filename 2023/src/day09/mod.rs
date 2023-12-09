use itertools::Itertools;
use std::collections::VecDeque;
use std::ops::{Add, Sub};

fn solve(
    input: &str,
    push: fn(&mut VecDeque<isize>, isize),
    access: fn(&VecDeque<isize>) -> Option<&isize>,
    combine: fn(isize, isize) -> isize,
) -> isize {
    input
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<VecDeque<_>>();

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

            push(sequences.last_mut().unwrap(), 0);
            for i in (1..sequences.len()).rev() {
                let next_value = combine(
                    *access(&sequences[i - 1]).unwrap(),
                    *access(&sequences[i]).unwrap(),
                );
                push(&mut sequences[i - 1], next_value);
            }

            *access(sequences.first().unwrap()).unwrap()
        })
        .sum()
}

pub fn solve_first(input: &str) -> isize {
    solve(input, VecDeque::push_back, VecDeque::back, isize::add)
}

pub fn solve_second(input: &str) -> isize {
    solve(input, VecDeque::push_front, VecDeque::front, isize::sub)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(114, solve_first(sample));
    assert_eq!(2, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1762065988, solve_first(input));
    assert_eq!(1066, solve_second(input));
}

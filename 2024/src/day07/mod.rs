use itertools::Itertools;
use std::iter::{repeat_n, zip};
use std::ops::{Add, Mul};

pub fn solve(input: &str, concatenation: bool) -> u64 {
    let operators = if concatenation {
        vec![u64::add, u64::mul, |a: u64, b: u64| a * 10_u64.pow(b.ilog10() + 1) + b]
    } else {
        vec![u64::add, u64::mul]
    };

    input
        .lines()
        .filter_map(|line| {
            let (value, equation) = line.split_once(": ").unwrap();
            let value = value.parse::<u64>().unwrap();
            let equation =
                equation.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
            repeat_n(&operators, equation.len() - 1)
                .multi_cartesian_product()
                .any(|operators| {
                    let mut equation = equation.iter();
                    let first = *equation.next().unwrap();
                    zip(equation, operators).fold(first, |a, (b, operator)| operator(a, *b))
                        == value
                })
                .then_some(value)
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(3749, solve(sample, false));
    assert_eq!(11387, solve(sample, true));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2314935962622, solve(input, false));
    assert_eq!(401477450831495, solve(input, true));
}

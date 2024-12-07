use itertools::Itertools;
use std::iter::{repeat_n, zip};
use std::ops::{Add, Mul};

pub fn solve_first(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (value, equation) = line.split_once(": ").unwrap();
            let value = value.parse::<usize>().unwrap();
            let equation = equation
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            repeat_n([usize::add, usize::mul], equation.len() - 1)
                .multi_cartesian_product()
                .any(|operators| {
                    let mut equation = equation.iter();
                    let mut result = *equation.next().unwrap();
                    for (operand, operator) in zip(equation, operators) {
                        result = operator(result, *operand);
                    }
                    result == value
                })
                .then_some(value)
        })
        .sum()
}

pub fn solve_second(input: &str) -> u128 {
    input
        .lines()
        .filter_map(|line| {
            let (value, equation) = line.split_once(": ").unwrap();
            let value = value.parse::<u128>().unwrap();
            let equation =
                equation.split_whitespace().map(|x| x.parse::<u128>().unwrap()).collect::<Vec<_>>();
            repeat_n(
                [u128::add, u128::mul, |a: u128, b: u128| a * 10_u128.pow(b.ilog10() + 1) + b],
                equation.len() - 1,
            )
            .multi_cartesian_product()
            .any(|operators| {
                let mut equation = equation.iter();
                let mut result = *equation.next().unwrap();
                for (operand, operator) in zip(equation, operators) {
                    result = operator(result, *operand);
                }
                result == value
            })
            .then_some(value)
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(3749, solve_first(sample));
    assert_eq!(11387, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2314935962622, solve_first(input));
    assert_eq!(401477450831495, solve_second(input));
}

use std::collections::HashMap;
use std::iter::zip;

pub fn solve_first(input: &str) -> usize {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once("   ").unwrap();
        left.push(x.parse::<usize>().unwrap());
        right.push(y.parse::<usize>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).map(|(x, y)| usize::abs_diff(x, y)).sum()
}

pub fn solve_second(input: &str) -> usize {
    let mut left = Vec::new();
    let mut right = HashMap::new();
    for line in input.lines() {
        let (x, y) = line.split_once("   ").unwrap();
        left.push(x.parse::<usize>().unwrap());
        *right.entry(y.parse::<usize>().unwrap()).or_default() += 1;
    }

    left.into_iter().map(|x| x * *right.get(&x).unwrap_or(&0)).sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(11, solve_first(sample));
    assert_eq!(31, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2285373, solve_first(input));
    assert_eq!(21142653, solve_second(input));
}

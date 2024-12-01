use std::collections::HashMap;
use std::iter::zip;

pub fn solve_first(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once("   ").unwrap();
        left.push(x.parse::<u32>().unwrap());
        right.push(y.parse::<u32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).map(|(x, y)| u32::abs_diff(x, y)).sum()
}

pub fn solve_second(input: &str) -> u32 {
    let mut left = Vec::new();
    let mut right = HashMap::new();
    for line in input.lines() {
        let (x, y) = line.split_once("   ").unwrap();
        left.push(x.parse::<u32>().unwrap());
        *right.entry(y.parse::<u32>().unwrap()).or_default() += 1;
    }

    left.into_iter().map(|x| x * *right.get(&x).unwrap_or(&0)).sum()
}

#[test]
pub fn sample() {
    assert_eq!(11, solve_first(include_str!("sample.txt")));
    assert_eq!(31, solve_second(include_str!("sample.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2285373, solve_first(input));
    assert_eq!(21142653, solve_second(input));
}

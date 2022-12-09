use itertools::Itertools;
use std::ops::{BitAnd, BitOr};

fn bitset(slice: &[u8]) -> u64 {
    slice
        .iter()
        .map(|c| match c {
            b'a'..=b'z' => c - b'a' + 1,
            _ => c - b'A' + 27,
        })
        .map(|c| 1_u64 << c)
        .reduce(u64::bitor)
        .unwrap_or_default()
}

fn solve_first(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            let (left, right) = rucksack.as_bytes().split_at(rucksack.len() / 2);
            (bitset(left) & bitset(right)).trailing_zeros()
        })
        .sum()
}

fn solve_second(input: &str) -> u32 {
    input
        .lines()
        .map(str::as_bytes)
        .map(bitset)
        .chunks(3)
        .into_iter()
        .map(|bitsets| {
            bitsets
                .reduce(u64::bitand)
                .unwrap_or_default()
                .trailing_zeros()
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(157, solve_first(sample));
    assert_eq!(70, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(7701, solve_first(input));
    assert_eq!(2644, solve_second(input));
}

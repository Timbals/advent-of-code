use itertools::Itertools;
use std::array::from_fn;
use std::collections::HashSet;

fn solve_first(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            let n = rucksack.len();
            rucksack
                .chars()
                .map(|c| {
                    if c.is_ascii_lowercase() {
                        c as u32 - 96
                    } else {
                        c as u32 - 64 + 26
                    }
                })
                .chunks(n / 2)
        })
        .map(|contents| {
            let mut contents = contents.into_iter();
            let [left, right] = from_fn(|_| HashSet::<u32>::from_iter(contents.next().unwrap()));
            *HashSet::intersection(&left, &right).next().unwrap()
        })
        .sum()
}

fn solve_second(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            rucksack.chars().map(|c| {
                if c.is_ascii_lowercase() {
                    c as u32 - 96
                } else {
                    c as u32 - 64 + 26
                }
            })
        })
        .chunks(3)
        .into_iter()
        .map(|contents| {
            *contents
                .flat_map(|x| HashSet::<u32>::from_iter(x).into_iter())
                .counts()
                .iter()
                .find(|(_, count)| **count == 3)
                .unwrap()
                .0
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

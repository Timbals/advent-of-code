use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn rand(mut seed: usize) -> usize {
    seed ^= (seed << 6) & 0xFFFFFF;
    seed ^= (seed >> 5) & 0xFFFFFF;
    seed ^ (seed << 11) & 0xFFFFFF
}

pub fn solve_first(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut seed = line.parse::<usize>().unwrap();
            for _ in 0..2000 {
                seed = rand(seed);
            }
            seed
        })
        .sum()
}

pub fn solve_second(input: &str) -> usize {
    let mut scores = HashMap::new();

    for line in input.lines() {
        let mut seed = line.parse::<usize>().unwrap();
        let data = (0..2000).map(|_| {
            let initial = seed;
            seed = rand(seed);
            (seed % 10, (seed % 10) as isize - (initial % 10) as isize)
        });

        let mut seen = HashSet::new();
        for (a, b, c, d) in data.tuple_windows() {
            let sequence = (a.1, b.1, c.1, d.1);
            if seen.insert(sequence) {
                *scores.entry(sequence).or_default() += d.0;
            }
        }
    }

    scores.into_values().max().unwrap_or_default()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(37327623, solve_first(sample));
    assert_eq!(23, solve_second(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(20215960478, solve_first(input));
    assert_eq!(2221, solve_second(input));
}

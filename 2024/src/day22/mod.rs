use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve_first(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut seed = line.parse::<u64>().unwrap();
            for _ in 0..2000 {
                // shift left 6
                seed ^= seed * 64;
                // mask out last 24 bits
                seed %= 16777216;
                // shift right 5
                seed ^= seed / 32;
                // mask out last 24 bits
                seed %= 16777216;
                // shift left 11
                seed ^= seed * 2048;
                // mask out last 24 bits
                seed %= 16777216;
            }
            seed
        })
        .sum()
}

pub fn solve_second(input: &str) -> u64 {
    let mut scores = HashMap::new();

    for line in input.lines() {
        let mut seed = line.parse::<u64>().unwrap();
        let mut changes = VecDeque::with_capacity(5);
        let mut seen = HashSet::new();
        for _ in 0..2000 {
            let initial = seed;

            // shift left 6
            seed ^= seed * 64;
            // mask out last 24 bits
            seed %= 16777216;
            // shift right 5
            seed ^= seed / 32;
            // mask out last 24 bits
            seed %= 16777216;
            // shift left 11
            seed ^= seed * 2048;
            // mask out last 24 bits
            seed %= 16777216;

            let change = (seed % 10) as isize - (initial % 10) as isize;
            changes.push_back(change);
            if changes.len() == 5 {
                changes.pop_front();
            }
            if changes.len() == 4 {
                let sequence: [isize; 4] =
                    changes.iter().copied().collect::<Vec<_>>().try_into().unwrap();
                if seen.insert(sequence) {
                    *scores.entry(sequence).or_default() += seed % 10;
                }
            }
        }
    }

    scores.into_values().max().unwrap()
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

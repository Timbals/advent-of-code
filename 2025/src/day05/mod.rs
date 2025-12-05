use std::cmp::{max, min};
use std::ops::RangeInclusive;

pub fn solve_first(input: &str) -> usize {
    let mut lines = input.lines();

    let mut ranges = Vec::new();
    for line in lines.by_ref().take_while(|x| !x.is_empty()) {
        let (start, end) = line.split_once('-').unwrap();
        let (start, end) = (start.parse::<usize>().unwrap(), end.parse().unwrap());
        ranges.push(start..=end);
    }

    let mut fresh = 0;
    for line in lines {
        let ingredient = line.parse().unwrap();
        for range in &ranges {
            if range.contains(&ingredient) {
                fresh += 1;
                break;
            }
        }
    }
    fresh
}

pub fn solve_second(input: &str) -> usize {
    let mut lines = input.lines();

    let mut ranges = Vec::<RangeInclusive<usize>>::new();
    for line in lines.by_ref().take_while(|x| !x.is_empty()) {
        let (start, end) = line.split_once('-').unwrap();
        let (start, end) = (start.parse::<usize>().unwrap(), end.parse().unwrap());
        let mut range = start..=end;
        ranges.retain(|other| {
            if *range.start() <= *other.end() && *range.end() >= *other.start() {
                range = min(*range.start(), *other.start())..=max(*range.end(), *other.end());
                false
            } else {
                true
            }
        });
        ranges.push(range);
    }

    ranges.into_iter().map(RangeInclusive::count).sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(3, solve_first(sample));
    assert_eq!(14, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(726, solve_first(input));
    assert_eq!(354226555270043, solve_second(input));
}

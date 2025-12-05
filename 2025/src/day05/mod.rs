use itertools::Itertools;
use std::cmp::max;

pub fn solve_first(input: &str) -> usize {
    let mut lines = input.lines();

    let mut ranges = Vec::new();
    for line in lines.by_ref().take_while(|x| !x.is_empty()) {
        let (start, end) = line.split_once('-').unwrap();
        let (start, end) = (start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap() + 1);
        ranges.push(start..end);
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
    let ranges = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let (start, end) = (start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap() + 1);
            (start, end)
        })
        .sorted_unstable();

    let mut fresh = 0;
    let mut furthest_end = 0;
    for (start, end) in ranges {
        let (start, end) = (max(start, furthest_end), max(end, furthest_end));
        furthest_end = end;
        fresh += end - start;
    }
    fresh
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

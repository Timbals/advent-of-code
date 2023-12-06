use std::iter::zip;

pub fn solve_first(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    zip(times, distances)
        .map(|(time, distance)| {
            (0..time)
                .filter(|held| (time - held) * held > distance)
                .count()
        })
        .product()
}

pub fn solve_second(input: &str) -> usize {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    (0..time)
        .filter(|held| (time - held) * held > distance)
        .count()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(288, solve_first(sample));
    assert_eq!(71503, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(32076, solve_first(input));
    assert_eq!(34278221, solve_second(input));
}

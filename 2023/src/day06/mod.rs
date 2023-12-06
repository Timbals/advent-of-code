use std::iter::zip;

fn solve(time: u64, distance: u64) -> u64 {
    let left = (time as f64 / 2.0) - (time.pow(2) as f64 / 4.0 - distance as f64).sqrt();
    let right = (time as f64 / 2.0) + (time.pow(2) as f64 / 4.0 - distance as f64).sqrt();

    // Need to subtract/add some epsilon value,
    // because the solution needs to *beat* and not not just match the distance.
    // Should really use `next_down`/`next_up` (which also handle edge cases) but they are unstable.
    (f64::from_bits(right.to_bits() - 1).floor() - f64::from_bits(left.to_bits() + 1).ceil()) as u64
        + 1
}

pub fn solve_first(input: &str) -> u64 {
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
        .map(|(time, distance)| solve(time, distance))
        .product()
}

pub fn solve_second(input: &str) -> u64 {
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

    solve(time, distance)
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

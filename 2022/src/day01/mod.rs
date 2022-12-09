use itertools::Itertools;

pub fn solve_first(input: &str) -> u32 {
    input
        .split_terminator("\n\n")
        .map(|load| load.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn solve_second(input: &str) -> u32 {
    input
        .split_terminator("\n\n")
        .map(|load| load.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .sorted() // could be improved by using a min-heap with k=3
        .rev()
        .take(3)
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(24_000, solve_first(sample));
    assert_eq!(45_000, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(70_613, solve_first(input));
    assert_eq!(205_805, solve_second(input));
}

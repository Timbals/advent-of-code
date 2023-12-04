use std::collections::HashSet;

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| {
        let (left, right) = line.split_once(" | ").unwrap();
        let (_, left) = left.split_once(": ").unwrap();
        let winning = left
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        right
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .filter(|x| winning.contains(x))
            .count()
    })
}

pub fn solve_first(input: &str) -> usize {
    parse(input)
        .map(|x| if x == 0 { 0 } else { 2_usize.pow(x as u32 - 1) })
        .sum()
}

pub fn solve_second(input: &str) -> usize {
    let matches = parse(input).collect::<Vec<_>>();
    let mut cards = vec![1; matches.len()];

    for card in 0..cards.len() {
        for copy_card in (card + 1)..=(card + matches[card]) {
            cards[copy_card] += cards[card];
        }
    }

    cards.into_iter().sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(13, solve_first(sample));
    assert_eq!(30, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(24160, solve_first(input));
    assert_eq!(5659035, solve_second(input));
}

use std::collections::HashSet;

pub fn solve_first(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" | ").unwrap();
            let (_, left) = left.split_once(": ").unwrap();
            let winning = left
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let count = right
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|number| number.parse::<u32>().unwrap())
                .filter(|x| winning.contains(x))
                .count() as u32;
            if count == 0 {
                0
            } else {
                2_u32.pow(count.saturating_add_signed(-1))
            }
        })
        .sum()
}

pub fn solve_second(input: &str) -> u32 {
    let matches = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" | ").unwrap();
            let (_, left) = left.split_once(": ").unwrap();
            let winning = left
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let count = right
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|number| number.parse::<u32>().unwrap())
                .filter(|x| winning.contains(x))
                .count();
            count
        })
        .collect::<Vec<_>>();
    let mut cards = vec![1; matches.len()];

    for i in 0..matches.len() {
        for j in (i + 1)..=(i + matches[i]) {
            cards[j] += cards[i];
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

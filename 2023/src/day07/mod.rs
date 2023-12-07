use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

fn hand_type(hand: &str) -> usize {
    let mut counts = HashMap::<char, usize>::new();
    for c in hand.chars() {
        *counts.entry(c).or_default() += 1;
    }
    let counts = counts.into_values().sorted().collect::<Vec<_>>();

    match &counts.as_slice() {
        [.., 5] => 6,
        [.., 4] => 5,
        [.., 2, 3] => 4,
        [.., 3] => 3,
        [.., 2, 2] => 2,
        [.., 2] => 1,
        _ => 0,
    }
}

static CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

pub fn solve_first(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();

            (hand, bid)
        })
        .sorted_by(|(a, _), (b, _)| match hand_type(a).cmp(&hand_type(b)) {
            Ordering::Equal => {
                for (a, b) in zip(a.chars(), b.chars()) {
                    let cmp = CARDS
                        .iter()
                        .position(|&x| x == a)
                        .unwrap()
                        .cmp(&CARDS.iter().position(|&x| x == b).unwrap());
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                Ordering::Equal
            }
            ord => ord,
        })
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

fn hand_type_2(hand: &str) -> usize {
    let mut counts = HashMap::<char, usize>::new();
    for c in hand.chars() {
        *counts.entry(c).or_default() += 1;
    }
    let joker_count = counts.remove(&'J').unwrap_or(0);

    let counts = counts.into_values().sorted().collect::<Vec<_>>();

    match &counts.as_slice() {
        [.., x] if x + joker_count >= 5 => 6,
        [..] if joker_count >= 5 => 6,

        [.., x] if x + joker_count >= 4 => 5,

        [.., 2, 3] => 4,
        [.., 1, 3] | [.., 2, 2] if joker_count >= 1 => 4,
        [.., 1, 2] | [.., 3] if joker_count >= 2 => 4,
        [.., 1, 1] | [.., 2] if joker_count >= 3 => 4,

        [.., x] if x + joker_count >= 3 => 3,

        [.., 2, 2] => 2,
        [.., 1, 2] if joker_count >= 1 => 2,
        [.., 1, 1] | [.., 2] if joker_count >= 2 => 2,

        [.., x] if x + joker_count >= 2 => 1,

        _ => 0,
    }
}

static CARDS_2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

pub fn solve_second(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();

            (hand, bid)
        })
        .sorted_by(|(a, _), (b, _)| match hand_type_2(a).cmp(&hand_type_2(b)) {
            Ordering::Equal => {
                for (a, b) in zip(a.chars(), b.chars()) {
                    let cmp = CARDS_2
                        .iter()
                        .position(|&x| x == a)
                        .unwrap()
                        .cmp(&CARDS_2.iter().position(|&x| x == b).unwrap());
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                Ordering::Equal
            }
            ord => ord,
        })
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(6440, solve_first(sample));
    assert_eq!(5905, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(251058093, solve_first(input));
    assert_eq!(249781879, solve_second(input));
}

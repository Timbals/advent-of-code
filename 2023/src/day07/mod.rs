use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

fn card_value(card: char, joker: bool) -> usize {
    match card {
        'J' if joker => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' if !joker => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn hand_type(hand: &str, joker: bool) -> usize {
    let mut counts = HashMap::<char, usize>::new();
    for card in hand.chars() {
        *counts.entry(card).or_default() += 1;
    }

    let joker_count = if joker {
        counts.remove(&'J').unwrap_or(0)
    } else {
        0
    };

    match &counts.into_values().sorted().collect::<Vec<_>>().as_slice() {
        [.., x] if x + joker_count >= 5 => 6,        // five of a kind
        [..] if joker_count >= 5 => 6,               // five of a kind
        [.., x] if x + joker_count >= 4 => 5,        // four of a kind
        [.., x, y] if x + y + joker_count >= 5 => 4, // full house
        [.., x] if x + joker_count >= 3 => 3,        // three of a kind
        [.., x, y] if x + y + joker_count >= 4 => 2, // two pair
        [.., x] if x + joker_count >= 2 => 1,        // one pair
        _ => 0,                                      // high card
    }
}

pub fn solve(input: &str, joker: bool) -> usize {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();

            (hand, bid)
        })
        .sorted_unstable_by(
            |(a, _), (b, _)| match hand_type(a, joker).cmp(&hand_type(b, joker)) {
                Ordering::Equal => zip(a.chars(), b.chars())
                    .find(|(a, b)| a != b)
                    .map(|(a, b)| card_value(a, joker).cmp(&card_value(b, joker)))
                    .unwrap_or(Ordering::Equal),
                ord => ord,
            },
        )
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(6440, solve(sample, false));
    assert_eq!(5905, solve(sample, true));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(251058093, solve(input, false));
    assert_eq!(249781879, solve(input, true));
}

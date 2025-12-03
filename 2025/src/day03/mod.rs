use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve_first(input: &str) -> usize {
    let mut joltage = 0;

    for bank in input.lines().map(str::as_bytes) {
        let first = bank[..bank.len() - 1].iter().position_min_by_key(|x| b'9' - **x).unwrap();
        let second =
            bank[first + 1..].iter().position_min_by_key(|x| b'9' - **x).unwrap() + first + 1;
        let (first, second) = (bank[first] - b'0', bank[second] - b'0');
        joltage += first as usize * 10 + second as usize;
    }

    joltage
}

pub fn solve_second(input: &str) -> usize {
    let mut total = 0;

    for bank in input.lines().map(str::as_bytes) {
        let mut joltage = 0;
        let mut remaining = bank.iter().enumerate().map(|(i, x)| (*x, Reverse(i)));
        let mut heap = BinaryHeap::from_iter(remaining.by_ref().take(bank.len() - 12));
        for _ in 0..12 {
            heap.push(remaining.next().unwrap());
            let (x, Reverse(i)) = heap.pop().unwrap();
            heap.retain(|(_, Reverse(pos))| *pos > i);
            joltage = joltage * 10 + (x - b'0') as usize;
        }
        total += joltage;
    }

    total
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(357, solve_first(sample));
    assert_eq!(3121910778619, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(17034, solve_first(input));
    assert_eq!(168798209663590, solve_second(input));
}

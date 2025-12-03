use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve(input: &str, k: usize) -> usize {
    let mut total = 0;

    for bank in input.lines().map(str::as_bytes) {
        let mut joltage = 0;
        let mut remaining = bank.iter().enumerate().map(|(i, x)| (*x, Reverse(i)));
        let mut heap = BinaryHeap::from_iter(remaining.by_ref().take(bank.len() - k));
        for _ in 0..k {
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
    assert_eq!(357, solve(sample, 2));
    assert_eq!(3121910778619, solve(sample, 12));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(17034, solve(input, 2));
    assert_eq!(168798209663590, solve(input, 12));
}

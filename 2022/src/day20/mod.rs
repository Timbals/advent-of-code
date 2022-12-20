use std::collections::VecDeque;
use std::iter::repeat;

pub fn solve(input: &str, decryption_key: isize, times: usize) -> isize {
    let mut numbers = input
        .lines()
        .map(|x| x.parse::<isize>().unwrap() * decryption_key)
        .enumerate()
        .collect::<VecDeque<_>>();

    for i in repeat(0..numbers.len()).take(times).flatten() {
        let pos = numbers.iter().position(|(index, _)| *index == i).unwrap();
        let element = numbers.remove(pos).unwrap();
        let new_index = (pos as isize + element.1).rem_euclid(numbers.len() as isize) as usize;
        numbers.insert(new_index, element);
    }

    let zero_index = numbers.iter().position(|(_, value)| *value == 0).unwrap();

    [1000, 2000, 3000]
        .map(|i| (zero_index + i).rem_euclid(numbers.len()))
        .map(|i| numbers[i].1)
        .into_iter()
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(3, solve(sample, 1, 1));
    assert_eq!(1623178306, solve(sample, 811589153, 10));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2203, solve(input, 1, 1));
    assert_eq!(6641234038999, solve(input, 811589153, 10));
}

use itertools::Itertools;
use std::iter::zip;

pub fn solve(input: &str) -> usize {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for key_or_lock in input.lines().chunks(8).into_iter() {
        let mut key_or_lock = key_or_lock.take(7);
        let key_or_lock =
            std::array::from_fn::<_, 7, _>(|_| key_or_lock.next().unwrap().as_bytes());
        let heights = std::array::from_fn::<_, 5, _>(|column| {
            (0..7).map(|row| key_or_lock[row][column]).filter(|&x| x == b'#').count()
        });
        if key_or_lock[0] == [b'#', b'#', b'#', b'#', b'#'] {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    Itertools::cartesian_product(keys.into_iter(), locks)
        .filter(|(key, lock)| zip(key, lock).all(|(&key, &lock)| key + lock <= 7))
        .count()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(3, solve(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2824, solve(input));
}

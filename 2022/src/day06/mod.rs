use std::collections::HashSet;

fn solve<const K: usize>(input: &str) -> usize {
    input
        .as_bytes()
        .windows(K)
        .enumerate()
        .find(|(_, x)| HashSet::<u8>::from_iter(x.iter().copied()).len() == K)
        .map(|(i, _)| i + K)
        .unwrap()
}

#[test]
pub fn sample() {
    assert_eq!(7, solve::<4>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(5, solve::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, solve::<4>("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(10, solve::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(11, solve::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));

    assert_eq!(19, solve::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(23, solve::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, solve::<14>("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(29, solve::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(26, solve::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1109, solve::<4>(input));
    assert_eq!(3965, solve::<14>(input));
}

use itertools::Itertools;

pub fn solve_first(input: &str) -> usize {
    let mut invalid = 0;

    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());
        for id in start..=end {
            let bytes = id.to_string().into_bytes();
            let len = bytes.len();
            if len % 2 == 0 && bytes[..len / 2] == bytes[len / 2..] {
                invalid += id;
            }
        }
    }

    invalid
}

pub fn solve_second(input: &str) -> usize {
    let mut invalid = 0;

    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());
        for id in start..=end {
            let bytes = id.to_string().into_bytes();
            let len = bytes.len();
            for chunk_size in 1..=len / 2 {
                if len % chunk_size == 0 && bytes.chunks_exact(chunk_size).all_equal() {
                    invalid += id;
                    break;
                }
            }
        }
    }

    invalid
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(1227775554, solve_first(sample));
    assert_eq!(4174379265, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(40398804950, solve_first(input));
    assert_eq!(65794984339, solve_second(input));
}

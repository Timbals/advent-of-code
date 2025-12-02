pub fn solve_first(input: &str) -> usize {
    let mut invalid = 0;

    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());
        for id in start..=end {
            let digits = id.checked_ilog10().unwrap_or_default() as usize + 1;
            let divisor = 10_usize.pow(digits as u32 / 2);
            if id / divisor == id % divisor {
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
            let digits = id.checked_ilog10().unwrap_or_default() as usize + 1;
            for chunk_count in 2..=digits {
                if !digits.is_multiple_of(chunk_count) {
                    continue;
                }

                let divisor = 10_usize.pow((digits / chunk_count) as u32);
                let mut remaining = id;
                let first_chunk = remaining % divisor;
                while remaining % divisor == first_chunk {
                    remaining /= divisor;
                }

                if remaining == 0 {
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

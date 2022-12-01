fn solve_first(input: &str) -> u32 {
    input
        .split_terminator("\r\n\r\n")
        .map(|load| {
            load.lines()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum()
        })
        .max()
        .unwrap()
}

fn solve_second(input: &str) -> u32 {
    input
        .split_terminator("\r\n\r\n")
        .map(|load| {
            load.lines()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum::<u32>()
        })
        .fold([0, 0, 0], |mut biggest, mut x| {
            // could be improved by structuring `biggest` into a binary heap but n=3 so I won't bother
            for a in &mut biggest {
                if x > *a {
                    std::mem::swap(&mut x, a)
                }
            }

            biggest
        })
        .into_iter()
        .sum()
}

#[test]
pub fn day01_sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(24_000, solve_first(sample));
    assert_eq!(45_000, solve_second(sample));
}

#[test]
pub fn day01() {
    let input = include_str!("input.txt");
    assert_eq!(70_613, solve_first(input));
    assert_eq!(205_805, solve_second(input));
}

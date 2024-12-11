use std::collections::HashMap;

pub fn solve_second(input: &str, blinks: usize) -> usize {
    let mut stones = input
        .split_whitespace()
        .map(|x| (x.parse::<usize>().unwrap(), 1))
        .collect::<HashMap<_, _>>();
    let mut next_stones = HashMap::new();

    for _ in 0..blinks {
        for (&stone, &count) in &stones {
            if stone == 0 {
                *next_stones.entry(1).or_default() += count;
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let shift = 10_usize.pow((stone.ilog10() + 1) / 2);
                *next_stones.entry(stone / shift).or_default() += count;
                *next_stones.entry(stone % shift).or_default() += count;
            } else {
                *next_stones.entry(stone * 2024).or_default() += count;
            }
        }

        std::mem::swap(&mut stones, &mut next_stones);
        next_stones.clear();
    }

    stones.into_values().sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(55312, solve_second(sample, 25));
    assert_eq!(65601038650482, solve_second(sample, 75));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(184927, solve_second(input, 25));
    assert_eq!(220357186726677, solve_second(input, 75));
}

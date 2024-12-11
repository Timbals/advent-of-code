use std::collections::HashMap;

pub fn solve_first(input: &str) -> usize {
    let mut stones =
        input.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut next_stones = Vec::new();

    for _ in 0..25 {
        for &stone in &stones {
            if stone == 0 {
                next_stones.push(1);
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let left = stone / 10_usize.pow((stone.ilog10() + 1) / 2);
                let right = stone - left * 10_usize.pow((stone.ilog10() + 1) / 2);
                next_stones.push(left);
                next_stones.push(right);
            } else {
                next_stones.push(stone * 2024);
            }
        }

        std::mem::swap(&mut stones, &mut next_stones);
        next_stones.clear();
    }

    stones.len()
}

pub fn solve_second(input: &str) -> usize {
    let mut stones =
        input.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut stones = stones.into_iter().map(|x| (x, 1)).collect::<HashMap<_, _>>();
    let mut next_stones = HashMap::new();

    for _ in 0..75 {
        for (&stone, &count) in &stones {
            if stone == 0 {
                *next_stones.entry(1).or_default() += count;
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let left = stone / 10_usize.pow((stone.ilog10() + 1) / 2);
                let right = stone - left * 10_usize.pow((stone.ilog10() + 1) / 2);
                *next_stones.entry(left).or_default() += count;
                *next_stones.entry(right).or_default() += count;
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
    assert_eq!(55312, solve_first(sample));
    assert_eq!(65601038650482, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(184927, solve_first(input));
    assert_eq!(220357186726677, solve_second(input));
}

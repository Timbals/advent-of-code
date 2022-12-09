use itertools::Itertools;
use std::cmp::max;

pub fn parse(input: &str) -> (usize, Vec<i32>) {
    (
        input.lines().next().unwrap().len(),
        input
            .lines()
            .flat_map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as _))
            .collect(),
    )
}

/// rotates a given `size`x`size` matrix stored in `slice` by 90° degrees
pub fn rotate<T: Copy>(slice: &mut [T], size: usize) {
    // rotate by flipping twice (first along a diagonal then along a horizontal)
    for y in 0..size {
        for x in 0..y {
            slice.swap(x + y * size, y + x * size);
        }
    }
    for y in 0..(size / 2) {
        for x in 0..size {
            slice.swap(x + y * size, x + (size - y - 1) * size);
        }
    }
}

pub fn solve_first(input: &str) -> usize {
    let (size, mut trees) = parse(input);
    let mut visible = vec![false; trees.len()];

    for _ in 0..4 {
        for y in 0..size {
            let mut highest = -1;
            for x in 0..size {
                let height = trees[x + y * size];
                if height > highest {
                    visible[x + y * size] |= true;
                }
                highest = max(height, highest);
            }
        }

        rotate(&mut trees, size);
        rotate(&mut visible, size);
    }

    visible.into_iter().filter(|&x| x).count()
}

pub fn solve_second(input: &str) -> usize {
    let (size, mut trees) = parse(input);
    let mut score = vec![1; trees.len()];

    for _ in 0..4 {
        for y in 0..size {
            for x in 0..size {
                let height = trees[x + y * size];
                let mut iter = trees[x + y * size + 1..y * size + size].iter().peekable();
                let smaller = iter.peeking_take_while(|&&x| height > x).count();
                let blocker = usize::from(iter.next().is_some());
                score[x + y * size] *= smaller + blocker;
            }
        }

        rotate(&mut trees, size);
        rotate(&mut score, size);
    }

    score.into_iter().max().unwrap_or_default()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(21, solve_first(sample));
    assert_eq!(8, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1851, solve_first(input));
    assert_eq!(574080, solve_second(input));
}

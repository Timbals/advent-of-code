use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub fn solve_first(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut heap = BinaryHeap::<Reverse<(usize, usize, usize, usize, usize)>>::new();
    // (heat, x, y, last_dir, consecutive)
    heap.push(Reverse((0, 0, 0, 0, 0)));

    let mut seen = HashSet::new();

    while let Some(Reverse((heat, x, y, last_dir, consecutive))) = heap.pop() {
        if (x == width - 1) && (y == height - 1) {
            return heat;
        }

        heap.extend(
            [
                (x + 1, y),
                (x, y + 1),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_sub(1)),
            ]
            .into_iter()
            .enumerate()
            .filter(|(_, (x, y))| *x < width && *y < height) // bounds check
            .filter(|(dir, _)| *dir != (last_dir + 2) % 4) // can't turn back
            .map(|(dir, coords)| {
                if dir == last_dir {
                    (dir, coords, consecutive + 1)
                } else {
                    (dir, coords, 1)
                }
            })
            .filter(|(_, _, consecutive)| *consecutive <= 3) // at most 3 blocks in one direction
            .filter(|(dir, (x, y), consecutive)| seen.insert((*x, *y, *dir, *consecutive)))
            .map(|(dir, (x, y), consecutive)| Reverse((heat + grid[y][x], x, y, dir, consecutive))),
        )
    }

    unreachable!()
}

pub fn solve_second(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut heap = BinaryHeap::<Reverse<(usize, usize, usize, usize, usize)>>::new();
    // (heat, x, y, last_direction, consecutive)
    heap.push(Reverse((0, 0, 0, 0, 0)));

    let mut seen = HashSet::new();

    while let Some(Reverse((heat, x, y, last_dir, consecutive))) = heap.pop() {
        if (x == width - 1) && (y == height - 1) && consecutive >= 4 {
            return heat;
        }

        heap.extend(
            [
                (x + 1, y),
                (x, y + 1),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_sub(1)),
            ]
            .into_iter()
            .enumerate()
            .filter(|(_, (x, y))| *x < width && *y < height) // bounds check
            .filter(|(dir, _)| *dir != (last_dir + 2) % 4) // can't turn back
            .filter(|(dir, _)| *dir == last_dir || consecutive >= 4) // move at least 4 blocks
            .map(|(dir, coords)| {
                if dir == last_dir {
                    (dir, coords, consecutive + 1)
                } else {
                    (dir, coords, 1)
                }
            })
            .filter(|(_, _, consecutive)| *consecutive <= 10) // at most 10 blocks in one direction
            .filter(|(dir, (x, y), consecutive)| seen.insert((*x, *y, *dir, *consecutive)))
            .map(|(dir, (x, y), consecutive)| Reverse((heat + grid[y][x], x, y, dir, consecutive))),
        )
    }

    unreachable!()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(102, solve_first(sample));
    assert_eq!(94, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(959, solve_first(input));
    assert_eq!(1135, solve_second(input));
}

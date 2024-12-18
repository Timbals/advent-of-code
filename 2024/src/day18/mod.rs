use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

pub fn solve_first(input: &str, size: isize, limit: usize) -> usize {
    let mut visited = input
        .lines()
        .take(limit)
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
        })
        .collect::<BTreeSet<_>>();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0, 0));
    while let Some((steps, x, y)) = queue.pop() {
        if x == size - 1 && y == size - 1 {
            return steps.0;
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nx, ny) = (x + dx, y + dy);
            if (0..size).contains(&nx)
                && (0..size).contains(&ny)
                && !visited.iter().contains(&(nx, ny))
            {
                visited.insert((nx, ny));
                queue.push((Reverse(steps.0 + 1), nx, ny));
            }
        }
    }

    unreachable!("no valid path")
}

pub fn solve_second(input: &str, size: isize) -> String {
    let mut visited_template = BTreeSet::new();
    'outer: for (new_x, new_y) in input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
    }) {
        visited_template.insert((new_x, new_y));
        let mut visited = visited_template.clone();

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), 0, 0));
        while let Some((steps, x, y)) = queue.pop() {
            if x == size - 1 && y == size - 1 {
                continue 'outer;
            }

            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (nx, ny) = (x + dx, y + dy);
                if (0..size).contains(&nx)
                    && (0..size).contains(&ny)
                    && !visited.iter().contains(&(nx, ny))
                {
                    visited.insert((nx, ny));
                    queue.push((Reverse(steps.0 + 1), nx, ny));
                }
            }
        }

        return format!("{new_x},{new_y}");
    }

    unreachable!("no byte blocks the path")
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(22, solve_first(sample, 7, 12));
    assert_eq!("6,1", solve_second(sample, 7));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(354, solve_first(input, 71, 1024));
    assert_eq!("36,17", solve_second(input, 71));
}

use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_first(input: &str) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    let antennas = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().map(move |(x, b)| (*b, (x as isize, y as isize)))
        })
        .filter(|(b, _)| *b != b'.')
        .into_group_map();

    let mut antinodes = HashSet::new();

    for (_, antennas) in antennas {
        for ((x1, y1), (x2, y2)) in antennas.iter().tuple_combinations() {
            let (x, y) = (x2 + (x2 - x1), y2 + (y2 - y1));
            if (0..width).contains(&x) && (0..height).contains(&y) {
                antinodes.insert((x, y));
            }
            let (x, y) = (x1 + (x1 - x2), y1 + (y1 - y2));
            if (0..width).contains(&x) && (0..height).contains(&y) {
                antinodes.insert((x, y));
            }
        }
    }

    antinodes.len()
}

pub fn solve_second(input: &str) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    let antennas = grid
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().map(move |(x, b)| (*b, (x as isize, y as isize)))
        })
        .filter(|(b, _)| *b != b'.')
        .into_group_map();

    let mut antinodes = HashSet::new();

    for (_, antennas) in antennas {
        for ((x1, y1), (x2, y2)) in antennas.iter().tuple_combinations() {
            let (dx, dy) = (x2 - x1, y2 - y1);
            let mut i = 0;
            while (0..width).contains(&(x2 + i * dx)) && (0..height).contains(&(y2 + i * dy)) {
                antinodes.insert((x2 + i * dx, y2 + i * dy));
                i += 1;
            }

            let (dx, dy) = (x1 - x2, y1 - y2);
            let mut i = 0;
            while (0..width).contains(&(x1 + i * dx)) && (0..height).contains(&(y1 + i * dy)) {
                antinodes.insert((x1 + i * dx, y1 + i * dy));
                i += 1;
            }
        }
    }

    antinodes.len()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(14, solve_first(sample));
    assert_eq!(34, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(214, solve_first(input));
    assert_eq!(809, solve_second(input));
}

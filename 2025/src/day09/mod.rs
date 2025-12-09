use itertools::Itertools;
use std::cmp::{max, min};

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect()
}

pub fn solve_first(input: &str) -> usize {
    let red_tiles = parse(input);

    red_tiles
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        .max()
        .unwrap()
}

pub fn solve_second(input: &str) -> usize {
    let red_tiles = parse(input);

    red_tiles
        .iter()
        .tuple_combinations()
        .filter(|&(&(x1, y1), &(x2, y2))| {
            // technically need to check this isn't like ((9, 7), (2, 5)) in the sample input
            let (x_min, x_max) = (min(x1, x2), max(x1, x2));
            let (y_min, y_max) = (min(y1, y2), max(y1, y2));

            red_tiles.iter().circular_tuple_windows().all(|(&(x3, y3), &(x4, y4))| {
                // check that no edges are inside the rectangle,
                // technically not sufficient because it doesn't account for "snaking" edges
                (x3 <= x_min && x4 <= x_min)
                    || (x3 >= x_max && x4 >= x_max)
                    || (y3 <= y_min && y4 <= y_min)
                    || (y3 >= y_max && y4 >= y_max)
            })
        })
        .map(|(&(x1, y1), &(x2, y2))| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        .max()
        .unwrap()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(50, solve_first(sample));
    assert_eq!(24, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(4759531084, solve_first(input));
    assert_eq!(1539238860, solve_second(input));
}

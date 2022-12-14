use itertools::Itertools;
use std::cmp::{max, min};

pub fn parse(input: &str) -> Vec<Vec<bool>> {
    let mut max_y = 0;

    let rocks = input
        .lines()
        .flat_map(|line| {
            let mut positions = Vec::new();
            for ((x1, y1), (x2, y2)) in line
                .split(" -> ")
                .map(|pos| pos.split_once(',').unwrap())
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .tuple_windows()
            {
                for x in min(x1, x2)..=max(x1, x2) {
                    for y in min(y1, y2)..=max(y1, y2) {
                        max_y = max(max_y, y);
                        positions.push((x, y))
                    }
                }
            }
            positions
        })
        .collect::<Vec<(usize, usize)>>();

    let mut tiles = vec![vec![false; max_y + 2]; 1000];

    for (x, y) in rocks.into_iter() {
        tiles[x][y] = true;
    }

    tiles
}

pub fn solve_first(input: &str) -> usize {
    let mut tiles = parse(input);

    let mut count = 0;
    let (mut x, mut y) = (500, 0);
    loop {
        if y == tiles[500].len() - 2 {
            break count;
        }

        if let Some(nx) = [x, x - 1, x + 1].into_iter().find(|x| !tiles[*x][y + 1]) {
            (x, y) = (nx, y + 1);
        } else {
            tiles[x][y] = true;
            count += 1;
            (x, y) = (500, 0);
        }
    }
}

pub fn solve_second(input: &str) -> usize {
    let mut tiles = parse(input);

    let mut count = 0;
    let (mut x, mut y) = (500, 0);
    loop {
        if y == tiles[500].len() - 1 {
            tiles[x][y] = true;
            (x, y) = (500, 0);
            count += 1;
        }

        if let Some(nx) = [x, x - 1, x + 1].into_iter().find(|x| !tiles[*x][y + 1]) {
            (x, y) = (nx, y + 1);
        } else {
            tiles[x][y] = true;
            count += 1;

            if (x, y) == (500, 0) {
                break count;
            }

            (x, y) = (500, 0);
        }
    }
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(24, solve_first(sample));
    assert_eq!(93, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(614, solve_first(input));
    assert_eq!(26170, solve_second(input));
}

use itertools::Itertools;
use std::cmp::max;

pub fn solve_first(input: &str) -> usize {
    let mut max_y = 0;

    let rocks = input
        .lines()
        .flat_map(|line| {
            let mut positions = Vec::new();
            for ((x1, y1), (x2, y2)) in line
                .split(" -> ")
                .map(|pos| pos.split_once(',').unwrap())
                .tuple_windows()
            {
                let mut x = x1.parse::<usize>().unwrap();
                let mut y = y1.parse::<usize>().unwrap();
                let x2 = x2.parse::<usize>().unwrap();
                let y2 = y2.parse::<usize>().unwrap();

                positions.push((x, y));
                max_y = max(max_y, y);
                while x != x2 || y != y2 {
                    x = (x as isize + (x2 as isize - x as isize).min(1).max(-1)) as usize;
                    y = (y as isize + (y2 as isize - y as isize).min(1).max(-1)) as usize;
                    max_y = max(max_y, y);
                    positions.push((x, y))
                }
            }
            positions
        })
        .collect_vec();

    #[derive(Copy, Clone, Debug)]
    enum Tile {
        Rock,
        Sand,
    }

    let mut tiles = vec![vec![None; max_y + 1]; 1000];

    for (x, y) in rocks.into_iter() {
        tiles[x][y] = Some(Tile::Rock);
    }

    let mut count = 0;
    let mut current = (500, 0);
    loop {
        if current.1 == max_y {
            break count;
        }

        if tiles[current.0][current.1 + 1].is_none() {
            current = (current.0, current.1 + 1);
        } else if tiles[current.0 - 1][current.1 + 1].is_none() {
            current = (current.0 - 1, current.1 + 1);
        } else if tiles[current.0 + 1][current.1 + 1].is_none() {
            current = (current.0 + 1, current.1 + 1);
        } else {
            tiles[current.0][current.1] = Some(Tile::Sand);
            current = (500, 0);
            count += 1;
        }
    }
}

pub fn solve_second(input: &str) -> usize {
    let mut max_y = 0;

    let rocks = input
        .lines()
        .flat_map(|line| {
            let mut positions = Vec::new();
            for ((x1, y1), (x2, y2)) in line
                .split(" -> ")
                .map(|pos| pos.split_once(',').unwrap())
                .tuple_windows()
            {
                let mut x = x1.parse::<usize>().unwrap();
                let mut y = y1.parse::<usize>().unwrap();
                let x2 = x2.parse::<usize>().unwrap();
                let y2 = y2.parse::<usize>().unwrap();

                positions.push((x, y));
                max_y = max(max_y, y);
                while x != x2 || y != y2 {
                    x = (x as isize + (x2 as isize - x as isize).min(1).max(-1)) as usize;
                    y = (y as isize + (y2 as isize - y as isize).min(1).max(-1)) as usize;
                    max_y = max(max_y, y);
                    positions.push((x, y))
                }
            }
            positions
        })
        .collect_vec();

    #[derive(Copy, Clone, Debug)]
    enum Tile {
        Rock,
        Sand,
    }

    max_y += 1;

    let mut tiles = vec![vec![None; max_y + 1]; 1000];

    for (x, y) in rocks.into_iter() {
        tiles[x][y] = Some(Tile::Rock);
    }

    let mut count = 0;
    let mut current = (500, 0);
    loop {
        if current.1 == max_y {
            tiles[current.0][current.1] = Some(Tile::Sand);
            current = (500, 0);
            count += 1;
        }

        if tiles[current.0][current.1 + 1].is_none() {
            current = (current.0, current.1 + 1);
        } else if tiles[current.0 - 1][current.1 + 1].is_none() {
            current = (current.0 - 1, current.1 + 1);
        } else if tiles[current.0 + 1][current.1 + 1].is_none() {
            current = (current.0 + 1, current.1 + 1);
        } else {
            tiles[current.0][current.1] = Some(Tile::Sand);
            count += 1;

            if current == (500, 0) {
                break count;
            }

            current = (500, 0);
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

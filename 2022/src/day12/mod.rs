use itertools::Itertools;
use std::cmp::min;
use std::collections::VecDeque;

pub fn solve_first(input: &str) -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut board = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, elevation)| {
                    (
                        match elevation {
                            b'S' => {
                                start = (x, y);
                                0
                            }
                            b'E' => {
                                end = (x, y);
                                25
                            }
                            elevation => elevation - b'a',
                        },
                        None,
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    let mut queue = VecDeque::new();

    board[start.1][start.0].1 = Some(0);
    queue.push_back(start);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        let (elevation, distance) = board[current.1][current.0];
        let distance = distance.unwrap();

        if current == end {
            return distance;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
            if current.0 == 0 && dx == -1 || current.1 == 0 && dy == -1 {
                continue;
            }

            let (x, y) = (
                (current.0 as isize + dx) as usize,
                (current.1 as isize + dy) as usize,
            );

            if let Some(line) = board.get_mut(y) {
                if let Some(tile) = line.get_mut(x) {
                    if tile.1.is_none() && tile.0 <= elevation + 1 {
                        tile.1 = Some(distance + 1);
                        queue.push_back((x, y));
                    }
                }
            }
        }
    }

    // destination is literally unreachable :)
    unreachable!()
}

pub fn solve_second(input: &str) -> usize {
    let mut start = (0, 0);
    let mut board = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, elevation)| {
                    (
                        match elevation {
                            b'S' => 0,
                            b'E' => {
                                start = (x, y);
                                25
                            }
                            elevation => elevation - b'a',
                        },
                        None,
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    let mut queue = VecDeque::new();

    board[start.1][start.0].1 = Some(0);
    queue.push_back(start);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        let (elevation, distance) = board[current.1][current.0];
        let distance = distance.unwrap();

        if elevation == 0 {
            return distance;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
            if current.0 == 0 && dx == -1 || current.1 == 0 && dy == -1 {
                continue;
            }

            let (x, y) = (
                (current.0 as isize + dx) as usize,
                (current.1 as isize + dy) as usize,
            );

            if let Some(line) = board.get_mut(y) {
                if let Some(tile) = line.get_mut(x) {
                    if tile.1.is_none() && tile.0 + 1 >= elevation {
                        tile.1 = Some(distance + 1);
                        queue.push_back((x, y));
                    }
                }
            }
        }
    }

    unreachable!()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(31, solve_first(sample));
    assert_eq!(29, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(447, solve_first(input));
    assert_eq!(446, solve_second(input));
}

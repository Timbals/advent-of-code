use itertools::Itertools;
use std::collections::VecDeque;

pub fn solve(input: &str) -> (usize, usize) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut board = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, elevation)| match elevation {
                    b'S' => {
                        start = (x, y);
                        0
                    }
                    b'E' => {
                        end = (x, y);
                        25
                    }
                    elevation => elevation - b'a',
                })
                .map(|elevation| (elevation, None))
                .collect_vec()
        })
        .collect_vec();

    board[end.1][end.0].1 = Some(0);
    let mut queue = VecDeque::from([end]);

    let mut part1 = 0;
    let mut part2 = 0;

    while part1 == 0 || part2 == 0 {
        let (x, y) = queue.pop_front().unwrap();
        let (elevation, distance) = board[y][x];

        if part1 == 0 && (x, y) == start {
            part1 = distance.unwrap();
        }
        if part2 == 0 && elevation == 0 {
            part2 = distance.unwrap();
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);

            if let Some(tile) = board.get_mut(y).and_then(|row| row.get_mut(x)) {
                if tile.1.is_none() && tile.0 + 1 >= elevation {
                    tile.1 = Some(distance.unwrap() + 1);
                    queue.push_back((x, y));
                }
            }
        }
    }

    (part1, part2)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!((31, 29), solve(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!((447, 446), solve(input));
}

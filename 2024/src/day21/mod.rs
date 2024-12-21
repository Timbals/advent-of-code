use itertools::{chain, repeat_n, Itertools};
use std::iter::once;

pub fn solve<const ROBOTS: usize>(input: &str) -> u64 {
    let mut dir_lookup = [[[0_u64; 5]; 5]; ROBOTS];
    for (from, to) in Itertools::cartesian_product(0..5, 0..5) {
        dir_lookup[0][from][to] = 1;
    }

    let coordinates = [(2, 0), (1, 0), (0, 1), (1, 1), (2, 1)];
    for i in 1..ROBOTS {
        for (from, to) in Itertools::cartesian_product(0..5, 0..5) {
            let dx = coordinates[to].0 - coordinates[from].0;
            let dy = coordinates[to].1 - coordinates[from].1;
            let up = repeat_n(1, (-dy).max(0) as usize);
            let left = repeat_n(2, (-dx).max(0) as usize);
            let down = repeat_n(3, dy.max(0) as usize);
            let right = repeat_n(4, dx.max(0) as usize);
            let path = chain!(once(0), right, down, left, up, once(0)).tuple_windows();
            dir_lookup[i][from][to] = path.clone().map(|(a, b)| dir_lookup[i - 1][a][b]).sum();
            // check the reverse path if it doesn't intersect the gap
            if dy == 0 || (coordinates[from].0 != 0 && coordinates[to].0 != 0) {
                dir_lookup[i][from][to] =
                    dir_lookup[i][from][to].min(path.map(|(a, b)| dir_lookup[i - 1][b][a]).sum());
            }
        }
    }

    let mut num_lookup = [[0_u64; 11]; 11];
    let coordinates =
        [(2, 3), (1, 3), (0, 2), (1, 2), (2, 2), (0, 1), (1, 1), (2, 1), (0, 0), (1, 0), (2, 0)];
    for (from, to) in Itertools::cartesian_product(0..11, 0..11) {
        let dx = coordinates[to].0 - coordinates[from].0;
        let dy = coordinates[to].1 - coordinates[from].1;
        let up = repeat_n(1, (-dy).max(0) as usize);
        let left = repeat_n(2, (-dx).max(0) as usize);
        let down = repeat_n(3, dy.max(0) as usize);
        let right = repeat_n(4, dx.max(0) as usize);
        let path = chain!(once(0), right, up, left, down, once(0)).tuple_windows();
        num_lookup[from][to] = path.clone().map(|(a, b)| dir_lookup[ROBOTS - 1][a][b]).sum();
        // check the reverse path if it doesn't intersect the gap
        if !((coordinates[from].0 == 0 && coordinates[to].1 == 3)
            || (coordinates[to].0 == 0 && coordinates[from].1 == 3))
        {
            num_lookup[from][to] =
                num_lookup[from][to].min(path.map(|(a, b)| dir_lookup[ROBOTS - 1][b][a]).sum());
        }
    }

    input
        .lines()
        .map(|line| {
            let numeric = line[..3].parse::<u64>().unwrap();
            let shortest = once(0)
                .chain(line.chars().map(|c| match c {
                    'A' => 0,
                    c => c.to_digit(10).unwrap() as usize + 1,
                }))
                .tuple_windows()
                .map(|(from, to)| num_lookup[from][to])
                .sum::<u64>();

            shortest * numeric
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(126384, solve::<3>(sample));
    assert_eq!(154115708116294, solve::<26>(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(197560, solve::<3>(input));
    assert_eq!(242337182910752, solve::<26>(input));
}

use itertools::Itertools;
use std::cmp::max;
use std::collections::VecDeque;
use std::iter::once;

const SIDES: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

#[allow(clippy::type_complexity)]
pub fn parse(input: &str) -> (Vec<(usize, usize, usize)>, Vec<Vec<Vec<bool>>>) {
    let cubes = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|v| v.parse::<usize>().unwrap() + 1)
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
    for (x, y, z) in &cubes {
        max_x = max(max_x, *x + 2);
        max_y = max(max_y, *y + 2);
        max_z = max(max_z, *z + 2);
    }

    let mut space = vec![vec![vec![false; max_z]; max_y]; max_x];
    for (x, y, z) in &cubes {
        space[*x][*y][*z] = true;
    }

    (cubes, space)
}

pub fn solve_first(input: &str) -> usize {
    let (cubes, space) = parse(input);

    let mut score = 0;
    for (x, y, z) in cubes.into_iter() {
        for (dx, dy, dz) in SIDES {
            let x = x.wrapping_add_signed(dx);
            let y = y.wrapping_add_signed(dy);
            let z = z.wrapping_add_signed(dz);

            if !*space
                .get(x)
                .and_then(|v| v.get(y))
                .and_then(|v| v.get(z))
                .unwrap_or(&false)
            {
                score += 1;
            }
        }
    }

    score
}

pub fn solve_second(input: &str) -> usize {
    let (_, space) = parse(input);

    let mut score = 0;
    let mut queue = VecDeque::from_iter(once((0, 0, 0)));
    let mut visited = vec![vec![vec![false; space[0][0].len()]; space[0].len()]; space.len()];
    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();

        if *visited
            .get(x)
            .and_then(|v| v.get(y))
            .and_then(|v| v.get(z))
            .unwrap_or(&true)
        {
            continue;
        }

        if space[x][y][z] {
            score += 1;
            continue;
        }

        visited[x][y][z] = true;

        for (dx, dy, dz) in SIDES {
            queue.push_back((
                x.wrapping_add_signed(dx),
                y.wrapping_add_signed(dy),
                z.wrapping_add_signed(dz),
            ));
        }
    }

    score
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(64, solve_first(sample));
    assert_eq!(58, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(3466, solve_first(input));
    assert_eq!(2012, solve_second(input));
}

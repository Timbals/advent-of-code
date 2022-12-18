use itertools::Itertools;
use std::cmp::max;
use std::collections::VecDeque;
use std::iter::once;

pub fn solve_first(input: &str) -> usize {
    let cubes = input
        .lines()
        .map(|line| {
            let mut values = line.split(',').map(|v| v.parse::<usize>().unwrap());
            (
                values.next().unwrap(),
                values.next().unwrap(),
                values.next().unwrap(),
            )
        })
        .collect_vec();

    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
    for (x, y, z) in &cubes {
        max_x = max(max_x, *x + 1);
        max_y = max(max_y, *y + 1);
        max_z = max(max_z, *z + 1);
    }

    let mut space = vec![vec![vec![false; max_z]; max_y]; max_x];

    for (x, y, z) in &cubes {
        space[*x][*y][*z] = true;
    }

    let sides = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    let mut count = 0;
    for (x, y, z) in cubes.into_iter() {
        for (dx, dy, dz) in sides {
            let x = x.wrapping_add_signed(dx);
            let y = y.wrapping_add_signed(dy);
            let z = z.wrapping_add_signed(dz);

            if !*space
                .get(x)
                .and_then(|v| v.get(y))
                .and_then(|v| v.get(z))
                .unwrap_or(&false)
            {
                count += 1;
            }
        }
    }

    count
}

pub fn solve_second(input: &str) -> usize {
    let cubes = input
        .lines()
        .map(|line| {
            let mut values = line.split(',').map(|v| v.parse::<usize>().unwrap());
            (
                values.next().unwrap() + 1,
                values.next().unwrap() + 1,
                values.next().unwrap() + 1,
            )
        })
        .collect_vec();

    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
    for (x, y, z) in &cubes {
        max_x = max(max_x, *x + 3);
        max_y = max(max_y, *y + 3);
        max_z = max(max_z, *z + 3);
    }

    let mut space = vec![vec![vec![false; max_z]; max_y]; max_x];

    for (x, y, z) in &cubes {
        space[*x][*y][*z] = true;
    }

    let sides = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    let mut count = 0;
    let mut queue = VecDeque::from_iter(once((0, 0, 0)));
    let mut visited = vec![vec![vec![false; max_z]; max_y]; max_x];
    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();

        if space[x][y][z] {
            count += 1;
            continue;
        }

        if visited[x][y][z] {
            continue;
        }
        visited[x][y][z] = true;

        for (dx, dy, dz) in sides {
            if let (Some(x), Some(y), Some(z)) = (
                x.checked_add_signed(dx),
                y.checked_add_signed(dy),
                z.checked_add_signed(dz),
            ) {
                if x < max_x && y < max_y && z < max_z {
                    queue.push_back((x, y, z));
                }
            }
        }
    }

    count
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

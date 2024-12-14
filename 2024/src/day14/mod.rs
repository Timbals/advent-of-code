use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BTreeSet;

fn parse(input: &str) -> impl Iterator<Item = (isize, isize, isize, isize)> + use<'_> {
    input.lines().map(|line| {
        let (p, v) = line.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
        let (px, py) = p.split_once(',').unwrap();
        let (px, py) = (px.parse::<isize>().unwrap(), py.parse::<isize>().unwrap());
        let (vx, vy) = v.split_once(',').unwrap();
        let (vx, vy) = (vx.parse::<isize>().unwrap(), vy.parse::<isize>().unwrap());
        (px, py, vx, vy)
    })
}

pub fn solve_first(input: &str, width: isize, height: isize) -> usize {
    let robots = parse(input);

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for (px, py, vx, vy) in robots {
        let px = (px + 100 * vx).rem_euclid(width);
        let py = (py + 100 * vy).rem_euclid(height);
        match (px.cmp(&(width / 2)), py.cmp(&(height / 2))) {
            (Ordering::Less, Ordering::Less) => q1 += 1,
            (Ordering::Greater, Ordering::Less) => q2 += 1,
            (Ordering::Less, Ordering::Greater) => q3 += 1,
            (Ordering::Greater, Ordering::Greater) => q4 += 1,
            _ => {}
        }
    }

    q1 * q2 * q3 * q4
}

pub fn solve_second(input: &str, width: isize, height: isize) -> isize {
    let robots = parse(input).collect::<Vec<_>>();

    (0..10000)
        .max_by_key(|i| {
            let mut positions = BTreeSet::new();
            for (px, py, vx, vy) in &robots {
                let px = (px + i * vx).rem_euclid(width);
                let py = (py + i * vy).rem_euclid(height);
                positions.insert((px, py));
            }

            // detect very long run of pixels
            positions
                .into_iter()
                .map(|(x, y)| (x, y, 1))
                .coalesce(|prev, curr| {
                    if prev.0 == curr.0 && prev.1.abs_diff(curr.1) == 1 {
                        Ok((curr.0, curr.1, prev.2 + 1))
                    } else {
                        Err((prev, curr))
                    }
                })
                .max_by_key(|(_, _, run_length)| *run_length)
                .unwrap()
                .2
        })
        .unwrap()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(12, solve_first(sample, 11, 7));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(219150360, solve_first(input, 101, 103));
    assert_eq!(8053, solve_second(input, 101, 103));
}

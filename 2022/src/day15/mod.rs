use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;

pub fn parse(input: &str) -> impl Iterator<Item = ((isize, isize), (isize, isize))> + Clone + '_ {
    input.lines().map(|line| {
        let mut line = line.split(['=', ',', ':']);
        line.next();
        let sx = line.next().unwrap().parse::<isize>().unwrap();
        line.next();
        let sy = line.next().unwrap().parse::<isize>().unwrap();
        line.next();
        let bx = line.next().unwrap().parse::<isize>().unwrap();
        line.next();
        let by = line.next().unwrap().parse::<isize>().unwrap();

        ((sx, sy), (bx, by))
    })
}

pub fn solve_first(input: &str, y: isize) -> usize {
    let sensors = parse(input);
    let mut radii = Vec::new();
    let mut beacons = HashSet::new();
    for ((sx, sy), (bx, by)) in sensors {
        let radius = (sx - bx).abs() + (sy - by).abs();
        let diff = (radius - (y - sy).abs()).max(-1);
        radii.push(sx - diff..=sx + diff);

        if by == y {
            beacons.insert(bx);
        }
    }

    radii.sort_by_key(|radius| *radius.start());

    // add the length of the radii and ignore overlapping parts of the radii using `last_end`
    let mut count = 0;
    let mut last_end = isize::MIN;
    for radius in radii.into_iter() {
        count += max(*radius.end() + 1, last_end) - max(*radius.start(), last_end);
        last_end = max(*radius.end() + 1, last_end);
    }
    count -= beacons.len() as isize;

    count as usize
}

pub fn solve_second(input: &str, limit: isize) -> isize {
    let sensors = parse(input)
        .map(|((sx, sy), (bx, by))| (sx, sy, (sx - bx).abs() + (sy - by).abs()))
        .collect_vec();

    for (sx, sy, radius) in sensors.iter().copied() {
        // check all points on the bottom-left diagonal just outside of the sensor radius
        for delta in max(0, -(sx - radius - 1))..=min(radius + 1, limit - sy) {
            let (x, y) = (sx - radius - 1 + delta, sy + delta);

            if sensors
                .iter()
                .copied()
                .all(|(sx, sy, radius)| (sx - x).abs() + (sy - y).abs() > radius)
            {
                return x * 4000000 + y;
            }
        }
    }

    unreachable!() // beacon is in one of the corners or at the top or right corners of the sensor radius
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(26, solve_first(sample, 10));
    assert_eq!(56000011, solve_second(sample, 20));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(5100463, solve_first(input, 2000000));
    assert_eq!(11557863040754, solve_second(input, 4000000));
}

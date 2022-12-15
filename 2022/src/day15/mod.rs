use std::cmp::min;
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
    let mut coverage = HashSet::new();
    let mut beacons = HashSet::new();
    for ((sx, sy), (bx, by)) in sensors {
        let range = (sx - bx).abs() + (sy - by).abs();
        let sensor_y = (y - sy).abs();
        let diff = (range - sensor_y).max(-1);
        coverage.extend(sx - diff..=sx + diff);

        if by == y {
            beacons.insert(bx);
        }
    }

    coverage.difference(&beacons).count()
}

pub fn solve_second(input: &str, limit: isize) -> isize {
    let sensors = parse(input);
    let mut candidates = HashSet::new();
    for ((sx, sy), (bx, by)) in sensors.clone() {
        let range = (sx - bx).abs() + (sy - by).abs();
        let start_x = sx - range - 1;
        let start_y = sy;
        for delta in 0..=(range + 1) {
            let (x, y) = (start_x + delta, start_y + delta);
            if 0 <= x && x <= limit && 0 <= y && y <= limit {
                candidates.insert((start_x + delta, start_y + delta));
            }
        }
    }

    for ((sx, sy), (bx, by)) in sensors {
        let range = (sx - bx).abs() + (sy - by).abs();
        candidates.retain(|(x, y)| (sx - x).abs() + (sy - y).abs() > range);
    }

    debug_assert!(candidates.len() == 1);
    let (x, y) = candidates.into_iter().next().unwrap();
    x * 4000000 + y
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

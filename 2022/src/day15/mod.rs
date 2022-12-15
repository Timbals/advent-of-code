use itertools::Itertools;
use std::cmp::max;
use std::collections::HashSet;

pub fn parse(input: &str) -> impl Iterator<Item = ((isize, isize), (isize, isize))> + '_ {
    input.lines().map(|line| {
        let mut line = line.split(['=', ',', ':']).skip(1).step_by(2);
        let sx = line.next().unwrap().parse::<isize>().unwrap();
        let sy = line.next().unwrap().parse::<isize>().unwrap();
        let bx = line.next().unwrap().parse::<isize>().unwrap();
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

    let mut increasing_lines = Vec::new(); // lines with slope=1 just outside the sensor ranges
    let mut decreasing_lines = Vec::new(); // lines with slope=-1 just outside the sensor ranges
    for (sx, sy, range) in sensors.iter().copied() {
        increasing_lines.push(sy - sx + range + 1); // bottom-right
        increasing_lines.push(sy - sx - range - 1); // top-left
        decreasing_lines.push(sy + range + 1 + sx); // top-right
        decreasing_lines.push(sy - range - 1 + sx); // bottom-left
    }

    for increasing in increasing_lines.into_iter() {
        for decreasing in decreasing_lines.iter().copied() {
            let (x, y) = ((decreasing - increasing) / 2, (decreasing + increasing) / 2); // intersection of the two lines
            if x < 0 || x > limit || y < 0 || y > limit {
                continue;
            }
            if sensors
                .iter()
                .copied()
                .all(|(sx, sy, radius)| (sx - x).abs() + (sy - y).abs() > radius)
            {
                return x * 4000000 + y;
            }
        }
    }

    unreachable!() // beacon is in one of the corners
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

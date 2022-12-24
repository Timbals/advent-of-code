use ahash::AHashSet;
use itertools::Itertools;
use std::mem::swap;

pub fn solve_first(input: &str) -> usize {
    let mut blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(x, v)| match v {
                    b'>' => Some((x, y, 1, 0)),
                    b'<' => Some((x, y, -1, 0)),
                    b'^' => Some((x, y, 0, -1)),
                    b'v' => Some((x, y, 0, 1)),
                    _ => None,
                })
        })
        .collect_vec();
    let mut occupied: AHashSet<_> = blizzards.iter().map(|(x, y, _, _)| (*x, *y)).collect();

    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;

    let mut time = 0;

    let mut step = |occupied: &mut AHashSet<(usize, usize)>| {
        for (x, y, dx, dy) in &mut blizzards {
            *x = ((*x as isize + *dx - 1).rem_euclid(width - 2) + 1) as usize;
            *y = ((*y as isize + *dy - 1).rem_euclid(height - 2) + 1) as usize;
        }
        occupied.clear();
        occupied.extend(blizzards.iter().map(|(x, y, _, _)| (*x, *y)));
    };

    let start = (1, 0);
    let end = (width - 2, height - 1);

    let mut previous_positions = AHashSet::from([start]);
    let mut next_positions = AHashSet::new();

    loop {
        step(&mut occupied);
        time += 1;
        for (x, y) in previous_positions.drain() {
            for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (x, y) = (x + dx, y + dy);

                if (x, y) == end {
                    return time;
                }

                if x > 0
                    && y > 0
                    && x < width - 1
                    && y < height - 1
                    && !occupied.contains(&(x as usize, y as usize))
                {
                    next_positions.insert((x, y));
                }
            }
        }
        swap(&mut previous_positions, &mut next_positions);
    }
}

pub fn solve_second(input: &str) -> usize {
    let mut blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(x, v)| match v {
                    b'>' => Some((x, y, 1, 0)),
                    b'<' => Some((x, y, -1, 0)),
                    b'^' => Some((x, y, 0, -1)),
                    b'v' => Some((x, y, 0, 1)),
                    _ => None,
                })
        })
        .collect_vec();
    let mut occupied: AHashSet<_> = blizzards.iter().map(|(x, y, _, _)| (*x, *y)).collect();

    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;

    let mut time = 0;

    let mut step = |occupied: &mut AHashSet<(usize, usize)>| {
        for (x, y, dx, dy) in &mut blizzards {
            *x = ((*x as isize + *dx - 1).rem_euclid(width - 2) + 1) as usize;
            *y = ((*y as isize + *dy - 1).rem_euclid(height - 2) + 1) as usize;
        }
        occupied.clear();
        occupied.extend(blizzards.iter().map(|(x, y, _, _)| (*x, *y)));
    };

    let start = (1, 0);
    let mut count = 0;
    let end = (width - 2, height - 1);
    let mut target = end;

    let mut previous_positions = AHashSet::from([start]);
    let mut next_positions = AHashSet::new();

    loop {
        step(&mut occupied);
        time += 1;
        // dbg!(time);
        assert!(!previous_positions.is_empty());
        'outer: for (x, y) in previous_positions.drain() {
            // dbg!((x, y));
            for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (x, y) = (x + dx, y + dy);

                if (x, y) == target {
                    if count == 0 {
                        // dbg!("first", time);
                        next_positions.clear();
                        next_positions.insert(target);
                        target = start;
                        count += 1;
                        break 'outer;
                    } else if count == 1 {
                        // dbg!("second", time, target);
                        next_positions.clear();
                        next_positions.insert(target);
                        target = end;
                        count += 1;
                        break 'outer;
                    } else if count == 2 {
                        return time;
                    }
                }

                if (x > 0
                    && y > 0
                    && x < width - 1
                    && y < height - 1
                    && !occupied.contains(&(x as usize, y as usize)))
                    || (x, y) == start
                    || (x, y) == end
                {
                    next_positions.insert((x, y));
                }
            }
        }
        assert!(previous_positions.is_empty());
        swap(&mut previous_positions, &mut next_positions);
    }
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(18, solve_first(sample));
    assert_eq!(54, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(286, solve_first(input));
    assert_eq!(820, solve_second(input));
}

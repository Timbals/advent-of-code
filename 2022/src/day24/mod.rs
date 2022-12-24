use ahash::AHashSet;
use itertools::Itertools;
use std::mem::swap;

pub fn solve(input: &str, snacks: bool) -> usize {
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
    let mut step = |time: &mut usize, occupied: &mut AHashSet<(usize, usize)>| {
        *time += 1;
        for (x, y, dx, dy) in &mut blizzards {
            *x = ((*x as isize + *dx - 1).rem_euclid(width - 2) + 1) as usize;
            *y = ((*y as isize + *dy - 1).rem_euclid(height - 2) + 1) as usize;
        }
        occupied.clear();
        occupied.extend(blizzards.iter().map(|(x, y, _, _)| (*x, *y)));
    };

    let start = (1, 0);
    let end = (width - 2, height - 1);
    let mut goals = if snacks {
        vec![end, start, end]
    } else {
        vec![end]
    };

    let mut previous_positions = AHashSet::from([start]);
    let mut next_positions = AHashSet::new();

    loop {
        step(&mut time, &mut occupied);

        'outer: for (x, y) in previous_positions.drain() {
            for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (x, y) = (x + dx, y + dy);

                if (x, y) == goals[0] {
                    goals.remove(0);

                    if goals.is_empty() {
                        return time;
                    }

                    next_positions.clear();
                    next_positions.insert((x, y));
                    break 'outer;
                }

                if (x, y) == start
                    || (x, y) == end
                    || (x > 0
                        && x < width - 1
                        && y > 0
                        && y < height - 1
                        && !occupied.contains(&(x as usize, y as usize)))
                {
                    next_positions.insert((x, y));
                }
            }
        }

        swap(&mut previous_positions, &mut next_positions);
    }
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(18, solve(sample, false));
    assert_eq!(54, solve(sample, true));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(286, solve(input, false));
    assert_eq!(820, solve(input, true));
}

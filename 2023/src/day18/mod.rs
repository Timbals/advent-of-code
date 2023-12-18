use itertools::Itertools;

pub fn solve_first(input: &str) -> usize {
    let mut b = 0;

    let a = input
        .lines()
        .map(|line| {
            let (dir, tail) = line.split_once(' ').unwrap();
            let (distance, _) = tail.split_once(' ').unwrap();
            let dir = dir.chars().next().unwrap();
            let distance = distance.parse::<isize>().unwrap();
            (dir, distance)
        })
        .scan((0, 0), |current, (dir, distance)| {
            let (dx, dy) = match dir {
                'R' => (distance, 0),
                'D' => (0, distance),
                'L' => (-distance, 0),
                'U' => (0, -distance),
                _ => unreachable!(),
            };

            b += distance;

            *current = (current.0 + dx, current.1 + dy);

            Some(*current)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .circular_tuple_windows()
        .map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2)) // shoelace formula
        .sum::<isize>() as usize
        / 2;
    let b = b as usize;

    a + 1 - b / 2 + b // pick's theorem
}

pub fn solve_second(input: &str) -> usize {
    let mut b = 0;

    let a = input
        .lines()
        .map(|line| {
            let (_, tail) = line.split_once(' ').unwrap();
            let (_, color) = tail.split_once(' ').unwrap();

            let mut distance = 0;
            for (i, digit) in color[2..7].chars().rev().enumerate() {
                distance += 16_u32.pow(i as _) * digit.to_digit(16).unwrap();
            }
            let distance = distance as isize;

            let dir = match color.chars().nth(7).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!(),
            };

            (dir, distance)
        })
        .scan((0, 0), |current, (dir, distance)| {
            let (dx, dy) = match dir {
                'R' => (distance, 0),
                'D' => (0, distance),
                'L' => (-distance, 0),
                'U' => (0, -distance),
                _ => unreachable!(),
            };

            b += distance;

            *current = (current.0 + dx, current.1 + dy);

            Some(*current)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .circular_tuple_windows()
        .map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2)) // shoelace formula
        .sum::<isize>() as usize
        / 2;
    let b = b as usize;

    a + 1 - b / 2 + b // pick's theorem
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(62, solve_first(sample));
    assert_eq!(952408144115, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(49578, solve_first(input));
    assert_eq!(52885384955882, solve_second(input));
}

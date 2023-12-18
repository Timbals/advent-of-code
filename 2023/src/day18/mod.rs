use itertools::Itertools;

pub fn solve(input: &str, use_color: bool) -> usize {
    let mut edge_points = 0;

    let area = input
        .lines()
        .map(|line| {
            let (direction, tail) = line.split_once(' ').unwrap();
            let (distance, color) = tail.split_once(' ').unwrap();

            if use_color {
                (
                    match color.as_bytes()[7] {
                        b'0' => b'R',
                        b'1' => b'D',
                        b'2' => b'L',
                        b'3' => b'U',
                        _ => unreachable!(),
                    },
                    isize::from_str_radix(&color[2..7], 16).unwrap(),
                )
            } else {
                (direction.as_bytes()[0], distance.parse::<isize>().unwrap())
            }
        })
        .inspect(|(_, distance)| edge_points += distance)
        .scan((0, 0), |current, (direction, distance)| {
            let (dx, dy) = match direction {
                b'R' => (distance, 0),
                b'D' => (0, distance),
                b'L' => (-distance, 0),
                b'U' => (0, -distance),
                _ => unreachable!(),
            };

            *current = (current.0 + dx, current.1 + dy);
            Some(*current)
        })
        .collect::<Vec<_>>() // unfortunate, needed because `circular_tuple_windows` has an `ExactSizeIterator` constraint
        .into_iter()
        .circular_tuple_windows()
        .map(|((x1, y1), (x2, y2))| (y1 + y2) * (x1 - x2)) // shoelace formula
        .sum::<isize>()
        / 2;

    (area + 1 - edge_points / 2 + edge_points) as usize // pick's theorem
}

pub fn solve_first(input: &str) -> usize {
    solve(input, false)
}

pub fn solve_second(input: &str) -> usize {
    solve(input, true)
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

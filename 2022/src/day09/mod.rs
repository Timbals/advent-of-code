use std::array::from_fn;
use std::collections::HashSet;
use std::iter::repeat;

pub fn parse(input: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    input.lines().flat_map(|line| {
        let movement = match line.as_bytes()[0] {
            b'U' => (0, 1),
            b'D' => (0, -1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            _ => unreachable!(),
        };
        let count = line[2..].parse().unwrap();
        repeat(movement).take(count)
    })
}

pub fn solve<const K: usize>(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut knots: [(i32, i32); K] = from_fn(|_| (0, 0));

    for (dx, dy) in parse(input) {
        knots[0].0 += dx;
        knots[0].1 += dy;

        for i in 0..K - 1 {
            let (hx, hy) = knots[i];
            let (tx, ty) = &mut knots[i + 1];

            if (hx - *tx).abs() > 1 || (hy - *ty).abs() > 1 {
                *tx += (hx - *tx).clamp(-1, 1);
                *ty += (hy - *ty).clamp(-1, 1);
            }
        }

        visited.insert(knots[K - 1]);
    }

    visited.len()
}

#[test]
pub fn sample() {
    let sample1 = include_str!("sample1.txt");
    assert_eq!(13, solve::<2>(sample1));
    assert_eq!(1, solve::<10>(sample1));

    let sample2 = include_str!("sample2.txt");
    assert_eq!(36, solve::<10>(sample2));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(6406, solve::<2>(input));
    assert_eq!(2643, solve::<10>(input));
}

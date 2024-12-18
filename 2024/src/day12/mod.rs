use itertools::Itertools;
use std::collections::BTreeSet;

pub fn solve_first(input: &str) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut remaining = (0..width).cartesian_product(0..height).collect::<BTreeSet<_>>();
    let mut result = 0;
    while let Some(start @ (x, y)) = remaining.pop_first() {
        let mut perimeter = 0;
        let mut area = 0;

        let value = grid[y][x];
        let mut stack = vec![start];
        remaining.remove(&start);

        while let Some((x, y)) = stack.pop() {
            area += 1;

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x2, y2) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                if (0..width).contains(&x2) && (0..height).contains(&y2) && grid[y2][x2] == value {
                    if remaining.contains(&(x2, y2)) {
                        remaining.remove(&(x2, y2));
                        stack.push((x2, y2));
                    }
                } else {
                    perimeter += 1;
                }
            }
        }

        result += area * perimeter;
    }

    result
}

pub fn solve_second(input: &str) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut remaining = (0..width).cartesian_product(0..height).collect::<BTreeSet<_>>();
    let mut result = 0;
    while let Some(start @ (x, y)) = remaining.pop_first() {
        let mut area = 0;

        let value = grid[y][x];
        let mut stack = vec![start];
        remaining.remove(&start);

        // (vertical, main_coordinate, cross_coordinate, sign)
        let mut sides = BTreeSet::new();

        while let Some((x, y)) = stack.pop() {
            area += 1;

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x2, y2) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                if (0..width).contains(&x2) && (0..height).contains(&y2) && grid[y2][x2] == value {
                    if remaining.contains(&(x2, y2)) {
                        remaining.remove(&(x2, y2));
                        stack.push((x2, y2));
                    }
                } else {
                    match (dx, dy) {
                        (-1, 0) => sides.insert((true, x, y, false)),
                        (1, 0) => sides.insert((true, x + 1, y, true)),
                        (0, -1) => sides.insert((false, y, x, false)),
                        (0, 1) => sides.insert((false, y + 1, x, true)),
                        _ => unreachable!(),
                    };
                }
            }
        }

        let sides = sides
            .into_iter()
            .coalesce(|a, b| {
                if a.0 == b.0 && a.1 == b.1 && a.2 + 1 == b.2 && a.3 == b.3 {
                    Ok(b)
                } else {
                    Err((a, b))
                }
            })
            .count();

        result += area * sides;
    }

    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(1930, solve_first(sample));
    assert_eq!(1206, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1451030, solve_first(input));
    assert_eq!(859494, solve_second(input));
}

use std::collections::HashSet;

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<u32>>) {
    let mut zeroes = Vec::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let value = c.to_digit(10).unwrap();
                    if value == 0 {
                        zeroes.push((x, y));
                    }
                    value
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (zeroes, grid)
}

fn walk(grid: &[Vec<u32>], mut paths: Vec<(usize, usize)>, mut f: impl FnMut((usize, usize))) {
    while let Some(coord @ (x, y)) = paths.pop() {
        let value = grid[y][x];

        if value == 9 {
            f(coord);
            continue;
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x2 = x.wrapping_add_signed(dx);
            let y2 = y.wrapping_add_signed(dy);
            if let Some(&neighbor) = grid.get(y2).and_then(|row| row.get(x2)) {
                if value + 1 == neighbor {
                    paths.push((x2, y2));
                }
            }
        }
    }
}

pub fn solve_first(input: &str) -> usize {
    let (zeroes, grid) = parse(input);

    let mut result = 0;
    for zero in zeroes {
        let mut endings = HashSet::new();
        walk(&grid, vec![zero], |coord| {
            endings.insert(coord);
        });
        result += endings.len();
    }

    result
}

pub fn solve_second(input: &str) -> usize {
    let (zeroes, grid) = parse(input);
    let mut result = 0;
    walk(&grid, zeroes, |_| result += 1);
    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(36, solve_first(sample));
    assert_eq!(81, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(652, solve_first(input));
    assert_eq!(1432, solve_second(input));
}

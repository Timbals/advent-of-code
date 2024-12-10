use std::cmp::min;

pub fn solve_first(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let size = width * height;
    let mut adjacency =
        (0..size).map(|_| (0..size).map(|_| usize::MAX).collect::<Vec<_>>()).collect::<Vec<_>>();

    let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut zeroes = Vec::new();
    let mut nines = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let value = grid[y][x];
            if value == 0 {
                zeroes.push((x, y));
            } else if value == 9 {
                nines.push((x, y));
            }
            for (dx, dy) in neighbors {
                let x2 = x.wrapping_add_signed(dx);
                let y2 = y.wrapping_add_signed(dy);
                if let Some(&neighbor) = grid.get(y2).and_then(|row| row.get(x2)) {
                    if value + 1 == neighbor {
                        adjacency[y * width + x][y2 * width + x2] = 1;
                    }
                }
            }
        }
    }

    for k in 0..size {
        for i in 0..size {
            for j in 0..size {
                adjacency[i][j] =
                    min(adjacency[i][j], usize::saturating_add(adjacency[i][k], adjacency[k][j]));
            }
        }
    }

    zeroes
        .into_iter()
        .map(|(x, y)| {
            nines
                .iter()
                .filter(|&(x2, y2)| adjacency[y * width + x][y2 * width + x2] < usize::MAX)
                .count()
        })
        .sum()
}

pub fn solve_second(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut paths = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let value = grid[y][x];
            if value == 0 {
                paths.push((x, y));
            }
        }
    }

    let mut result = 0;
    while let Some((x, y)) = paths.pop() {
        let value = grid[y][x];

        if value == 9 {
            result += 1;
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

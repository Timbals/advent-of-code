use std::collections::HashSet;

pub fn solve_first(input: &str, steps: usize) -> usize {
    let mut start = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => true,
                    '#' => false,
                    'S' => {
                        start = (x, y);
                        true
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut current = HashSet::new();
    current.insert(start);
    let mut next = HashSet::new();

    for _ in 0..steps {
        for (x, y) in current.drain() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                if x < width && y < height && grid[y][x] {
                    next.insert((x, y));
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
    }

    current.len()
}

pub fn solve_second(input: &str) -> usize {
    let mut start = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => true,
                    '#' => false,
                    'S' => {
                        start = (x as isize, y as isize);
                        true
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;
    assert_eq!(width, height);
    let size = width as usize;
    let to_edge = (size - 1) / 2;

    let mut current = HashSet::new();
    current.insert(start);
    let mut next = HashSet::new();

    let mut values = [0; 3];

    for i in 0..=(to_edge + size * 2) {
        if i >= to_edge && (i - to_edge) % size == 0 {
            values[(i - to_edge) / size] = current.len() as isize;
        }

        for (x, y) in current.drain() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (x, y) = (x + dx, y + dy);
                if grid[(y.rem_euclid(height)) as usize][(x.rem_euclid(width)) as usize] {
                    next.insert((x, y));
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
    }

    let target = 26501365;

    // Derivation from Lagrange polynomial:
    // x = [0, 1, 2]
    // l0(x) = ((x - 1) / (0 - 1)) * ((x - 2) / (0 - 2)) = x^2 / 2 - x - x/2 + 1
    // l1(x) = ((x - 0) / (1 - 0)) * ((x - 2) / (1 - 2)) = -x^2 + 2x
    // l2(x) = ((x - 0) / (2 - 0)) * ((x - 1) / (2 - 1)) = x/2 * (x - 1) = 1/2 * (x^2 - x)
    // P(x)  = y0 * (x^2 / 2 - x - x/2 + 1) + y1 * (-x^2 + 2x) + y2 * (1/2 * (x^2 - x))

    let x = ((target - to_edge) / size) as isize;

    (values[0] * (x.pow(2) / 2 - x - x / 2 + 1)
        + values[1] * (-x.pow(2) + 2 * x)
        + (values[2] * (x.pow(2) - x)) / 2) as usize
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(16, solve_first(sample, 6));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(3600, solve_first(input, 64));

    // println!("{}", solve_second(input, 65 + 131 * 0));
    // println!("{}", solve_second(input, 65 + 131 * 1));
    // println!("{}", solve_second(input, 65 + 131 * 2));
    let x = 202300;
    let x: u64 = (26501365 - 65) / 131;
    println!("{}", 3720 + 14775 * x + 14655 * x * x);
    assert_eq!(599763113936220, solve_second(input));
}

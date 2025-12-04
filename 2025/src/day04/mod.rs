use itertools::Itertools;

pub fn solve_first(input: &str) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();

    let mut total = 0;

    for (y, x) in Itertools::cartesian_product(0..grid.len(), 0..grid[0].len()) {
        if grid[y][x] != b'@' {
            continue;
        }

        let mut neighbors = 0;
        for (dy, dx) in Itertools::cartesian_product(-1..=1, -1..=1) {
            if let Some(row) = grid.get(y.wrapping_add_signed(dy))
                && row.get(x.wrapping_add_signed(dx)) == Some(&b'@')
            {
                neighbors += 1;
            }
        }
        if neighbors <= 4 {
            total += 1;
        }
    }

    total
}

pub fn solve_second(input: &str) -> usize {
    let mut grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<_>>();

    let mut total = 0;

    let mut done = false;
    while !done {
        done = true;
        for (y, x) in Itertools::cartesian_product(0..grid.len(), 0..grid[0].len()) {
            if grid[y][x] != b'@' {
                continue;
            }

            let mut neighbors = 0;
            for (dy, dx) in Itertools::cartesian_product(-1..=1, -1..=1) {
                if let Some(row) = grid.get(y.wrapping_add_signed(dy))
                    && row.get(x.wrapping_add_signed(dx)) == Some(&b'@')
                {
                    neighbors += 1;
                }
            }
            if neighbors <= 4 {
                total += 1;
                grid[y][x] = b'.';
                done = false;
            }
        }
    }

    total
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(13, solve_first(sample));
    assert_eq!(43, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1527, solve_first(input));
    assert_eq!(8690, solve_second(input));
}

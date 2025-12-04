pub fn solve_first(input: &str) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();

    let mut total = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != b'@' {
                continue;
            }

            let mut neighbors = 0;
            for dy in -1..=1 {
                let Some(y) = y.checked_add_signed(dy) else {
                    continue;
                };
                if y >= grid.len() {
                    continue;
                }

                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }

                    let Some(x) = x.checked_add_signed(dx) else {
                        continue;
                    };
                    if x >= grid[0].len() {
                        continue;
                    }

                    if grid[y][x] == b'@' {
                        neighbors += 1;
                    }
                }
            }
            if neighbors < 4 {
                total += 1;
            }
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
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] != b'@' {
                    continue;
                }

                let mut neighbors = 0;
                for dy in -1..=1 {
                    let Some(y) = y.checked_add_signed(dy) else {
                        continue;
                    };
                    if y >= grid.len() {
                        continue;
                    }

                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }

                        let Some(x) = x.checked_add_signed(dx) else {
                            continue;
                        };
                        if x >= grid[0].len() {
                            continue;
                        }

                        if grid[y][x] == b'@' {
                            neighbors += 1;
                        }
                    }
                }
                if neighbors < 4 {
                    total += 1;
                    grid[y][x] = b'.';
                    done = false;
                }
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

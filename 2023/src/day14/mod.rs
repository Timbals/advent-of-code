use std::collections::HashMap;

pub fn solve_first(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut load = 0;

    for column in 0..grid[0].len() {
        let mut next_load = grid.len();
        for row in 0..grid.len() {
            match grid[row][column] {
                'O' => {
                    load += next_load;
                    next_load = next_load.saturating_sub(1);
                }
                '#' => {
                    next_load = grid.len() - row - 1;
                }
                _ => {}
            }
        }
    }

    load
}

pub fn solve_second(input: &str) -> usize {
    let mut grid = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut seen = HashMap::new();

    let rounds = 1000000000;

    let mut index = 0;
    while index < rounds {
        index += 1;

        for _ in 0..4 {
            for x in 0..width {
                let mut next_y = 0;
                for y in 0..height {
                    match grid[x + y * width] {
                        'O' => {
                            grid[x + y * width] = '.';
                            grid[x + next_y * width] = 'O';
                            next_y += 1;
                        }
                        '#' => {
                            next_y = y + 1;
                        }
                        _ => {}
                    }
                }
            }

            // rotate clockwise by flipping twice (first along a vertical than along a diagonal)
            for y in 0..height {
                for x in 0..(width / 2) {
                    grid.swap(x + y * width, (width - x - 1) + y * width);
                }
            }
            for y in 0..height {
                for x in 0..(height - y - 1) {
                    grid.swap(x + y * width, (height - y - 1) + (width - x - 1) * width);
                }
            }
        }

        if let Some(old_index) = seen.insert(grid.clone(), index) {
            let cycle_length = old_index - index;
            let remaining = rounds - index;
            let cycle_count = remaining / cycle_length;
            index += cycle_count * cycle_length;
        }
    }

    let mut load = 0;
    for x in 0..width {
        for y in 0..height {
            if grid[x + y * width] == 'O' {
                load += height - y;
            }
        }
    }

    load
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(136, solve_first(sample));
    assert_eq!(64, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(112048, solve_first(input));
    assert_eq!(105606, solve_second(input));
}

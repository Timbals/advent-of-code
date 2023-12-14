use std::collections::HashMap;
use std::mem::swap;

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
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut next_grid = grid.clone();

    let mut seen = HashMap::new();

    let mut index = 0;
    while index < 1000000000 {
        swap(&mut grid, &mut next_grid);
        for column in 0..next_grid[0].len() {
            for row in &mut next_grid {
                if row[column] == 'O' {
                    row[column] = '.';
                }
            }
        }

        for column in 0..grid[0].len() {
            let mut next_load = 0;
            for (row_index, row) in grid.iter().enumerate() {
                match row[column] {
                    'O' => {
                        next_grid[next_load][column] = 'O';
                        next_load += 1;
                    }
                    '#' => {
                        next_load = row_index + 1;
                    }
                    _ => {}
                }
            }
        }

        swap(&mut grid, &mut next_grid);
        for column in 0..next_grid[0].len() {
            for row in &mut next_grid {
                if row[column] == 'O' {
                    row[column] = '.';
                }
            }
        }

        for row in 0..grid.len() {
            let mut next_load = 0;
            for column in 0..grid[0].len() {
                match grid[row][column] {
                    'O' => {
                        next_grid[row][next_load] = 'O';
                        next_load += 1;
                    }
                    '#' => {
                        next_load = column + 1;
                    }
                    _ => {}
                }
            }
        }

        swap(&mut grid, &mut next_grid);
        for column in 0..next_grid[0].len() {
            for row in &mut next_grid {
                if row[column] == 'O' {
                    row[column] = '.';
                }
            }
        }

        for column in 0..grid[0].len() {
            let mut next_load = grid.len() - 1;
            for row in (0..grid.len()).rev() {
                match grid[row][column] {
                    'O' => {
                        next_grid[next_load][column] = 'O';
                        next_load = next_load.saturating_sub(1);
                    }
                    '#' => {
                        next_load = row.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }

        swap(&mut grid, &mut next_grid);
        for column in 0..next_grid[0].len() {
            for row in &mut next_grid {
                if row[column] == 'O' {
                    row[column] = '.';
                }
            }
        }

        for row in 0..grid.len() {
            let mut next_load = grid[0].len() - 1;
            for column in (0..grid[0].len()).rev() {
                match grid[row][column] {
                    'O' => {
                        next_grid[row][next_load] = 'O';
                        next_load = next_load.saturating_sub(1);
                    }
                    '#' => {
                        next_load = column.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }

        index += 1;

        if let Some(old_index) = seen.insert(next_grid.clone(), index) {
            let cycle_length = old_index - index;
            let remaining = 1000000000 - index;
            let cycle_count = remaining / cycle_length;
            index += cycle_count * cycle_length;
        }
    }

    swap(&mut grid, &mut next_grid);

    let mut load = 0;
    for column in 0..grid[0].len() {
        for row in 0..grid.len() {
            if grid[row][column] == 'O' {
                load += grid.len() - row;
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

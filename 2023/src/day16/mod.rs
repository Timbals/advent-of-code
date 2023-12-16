use itertools::chain;
use std::iter::once;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

fn solve(grid: &[Vec<char>], start: (usize, usize, usize)) -> usize {
    let mut energy = grid
        .iter()
        .map(|row| row.iter().map(|_| [false; 4]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut stack = vec![start];

    while let Some((x, y, dir)) = stack.pop() {
        energy[y][x][dir] = true;

        let (new_dir1, new_dir2) = match grid[y][x] {
            '.' => (dir, None),
            '/' if dir == RIGHT => (UP, None),
            '/' if dir == UP => (RIGHT, None),
            '/' if dir == DOWN => (LEFT, None),
            '/' if dir == LEFT => (DOWN, None),
            '\\' if dir == RIGHT => (DOWN, None),
            '\\' if dir == DOWN => (RIGHT, None),
            '\\' if dir == LEFT => (UP, None),
            '\\' if dir == UP => (LEFT, None),
            '|' if dir == DOWN || dir == UP => (dir, None),
            '|' if dir == RIGHT || dir == LEFT => (DOWN, Some(UP)),
            '-' if dir == RIGHT || dir == LEFT => (dir, None),
            '-' if dir == DOWN || dir == UP => (RIGHT, Some(LEFT)),
            _ => unreachable!(),
        };

        for new_dir in once(new_dir1).chain(new_dir2) {
            let (new_x, new_y) = match new_dir {
                RIGHT if x + 1 < grid[0].len() => (x + 1, y),
                DOWN if y + 1 < grid.len() => (x, y + 1),
                LEFT if x > 0 => (x - 1, y),
                UP if y > 0 => (x, y - 1),
                _ => continue, // out of bounds
            };

            // check if already energized in the direction
            if !energy[new_y][new_x][new_dir] {
                stack.push((new_x, new_y, new_dir))
            }
        }
    }

    energy
        .iter()
        .flat_map(|row| row.iter().filter(|x| x.iter().any(|&y| y)))
        .count()
}

pub fn solve_first(input: &str) -> usize {
    let grid = parse(input);
    solve(&grid, (0, 0, 0))
}

pub fn solve_second(input: &str) -> usize {
    let grid = parse(input);

    chain!(
        (0..grid.len()).map(|y| (0, y, 0)),                 // left side
        (0..grid[0].len()).map(|x| (x, 0, 1)),              // top side
        (0..grid.len()).map(|y| (grid[0].len() - 1, y, 2)), // right side
        (0..grid[0].len()).map(|x| (x, grid.len() - 1, 3)), // bottom side
    )
    .map(|start| solve(&grid, start))
    .max()
    .unwrap()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(46, solve_first(sample));
    assert_eq!(51, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(8116, solve_first(input));
    assert_eq!(8383, solve_second(input));
}

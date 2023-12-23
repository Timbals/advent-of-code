pub fn solve(input: &str, slopes: bool) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut stack = Vec::new();
    stack.push((0, (1, 0)));

    let mut on_path = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;

    while let Some((steps, (x, y))) = stack.last().copied() {
        if on_path[x][y] {
            stack.pop();
            on_path[x][y] = false;

            if x == grid[0].len() - 2 && y == grid.len() - 1 {
                result = result.max(steps);
            }

            continue;
        }

        on_path[x][y] = true;

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            if nx >= grid[0].len() || ny >= grid.len() {
                continue;
            }
            let tile = grid[ny][nx];
            if tile == '#' {
                continue;
            }
            if slopes
                && ((tile == '>' && dx == -1)
                    || (tile == '<' && dx == 1)
                    || (tile == 'v' && dy == -1)
                    || (tile == '^' && dy == 1))
            {
                continue;
            }

            if !on_path[nx][ny] {
                stack.push((steps + 1, (nx, ny)));
            }
        }
    }

    result
}

pub fn solve_first(input: &str) -> usize {
    solve(input, true)
}

pub fn solve_second(input: &str) -> usize {
    solve(input, false)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(94, solve_first(sample));
    assert_eq!(154, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2042, solve_first(input));
    assert_eq!(6466, solve_second(input));
}

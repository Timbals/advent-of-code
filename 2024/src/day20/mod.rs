use itertools::Itertools;

pub fn solve(input: &str, save: usize, cheat: isize) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let width = grid[0].len();
    let height = grid.len();

    let (sy, sx) =
        (0..height).cartesian_product(0..width).find(|&(y, x)| grid[y][x] == b'S').unwrap();
    let mut path = vec![(sx, sy)];
    while grid[path.last().unwrap().1][path.last().unwrap().0] != b'E' {
        let &(x, y) = path.last().unwrap();
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            if grid[ny][nx] != b'#' && (path.len() == 1 || (nx, ny) != path[path.len() - 2]) {
                path.push((nx, ny));
                break;
            }
        }
    }
    let mut shortcut = vec![vec![usize::MAX; width]; height];
    for (remaining_steps, &(x, y)) in path.iter().rev().enumerate() {
        shortcut[y][x] = remaining_steps;
    }

    let mut result = 0;
    for (start_steps, &(x, y)) in path.iter().enumerate() {
        for dy in (-cheat)..=cheat {
            let remaining = cheat - dy.abs();
            for dx in -remaining..=remaining {
                let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                if nx < width && ny < height && grid[ny][nx] != b'#' {
                    let steps = start_steps + x.abs_diff(nx) + y.abs_diff(ny) + shortcut[ny][nx];
                    if steps + save <= path.len() - 1 {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(44, solve(sample, 2, 2));
    assert_eq!(285, solve(sample, 50, 20));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1375, solve(input, 100, 2));
    assert_eq!(983054, solve(input, 100, 20));
}

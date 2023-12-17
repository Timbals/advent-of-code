use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Cache {
    x_stride: usize,
    y_stride: usize,
    dir_stride: usize,
    data: Vec<bool>,
}

impl Cache {
    fn new(width: usize, height: usize, max_move: usize) -> Self {
        let directions = 4;
        Self {
            x_stride: height * directions * (max_move + 1),
            y_stride: directions * (max_move + 1),
            dir_stride: max_move + 1,
            data: vec![false; width * height * directions * (max_move + 1)],
        }
    }

    fn insert(&mut self, x: usize, y: usize, dir: usize, consecutive: usize) -> bool {
        std::mem::replace(
            &mut self.data
                [x * self.x_stride + y * self.y_stride + dir * self.dir_stride + consecutive],
            true,
        )
    }
}

pub fn solve(input: &str, min_move: usize, max_move: usize) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut heap = BinaryHeap::<Reverse<(usize, usize, usize, usize, usize)>>::new();
    // (heat, x, y, last_dir, consecutive)
    heap.push(Reverse((0, 0, 0, 0, 0)));

    let mut seen = Cache::new(width, height, max_move);

    while let Some(Reverse((heat, x, y, last_dir, consecutive))) = heap.pop() {
        if (x == width - 1) && (y == height - 1) && consecutive >= min_move {
            return heat;
        }

        heap.extend(
            [
                (x + 1, y),
                (x, y + 1),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_sub(1)),
            ]
            .into_iter()
            .enumerate()
            .filter(|(_, (x, y))| *x < width && *y < height) // bounds check
            .filter(|(dir, _)| *dir != (last_dir + 2) % 4) // can't turn back
            .filter(|(dir, _)| *dir == last_dir || consecutive >= min_move) // move at least `min_move` blocks in one direction
            .map(|(dir, coords)| {
                if dir == last_dir {
                    (dir, coords, consecutive + 1)
                } else {
                    (dir, coords, 1)
                }
            })
            .filter(|(_, _, consecutive)| *consecutive <= max_move) // at most `max_move` blocks in one direction
            .filter(|(dir, (x, y), consecutive)| !seen.insert(*x, *y, *dir, *consecutive))
            .map(|(dir, (x, y), consecutive)| Reverse((heat + grid[y][x], x, y, dir, consecutive))),
        )
    }

    unreachable!("no valid path")
}

pub fn solve_first(input: &str) -> usize {
    solve(input, 0, 3)
}

pub fn solve_second(input: &str) -> usize {
    solve(input, 4, 10)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(102, solve_first(sample));
    assert_eq!(94, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(959, solve_first(input));
    assert_eq!(1135, solve_second(input));
}

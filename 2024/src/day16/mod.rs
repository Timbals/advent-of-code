use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

pub fn solve(input: &str, count_tiles: bool) -> usize {
    let grid = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let width = grid[0].len();
    let height = grid.len();
    let (y, x) = Itertools::cartesian_product(0..height, 0..width)
        .find(|&(y, x)| grid[y][x] == b'S')
        .unwrap();

    let (ty, tx) = Itertools::cartesian_product(0..height, 0..width)
        .find(|&(y, x)| grid[y][x] == b'E')
        .unwrap();

    let facings = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut visited = BTreeMap::new();
    let mut previous = (0..height)
        .map(|_| {
            (0..width)
                .map(|_| (0..facings.len()).map(|_| Vec::new()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), x, y, 0_isize));
    while let Some((score, x, y, facing)) = heap.pop() {
        if (x, y) == (tx, ty) {
            return if count_tiles {
                let mut stack = vec![(tx, ty, 0), (tx, ty, 1), (tx, ty, 2), (tx, ty, 3)];
                let mut final_tiles = BTreeSet::new();
                while let Some((x, y, facing)) = stack.pop() {
                    if !final_tiles.insert((x, y, facing)) {
                        continue;
                    }
                    stack.extend(&previous[y][x][facing]);
                }
                final_tiles.into_iter().map(|(x, y, _)| (x, y)).dedup().count()
            } else {
                score.0
            };
        }

        let mut push_node = |score: usize, nx: usize, ny: usize, n_facing| {
            let mut push = true;
            if let Some(&best_score) = visited.get(&(nx, ny, n_facing)) {
                match score.cmp(&best_score) {
                    std::cmp::Ordering::Greater => push = false,
                    std::cmp::Ordering::Less => previous[ny][nx][n_facing as usize].clear(),
                    std::cmp::Ordering::Equal => {}
                }
            }

            if push {
                heap.push((Reverse(score), nx, ny, n_facing));
                previous[ny][nx][n_facing as usize].push((x, y, facing as usize));
                visited.insert((nx, ny, n_facing), score);
            }
        };

        let (dx, dy) = facings[facing as usize];
        let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        if grid[ny][nx] != b'#' {
            push_node(score.0 + 1, nx, ny, facing);
        }

        push_node(score.0 + 1000, x, y, (facing + 1).rem_euclid(facings.len() as isize));
        push_node(score.0 + 1000, x, y, (facing - 1).rem_euclid(facings.len() as isize));
    }

    unreachable!("no valid path")
}

#[test]
pub fn sample() {
    assert_eq!(7036, solve(include_str!("sample.txt"), false));
    assert_eq!(11048, solve(include_str!("sample2.txt"), false));
    assert_eq!(45, solve(include_str!("sample.txt"), true));
    assert_eq!(64, solve(include_str!("sample2.txt"), true));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(98520, solve(input, false));
    assert_eq!(609, solve(input, true));
}

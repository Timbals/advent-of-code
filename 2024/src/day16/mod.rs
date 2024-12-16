use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

pub fn solve_first(input: &str) -> usize {
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
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), (x, y), 0_isize));

    let mut visited = BTreeSet::new();

    while let Some((score, (x, y), facing)) = heap.pop() {
        if !visited.insert((x, y, facing)) {
            continue;
        }

        if (x, y) == (tx, ty) {
            return score.0;
        }

        let (dx, dy) = facings[facing as usize];
        let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        if grid[ny][nx] != b'#' {
            heap.push((Reverse(score.0 + 1), (nx, ny), facing));
        }
        heap.push((Reverse(score.0 + 1000), (x, y), (facing + 1).rem_euclid(4)));
        heap.push((Reverse(score.0 + 1000), (x, y), (facing - 1).rem_euclid(4)));
    }

    unreachable!()
}

pub fn solve_second(input: &str) -> usize {
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
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), (x, y), 0_isize, BTreeSet::new()));

    let mut visited = BTreeMap::new();

    while let Some((score, (x, y), facing, mut tiles)) = heap.pop() {
        if let Some(&prev) = visited.get(&(x, y, facing)) {
            if prev < score.0 {
                continue;
            }
        }
        visited.insert((x, y, facing), score.0);

        if (x, y) == (tx, ty) {
            let mut final_tiles = tiles;
            while let Some((s, _, _, tiles)) = heap.pop() {
                if s == score {
                    final_tiles.extend(tiles);
                } else {
                    break;
                }
            }
            final_tiles.insert((tx, ty));

            return final_tiles.len();
        }

        tiles.insert((x, y));

        let (dx, dy) = facings[facing as usize];
        let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        if grid[ny][nx] != b'#' {
            heap.push((Reverse(score.0 + 1), (nx, ny), facing, tiles.clone()));
        }
        heap.push((Reverse(score.0 + 1000), (x, y), (facing + 1).rem_euclid(4), tiles.clone()));
        heap.push((Reverse(score.0 + 1000), (x, y), (facing - 1).rem_euclid(4), tiles));
    }

    unreachable!()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(7036, solve_first(sample));
    assert_eq!(11048, solve_first(include_str!("sample2.txt")));
    assert_eq!(45, solve_second(sample));
    assert_eq!(64, solve_second(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(98520, solve_first(input));
    assert_eq!(609, solve_second(input));
}

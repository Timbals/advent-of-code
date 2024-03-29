#![allow(clippy::needless_range_loop)]

use itertools::Itertools;
use std::collections::HashSet;

fn create_grid(input: &str) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);

    let bricks = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start: [_; 3] = start
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let end: [_; 3] = end
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            max_x = max_x.max(start[0]).max(end[0]);
            max_y = max_y.max(start[1]).max(end[1]);
            max_z = max_z.max(start[2]).max(end[2]);

            (start, end)
        })
        .collect::<Vec<_>>();

    let (width, depth, height) = (max_x + 1, max_y + 1, max_z + 1);
    let mut grid = vec![vec![vec![None::<usize>; height]; depth]; width];

    let mut supported_by = vec![HashSet::<usize>::new(); bricks.len()];
    let mut supports = vec![HashSet::<usize>::new(); bricks.len()];

    // fill grid with cubes in ascending z order
    for (i, (start, end)) in bricks
        .iter()
        .enumerate()
        .sorted_unstable_by_key(|(_, ([_, _, _], [_, _, end_z]))| end_z)
    {
        // search the height where the brick comes to rest
        let mut target_z = 1;
        for z in (1..start[2]).rev() {
            let mut found = false;
            for x in start[0]..=end[0] {
                for y in start[1]..=end[1] {
                    if let Some(support) = grid[x][y][z] {
                        supported_by[i].insert(support);
                        supports[support].insert(i);
                        target_z = z + 1;
                        found = true;
                    }
                }
            }

            if found {
                break;
            }
        }

        // place the brick at the height
        for x in start[0]..=end[0] {
            for y in start[1]..=end[1] {
                for z in target_z..=(target_z + start[2].abs_diff(end[2])) {
                    grid[x][y][z] = Some(i);
                }
            }
        }
    }

    (supported_by, supports)
}

pub fn solve_first(input: &str) -> usize {
    let (supported_by, supports) = create_grid(input);

    supports
        .iter()
        .filter(|supports| supports.iter().all(|&brick| supported_by[brick].len() > 1))
        .count()
}

pub fn solve_second(input: &str) -> usize {
    let (supported_by, supports) = create_grid(input);

    (0..supports.len())
        .map(|initial| {
            let mut falls = HashSet::new();

            let mut new_falls = HashSet::new();
            new_falls.insert(initial);

            while !new_falls.is_empty() {
                falls.extend(new_falls.drain());

                new_falls.extend(
                    (0..supported_by.len())
                        .filter(|&brick| !supported_by[brick].is_empty())
                        .filter(|brick| !falls.contains(brick))
                        .filter(|&brick| supported_by[brick].difference(&falls).count() == 0),
                );
            }

            falls.len() - 1
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(5, solve_first(sample));
    assert_eq!(7, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(401, solve_first(input));
    assert_eq!(63491, solve_second(input));
}

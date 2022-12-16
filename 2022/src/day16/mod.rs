use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn solve_first(input: &str) -> usize {
    // PARSING
    let valves = input
        .lines()
        .map(|line| {
            (
                &line[6..8],
                line.split(['=', ';'])
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                line.split([' ', ',']).skip(9).step_by(2).collect_vec(),
            )
        })
        .collect_vec();
    let mut indices = HashMap::new();
    for (valve, _, _) in &valves {
        indices.insert(*valve, indices.len());
    }
    let valves = valves
        .into_iter()
        .map(|(valve, rate, neighbors)| {
            (
                indices[valve],
                rate,
                neighbors.into_iter().map(|n| indices[n]).collect_vec(),
            )
        })
        .collect_vec();

    // SOLVING

    // all-pair-shortest-paths
    let mut distances = vec![vec![usize::MAX; valves.len()]; valves.len()];
    for (valve, _, neighbors) in &valves {
        distances[*valve][*valve] = 0;
        for neighbor in neighbors {
            distances[*valve][*neighbor] = 1;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                distances[i][j] = min(
                    distances[i][j],
                    usize::saturating_add(distances[i][k], distances[k][j]),
                );
            }
        }
    }

    // only need to look at valves with rate>0 and the start valve
    // TODO reduce valves and re-index

    // brute-force solution
    fn find_best(
        current: usize,
        mut visited: u64,
        remaining: usize,
        valves: &Vec<(usize, usize, Vec<usize>)>,
        distances: &Vec<Vec<usize>>,
    ) -> usize {
        if remaining == 0 {
            return 0;
        }

        visited |= 1 << current;
        (0..64)
            .map(|i| visited & (1 << i) != 0)
            .enumerate()
            .filter(|(_, x)| !*x)
            .map(|(target, _)| {
                find_best(
                    target,
                    visited,
                    remaining
                        .saturating_sub(distances[current][target])
                        .saturating_sub(1),
                    valves,
                    distances,
                )
            })
            .max()
            .unwrap_or_default()
            + remaining * valves[current].1
    }

    let mut visited = u64::MAX;
    for (valve, rate, _) in &valves {
        if *rate != 0 {
            visited ^= 1 << valve;
        }
    }

    find_best(indices["AA"], visited, 30, &valves, &distances)
}

pub fn solve_second(input: &str) -> usize {
    // PARSING
    let valves = input
        .lines()
        .map(|line| {
            (
                &line[6..8],
                line.split(['=', ';'])
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                line.split([' ', ',']).skip(9).step_by(2).collect_vec(),
            )
        })
        .collect_vec();
    let mut indices = HashMap::new();
    for (valve, _, _) in &valves {
        indices.insert(*valve, indices.len());
    }
    let valves = valves
        .into_iter()
        .map(|(valve, rate, neighbors)| {
            (
                indices[valve],
                rate,
                neighbors.into_iter().map(|n| indices[n]).collect_vec(),
            )
        })
        .collect_vec();

    // SOLVING

    // all-pair-shortest-paths
    let mut distances = vec![vec![usize::MAX; valves.len()]; valves.len()];
    for (valve, _, neighbors) in &valves {
        distances[*valve][*valve] = 0;
        for neighbor in neighbors {
            distances[*valve][*neighbor] = 1;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                distances[i][j] = min(
                    distances[i][j],
                    usize::saturating_add(distances[i][k], distances[k][j]),
                );
            }
        }
    }

    // only need to look at valves with rate>0 and the start valve
    // TODO reduce valves and re-index

    // brute-force solution
    fn find_best(
        start: usize,
        current: usize,
        elephant: bool,
        mut visited: u64,
        remaining: usize,
        valves: &Vec<(usize, usize, Vec<usize>)>,
        distances: &Vec<Vec<usize>>,
    ) -> usize {
        if remaining == 0 {
            return 0;
        }

        let score = remaining * valves[current].1;

        visited |= 1 << current;
        let score_if_keep_going = (0..64)
            .map(|i| visited & (1 << i) != 0)
            .enumerate()
            .filter(|(_, x)| !*x)
            .map(|(target, _)| {
                find_best(
                    start,
                    target,
                    elephant,
                    visited,
                    remaining
                        .saturating_sub(distances[current][target])
                        .saturating_sub(1),
                    valves,
                    distances,
                )
            })
            .max()
            .unwrap_or_default();

        let score_if_stop = if !elephant {
            find_best(start, start, true, visited, 26, valves, distances)
        } else {
            0
        };

        max(score_if_keep_going, score_if_stop) + score
    }

    let mut visited = u64::MAX;
    for (valve, rate, _) in &valves {
        if *rate != 0 {
            visited ^= 1 << valve;
        }
    }

    find_best(
        indices["AA"],
        indices["AA"],
        false,
        visited,
        26,
        &valves,
        &distances,
    )
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(1651, solve_first(sample));
    assert_eq!(1707, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1617, solve_first(input));
    assert_eq!(2171, solve_second(input));
}

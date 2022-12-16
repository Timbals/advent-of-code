use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    let mut indices = HashMap::new();
    let mut start = "";
    let mut valves = input
        .lines()
        .map(|line| {
            if indices.is_empty() {
                start = &line[6..8];
            }
            indices.insert(&line[6..8], indices.len());
            (
                line.split(['=', ';']).nth(1).unwrap().parse().unwrap(),
                line.split([' ', ',']).skip(9).step_by(2).collect_vec(),
            )
        })
        .collect_vec();

    valves.swap(0, indices["AA"]);
    indices.insert(start, indices["AA"]);
    indices.insert("AA", 0);

    valves
        .into_iter()
        .map(|(rate, neighbors)| (rate, neighbors.into_iter().map(|n| indices[n]).collect()))
        .collect()
}

pub fn all_pairs_shortest_paths(valves: &Vec<(usize, Vec<usize>)>) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![usize::MAX / 2; valves.len()]; valves.len()];
    for (valve, (_, neighbors)) in valves.iter().enumerate() {
        distances[valve][valve] = 0;
        for &neighbor in neighbors {
            distances[valve][neighbor] = 1;
        }
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                distances[i][j] = min(distances[i][j], distances[i][k] + distances[k][j]);
            }
        }
    }
    distances
}

pub fn solve(input: &str, time: usize, actors: usize) -> usize {
    let valves = parse(input);
    let distances = all_pairs_shortest_paths(&valves);

    fn search(
        current: usize,
        actors: usize,
        visited: usize,
        remaining: usize,
        valves: &Vec<(usize, Vec<usize>)>,
        distances: &Vec<Vec<usize>>,
        time: usize,
    ) -> usize {
        if remaining == 0 {
            return 0;
        }

        let visited = visited | (1 << current);

        let score_if_keep_going = (0..valves.len())
            .filter(|&i| visited & (1 << i) == 0)
            .map(|target| {
                search(
                    target,
                    actors,
                    visited,
                    remaining.saturating_sub(distances[current][target] + 1),
                    valves,
                    distances,
                    time,
                )
            })
            .max()
            .unwrap_or_default();

        let score_if_stop = if actors > 1 {
            search(0, actors - 1, visited, time, valves, distances, time)
        } else {
            0
        };

        max(score_if_keep_going, score_if_stop) + remaining * valves[current].0
    }

    let mut visited = usize::MAX;
    for (valve, (rate, _)) in valves.iter().enumerate() {
        // can ignore valves with rate==0
        if *rate != 0 {
            visited ^= 1 << valve;
        }
    }

    search(0, actors, visited, time, &valves, &distances, time)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(1651, solve(sample, 30, 1));
    assert_eq!(1707, solve(sample, 26, 2));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1617, solve(input, 30, 1));
    assert_eq!(2171, solve(input, 26, 2));
}

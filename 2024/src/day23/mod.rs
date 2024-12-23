use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

fn parse(input: &str) -> HashMap<&str, BTreeSet<&str>> {
    let mut neighbors = HashMap::<&str, BTreeSet<&str>>::new();
    for (from, to) in input.lines().map(|line| line.split_once('-').unwrap()) {
        neighbors.entry(from).or_default().insert(to);
        neighbors.entry(to).or_default().insert(from);
    }
    neighbors.shrink_to_fit();
    neighbors
}

pub fn solve_first(input: &str) -> usize {
    let neighbors = parse(input);

    let mut three_components = HashSet::new();
    for &node in neighbors.keys().filter(|node| node.starts_with('t')) {
        for &neighbor1 in &neighbors[node] {
            for neighbor2 in neighbors[node].intersection(&neighbors[neighbor1]) {
                three_components.insert(BTreeSet::from([node, neighbor1, neighbor2]));
            }
        }
    }

    three_components.len()
}

pub fn solve_second(input: &str) -> String {
    let neighbors = parse(input);

    let mut components =
        neighbors.keys().map(|&node| BTreeSet::from([node])).collect::<HashSet<_>>();
    while components.len() > 1 {
        for component in std::mem::take(&mut components) {
            for (&node, neighbors) in &neighbors {
                if component.is_subset(neighbors) {
                    let mut component = component.clone();
                    component.insert(node);
                    components.insert(component);
                }
            }
        }
    }

    components.into_iter().exactly_one().unwrap().into_iter().sorted_unstable().join(",")
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(7, solve_first(sample));
    assert_eq!("co,de,ka,ta", solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1046, solve_first(input));
    assert_eq!("de,id,ke,ls,po,sn,tf,tl,tm,uj,un,xw,yz", solve_second(input));
}

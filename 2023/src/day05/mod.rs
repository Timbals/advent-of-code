use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::ops::Range;
use std::str::Lines;

pub fn solve_first(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    lines.next(); // empty

    let next_map = |lines: &mut Lines<'_>, seeds: &mut Vec<u64>| {
        let mut next_seeds = Vec::with_capacity(seeds.len());

        for (source, destination) in lines.take_while(|line| !line.is_empty()).map(|line| {
            let (destination, tail) = line.split_once(' ').unwrap();
            let (source, len) = tail.split_once(' ').unwrap();
            let (destination, source, len) = (
                destination.parse::<u64>().unwrap(),
                source.parse::<u64>().unwrap(),
                len.parse::<u64>().unwrap(),
            );
            (source..(source + len), destination)
        }) {
            seeds.retain(|x| {
                if source.contains(x) {
                    next_seeds.push(*x - source.start + destination);
                    false
                } else {
                    true
                }
            });
        }

        next_seeds.extend(seeds.iter());
        *seeds = next_seeds;
    };

    while lines.next().is_some() {
        next_map(&mut lines, &mut seeds);
    }

    seeds.into_iter().min().unwrap()
}

pub fn solve_second_force(input: &str) -> u64 {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .tuples()
        .flat_map(|(start, len)| start..(start + len));

    let mut lines = input.lines();
    lines.next().unwrap(); // empty
    lines.next().unwrap(); // empty

    let mut mappings = Vec::new();
    while lines.next().is_some() {
        let x = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (destination, tail) = line.split_once(' ').unwrap();
                let (source, len) = tail.split_once(' ').unwrap();
                let (destination, source, len) = (
                    destination.parse::<u64>().unwrap(),
                    source.parse::<u64>().unwrap(),
                    len.parse::<u64>().unwrap(),
                );
                (source..(source + len), destination)
            })
            .collect_vec();
        mappings.push(x);
    }

    seeds
        .par_bridge()
        .map(|mut seed| {
            for mapping in &mappings {
                for (source, destination) in mapping {
                    if source.contains(&seed) {
                        seed = seed - source.start + *destination;
                        break;
                    }
                }
            }
            seed
        })
        .min()
        .unwrap()
}

pub fn solve_second(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let x = chunk.collect::<Vec<_>>();
            let (start, len) = (x[0], x[1]);
            start..(start + len)
        })
        .collect::<Vec<_>>();

    lines.next(); // empty

    let next_map = |lines: &mut Lines<'_>, seeds: &mut Vec<Range<u64>>| {
        let mut next_seeds = Vec::with_capacity(seeds.len());

        for (source, destination) in lines.take_while(|line| !line.is_empty()).map(|line| {
            let (destination, tail) = line.split_once(' ').unwrap();
            let (source, len) = tail.split_once(' ').unwrap();
            let (destination, source, len) = (
                destination.parse::<u64>().unwrap(),
                source.parse::<u64>().unwrap(),
                len.parse::<u64>().unwrap(),
            );
            (source..(source + len), destination)
        }) {
            seeds.retain_mut(|x| {
                assert!(!x.is_empty());

                if source.contains(&x.start) && source.contains(&(x.end - 1)) {
                    next_seeds.push(
                        (x.start - source.start + destination)
                            ..(x.end - source.start + destination),
                    );
                    println!(
                        "1 {x:?} {source:?} {destination} -> {:?}",
                        (x.start - source.start + destination)
                            ..(x.end - source.start + destination)
                    );
                    assert_eq!(
                        x.end - x.start,
                        (x.end - source.start + destination)
                            - (x.start - source.start + destination)
                    );
                    false
                } else if source.contains(&x.start) {
                    next_seeds.push(
                        (x.start - source.start + destination)
                            ..(source.end - source.start + destination),
                    );
                    println!(
                        "2 {x:?} {source:?} {destination} -> {:?} and {:?}",
                        (x.start - source.start + destination)
                            ..(source.end - source.start + destination),
                        source.end..x.end
                    );
                    assert_eq!(
                        x.end - x.start,
                        x.end - source.end + (source.end - source.start + destination)
                            - (x.start - source.start + destination)
                    );
                    x.start = source.end;
                    true
                } else if source.contains(&(x.end - 1)) {
                    next_seeds.push(
                        (source.start - source.start + destination)
                            ..(x.end - source.start + destination),
                    );
                    println!(
                        "3 {x:?} {source:?} {destination} -> {:?} and {:?}",
                        (source.start - source.start + destination)
                            ..(x.end - source.start + destination),
                        x.start..source.start
                    );
                    assert_eq!(
                        x.end - x.start,
                        source.start - x.start + (x.end - source.start + destination)
                            - (source.start - source.start + destination)
                    );
                    x.end = source.start;
                    true
                } else {
                    true
                }
            });
        }

        next_seeds.extend(seeds.iter().cloned());
        *seeds = next_seeds;
    };

    println!("{seeds:?}");
    while lines.next().is_some() {
        next_map(&mut lines, &mut seeds);
        println!("{seeds:?}");
    }

    seeds
        .into_iter()
        .filter(|range| !range.is_empty())
        .map(|range| range.start)
        .min()
        .unwrap()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(35, solve_first(sample));
    assert_eq!(46, solve_second_force(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(318728750, solve_first(input));
    assert_eq!(37384986, solve_second_force(input));
}

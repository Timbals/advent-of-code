use itertools::Itertools;
use std::ops::Range;

type ParseResult = (Vec<u64>, Vec<Vec<(Range<u64>, i64)>>);
fn parse(input: &str) -> ParseResult {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    lines.next(); // empty line

    let mut mappings = Vec::new();
    while lines.next().is_some() {
        mappings.push(
            lines
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
                    (source..(source + len), destination as i64 - source as i64)
                })
                .collect::<Vec<_>>(),
        );
    }

    (seeds, mappings)
}

pub fn solve_first(input: &str) -> u64 {
    let (mut seeds, mappings) = parse(input);

    for mapping in mappings {
        let mut next_seeds = Vec::with_capacity(seeds.len());

        for (source, offset) in mapping {
            seeds.retain(|seed| {
                if source.contains(seed) {
                    next_seeds.push(seed.checked_add_signed(offset).unwrap());
                    false
                } else {
                    true
                }
            });
        }

        next_seeds.extend(seeds.iter());
        seeds = next_seeds;
    }

    seeds.into_iter().min().unwrap()
}

pub fn solve_second(input: &str) -> u64 {
    let (seeds, mappings) = parse(input);
    let mut seeds = seeds
        .into_iter()
        .tuples()
        .map(|(start, len)| start..(start + len))
        .collect::<Vec<_>>();

    for mapping in mappings {
        let mut next_seeds = Vec::new();

        for (source, offset) in mapping {
            let mut retain = Vec::new();
            for seed in seeds {
                if source.contains(&seed.start) && source.contains(&(seed.end - 1)) {
                    next_seeds.push(
                        seed.start.checked_add_signed(offset).unwrap()
                            ..seed.end.checked_add_signed(offset).unwrap(),
                    );
                } else if source.contains(&seed.start) {
                    next_seeds.push(
                        seed.start.checked_add_signed(offset).unwrap()
                            ..source.end.checked_add_signed(offset).unwrap(),
                    );
                    retain.push(source.end..seed.end);
                } else if source.contains(&(seed.end - 1)) {
                    retain.push(seed.start..source.start);
                    next_seeds.push(
                        source.start.checked_add_signed(offset).unwrap()
                            ..seed.end.checked_add_signed(offset).unwrap(),
                    );
                } else if seed.contains(&source.start) {
                    retain.push(seed.start..source.start);
                    next_seeds.push(
                        source.start.checked_add_signed(offset).unwrap()
                            ..source.end.checked_add_signed(offset).unwrap(),
                    );
                    retain.push(source.end..seed.end);
                } else {
                    retain.push(seed);
                }
            }
            seeds = retain;
        }

        next_seeds.extend(seeds.iter().cloned());
        seeds = next_seeds;
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
    assert_eq!(46, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(318728750, solve_first(input));
    assert_eq!(37384986, solve_second(input));
}

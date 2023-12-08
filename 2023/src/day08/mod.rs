use num::Integer;
use std::collections::HashMap;
use std::io::BufRead;

pub fn solve_first(input: &str) -> usize {
    let mut lines = input.lines();
    let lr = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c == 'L')
        .collect::<Vec<_>>();

    lines.next().unwrap(); // empty

    let network = lines
        .map(|line| {
            let (from, tail) = line.split_once(" = ").unwrap();
            let (left, right) = tail[..tail.len() - 1][1..].split_once(", ").unwrap();
            (from, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut current = "AAA";
    let mut count = 0;
    for direction in lr.into_iter().cycle() {
        current = if direction {
            network[current].0
        } else {
            network[current].1
        };
        count += 1;

        if current == "ZZZ" {
            break;
        }
    }

    count
}

pub fn solve_second(input: &str) -> usize {
    let mut lines = input.lines();
    let lr = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c == 'L')
        .collect::<Vec<_>>();

    lines.next().unwrap(); // empty

    let network = lines
        .map(|line| {
            let (from, tail) = line.split_once(" = ").unwrap();
            let (left, right) = tail[..tail.len() - 1][1..].split_once(", ").unwrap();
            (from, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut current = network
        .keys()
        .copied()
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();

    current
        .iter()
        .map(|start| {
            let mut current = *start;
            let mut count = 0;
            for &direction in lr.iter().cycle() {
                current = if direction {
                    network[current].0
                } else {
                    network[current].1
                };
                count += 1;

                if current.ends_with('Z') {
                    break;
                }
            }
            count
        })
        // TODO this assumes quite a lot about the input
        //  - loops immediately and doesn't enter the loop later
        //  - only one goal in each loop
        //  - time to goal is the cycle time
        .reduce(|acc, count| acc.lcm(&count))
        .unwrap()
}

#[test]
pub fn sample() {
    assert_eq!(6, solve_first(include_str!("sample1.txt")));
    assert_eq!(6, solve_second(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(12643, solve_first(input));
    assert_eq!(13133452426987, solve_second(input));
}

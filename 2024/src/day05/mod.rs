use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let orderings = lines
        .by_ref()
        .take_while(|&line| !line.is_empty())
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .map(|(a, b)| (b, a))
        .into_group_map();

    let (mut one, mut two) = (0, 0);
    for update in lines {
        let mut update = update.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

        let mut correct = true;
        'outer: loop {
            let mut disallowed = HashMap::new();
            for (i, x) in update.iter().enumerate() {
                if let Some(&j) = disallowed.get(x) {
                    correct = false;
                    // not a very good sorting algorithm, but it works
                    update.swap(i, j);
                    continue 'outer;
                }

                for y in orderings.get(x).unwrap_or(&Vec::new()) {
                    disallowed.insert(*y, i);
                }
            }

            break 'outer;
        }

        if correct {
            one += update[update.len() / 2];
        } else {
            two += update[update.len() / 2];
        }
    }

    (one, two)
}

#[test]
pub fn sample() {
    let (one, two) = solve(include_str!("sample.txt"));
    assert_eq!(143, one);
    assert_eq!(123, two);
}

#[test]
pub fn input() {
    let (one, two) = solve(include_str!("input.txt"));
    assert_eq!(6612, one);
    assert_eq!(4944, two);
}

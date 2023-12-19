use std::cmp::{max, min};
use std::collections::HashMap;

pub fn solve_first(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\r\n\r\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, rules) = line[..line.len() - 1].split_once('{').unwrap();
            let rules = rules
                .split(',')
                .map(|rule| {
                    if let Some((condition, to)) = rule.split_once(':') {
                        let rating = match condition.chars().next().unwrap() {
                            'x' => 0_usize,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => unreachable!(),
                        };
                        let comp = condition.chars().nth(1).unwrap();
                        let value = condition[2..].parse::<usize>().unwrap();
                        (rating, comp, value, to)
                    } else {
                        (4, '>', 0, rule)
                    }
                })
                .collect::<Vec<_>>();
            (name, rules)
        })
        .collect::<HashMap<_, _>>();

    parts
        .lines()
        .map(|line| -> [_; 4] {
            line[1..line.len() - 1]
                .split(',')
                .map(|val| val[2..].parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .filter(|ratings| {
            let mut current = "in";

            while current != "A" && current != "R" {
                for (rating, comp, value, to) in &workflows[current] {
                    if *rating == 4 {
                        current = *to;
                        break;
                    } else {
                        match comp {
                            '<' if ratings[*rating] < *value => {
                                current = *to;
                                break;
                            }
                            '>' if ratings[*rating] > *value => {
                                current = *to;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }

            current == "A"
        })
        .map(|ratings| ratings.into_iter().sum::<usize>())
        .sum()
}

pub fn solve_second(input: &str) -> usize {
    let (workflows, _) = input.split_once("\r\n\r\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, rules) = line[..line.len() - 1].split_once('{').unwrap();
            let rules = rules
                .split(',')
                .map(|rule| {
                    if let Some((condition, to)) = rule.split_once(':') {
                        let rating = match condition.chars().next().unwrap() {
                            'x' => 0_usize,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => unreachable!(),
                        };
                        let comp = condition.chars().nth(1).unwrap();
                        let value = condition[2..].parse::<usize>().unwrap();
                        (rating, comp, value, to)
                    } else {
                        (4, '>', 0, rule)
                    }
                })
                .collect::<Vec<_>>();
            (name, rules)
        })
        .collect::<HashMap<_, _>>();

    let mut stack = vec![([1_usize..4001, 1..4001, 1..4001, 1..4001], "in")];
    let mut accepted = 0;

    while let Some((mut ratings, current)) = stack.pop() {
        if current == "A" {
            accepted += ratings
                .into_iter()
                .map(|range| range.len())
                .product::<usize>();
        } else if current != "R" {
            for (rating, comp, value, to) in &workflows[current] {
                if ratings.iter().any(|range| range.is_empty()) {
                    continue;
                }

                if *rating == 4 {
                    stack.push((ratings.clone(), *to));
                    break;
                } else {
                    match comp {
                        '<' if ratings[*rating].start < *value => {
                            let mut next = ratings.clone();
                            next[*rating].end = min(*value, next[*rating].end);
                            stack.push((next, *to));

                            ratings[*rating].start = *value;
                        }
                        '>' if ratings[*rating].end >= *value => {
                            let mut next = ratings.clone();
                            next[*rating].start = max(*value + 1, next[*rating].start);
                            stack.push((next, *to));

                            ratings[*rating].end = *value + 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    accepted
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(19114, solve_first(sample));
    assert_eq!(167409079868000, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(319295, solve_first(input));
    assert_eq!(110807725108076, solve_second(input));
}

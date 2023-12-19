use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Range;

pub fn solve<'a>(
    workflows: Vec<&'a str>,
    mut stack: Vec<([Range<usize>; 4], &'a str)>,
    combinations: bool,
) -> usize {
    let workflows = workflows
        .into_iter()
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
                        (Some(rating), comp, value, to)
                    } else {
                        (None, ' ', 0, rule)
                    }
                })
                .collect::<Vec<_>>();
            (name, rules)
        })
        .collect::<HashMap<_, _>>();

    let mut accepted = 0;

    while let Some((mut ratings, current)) = stack.pop() {
        if current == "A" {
            if combinations {
                // part 2
                accepted += ratings.into_iter().map(|r| r.len()).product::<usize>();
            } else {
                // part 1
                accepted += ratings.into_iter().map(|r| r.start).sum::<usize>();
            }
        } else if current != "R" {
            for (rating, comp, value, to) in &workflows[current] {
                if ratings.iter().any(|range| range.is_empty()) {
                    continue;
                }

                if let Some(rating) = *rating {
                    // split ranges at comparison point
                    match comp {
                        '<' if ratings[rating].start < *value => {
                            let mut next = ratings.clone();
                            next[rating].end = min(*value, next[rating].end);
                            stack.push((next, *to));

                            ratings[rating].start = *value;
                        }
                        '>' if ratings[rating].end >= *value => {
                            let mut next = ratings.clone();
                            next[rating].start = max(*value + 1, next[rating].start);
                            stack.push((next, *to));

                            ratings[rating].end = *value + 1;
                        }
                        _ => {}
                    }
                } else {
                    // fall-through workflow
                    stack.push((ratings.clone(), *to));
                    break;
                }
            }
        }
    }

    accepted
}

pub fn solve_first(input: &str) -> usize {
    let mut lines = input.lines();
    let workflows = lines.by_ref().take_while(|line| !line.is_empty()).collect();
    let parts = lines;

    // implement the first part as ranges with length 1
    let parts = parts
        .map(|line| -> [_; 4] {
            line[1..line.len() - 1]
                .split(',')
                .map(|val| val[2..].parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .map(|ratings| (ratings.map(|x| x..x + 1), "in"))
        .collect();

    solve(workflows, parts, false)
}

pub fn solve_second(input: &str) -> usize {
    let workflows = input.lines().take_while(|line| !line.is_empty()).collect();

    solve(
        workflows,
        vec![([1_usize..4001, 1..4001, 1..4001, 1..4001], "in")],
        true,
    )
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

use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
enum Monkey {
    Value(isize),
    Operation([fn(isize, isize) -> isize; 3], usize, usize),
}

fn parse(input: &str) -> (HashMap<&str, usize>, Vec<Monkey>) {
    let indices = input
        .lines()
        .enumerate()
        .map(|(i, x)| (&x[0..4], i))
        .collect::<HashMap<_, _>>();
    let nodes = input
        .lines()
        .map(
            |line| match line.split_whitespace().collect_vec().as_slice() {
                [_, x, op, y] => Monkey::Operation(
                    match *op {
                        "+" => [isize::add, isize::sub, isize::sub],
                        "-" => [isize::sub, isize::add, |x, y| y - x],
                        "*" => [isize::mul, isize::div, isize::div],
                        "/" => [isize::div, isize::mul, |x, y| y / x],
                        _ => unreachable!(),
                    },
                    indices[x],
                    indices[y],
                ),
                [_, value] => Monkey::Value(value.parse().unwrap()),
                _ => unreachable!(),
            },
        )
        .collect();

    (indices, nodes)
}

pub fn solve_first(input: &str) -> isize {
    let (indices, nodes) = parse(input);

    fn value(monkey: usize, nodes: &[Monkey]) -> isize {
        match nodes[monkey] {
            Monkey::Value(v) => v,
            Monkey::Operation([op, ..], x, y) => op(value(x, nodes), value(y, nodes)),
        }
    }

    value(indices["root"], &nodes)
}

pub fn solve_second(input: &str) -> isize {
    let (indices, nodes) = parse(input);

    fn value(monkey: usize, human: usize, nodes: &[Monkey]) -> Option<isize> {
        if monkey == human {
            return None;
        }
        match nodes[monkey] {
            Monkey::Value(v) => Some(v),
            Monkey::Operation([op, ..], x, y) => {
                Some(op(value(x, human, nodes)?, value(y, human, nodes)?))
            }
        }
    }

    fn solve(monkey: usize, s: isize, human: usize, nodes: &[Monkey]) -> isize {
        match nodes[monkey] {
            Monkey::Value(_) => s,
            Monkey::Operation([_, solve_x, solve_y], x, y) => {
                if let Some(v) = value(x, human, nodes) {
                    solve(y, solve_y(s, v), human, nodes)
                } else {
                    solve(x, solve_x(s, value(y, human, nodes).unwrap()), human, nodes)
                }
            }
        }
    }

    let root_value = 2 * match nodes[indices["root"]] {
        Monkey::Operation(_, x, y) => value(x, indices["humn"], &nodes)
            .unwrap_or_else(|| value(y, indices["humn"], &nodes).unwrap()),
        Monkey::Value(_) => unreachable!(),
    };

    solve(indices["root"], root_value, indices["humn"], &nodes)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(152, solve_first(sample));
    assert_eq!(301, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(158661812617812, solve_first(input));
    assert_eq!(3352886133831, solve_second(input));
}

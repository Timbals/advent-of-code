use itertools::Itertools;
use std::cell::RefCell;
use std::iter::zip;

pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
}

pub enum Operation {
    Multiply(u64),
    Add(u64),
    Square,
}

pub fn parse(input: &str) -> Vec<RefCell<Monkey>> {
    input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|mut monkey| {
            monkey.next();
            let items = monkey
                .next()
                .unwrap()
                .trim_start_matches("  Starting items: ")
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            let mut operation = monkey
                .next()
                .unwrap()
                .trim_start_matches("  Operation: new = old ")
                .split_whitespace();
            let operator = operation.next().unwrap();
            let value = operation.next().unwrap();
            let operation = match [operator, value] {
                ["*", "old"] => Operation::Square,
                ["*", _] => Operation::Multiply(value.parse::<u64>().unwrap()),
                ["+", _] => Operation::Add(value.parse::<u64>().unwrap()),
                _ => unreachable!(),
            };

            let test = monkey
                .next()
                .unwrap()
                .trim_start_matches("  Test: divisible by ")
                .parse()
                .unwrap();
            let if_true = monkey
                .next()
                .unwrap()
                .trim_start_matches("    If true: throw to monkey ")
                .parse()
                .unwrap();
            let if_false = monkey
                .next()
                .unwrap()
                .trim_start_matches("    If false: throw to monkey ")
                .parse()
                .unwrap();

            Monkey {
                items,
                operation,
                test,
                if_true,
                if_false,
            }
        })
        .map(RefCell::new)
        .collect()
}

pub fn solve_first(input: &str) -> u64 {
    let monkeys = parse(input);
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for (monkey, inspections) in zip(monkeys.iter(), inspections.iter_mut()) {
            let monkey = &mut *monkey.borrow_mut();
            let mut if_true = monkeys[monkey.if_true].borrow_mut();
            let mut if_false = monkeys[monkey.if_false].borrow_mut();

            for item in monkey.items.drain(..) {
                *inspections += 1;

                let item = match monkey.operation {
                    Operation::Multiply(x) => (item * x) / 3,
                    Operation::Add(x) => (item + x) / 3,
                    Operation::Square => (item * item) / 3,
                };
                if item % monkey.test == 0 {
                    if_true.items.push(item);
                } else {
                    if_false.items.push(item);
                }
            }
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

pub fn solve_second(input: &str) -> u64 {
    let monkeys = parse(input);
    let mut inspections = vec![0; monkeys.len()];

    let all_tests = monkeys.iter().map(|x| x.borrow().test).product::<u64>();

    for _ in 0..10000 {
        for (monkey, inspections) in zip(monkeys.iter(), inspections.iter_mut()) {
            let monkey = &mut *monkey.borrow_mut();
            let mut if_true = monkeys[monkey.if_true].borrow_mut();
            let mut if_false = monkeys[monkey.if_false].borrow_mut();

            for item in monkey.items.drain(..) {
                *inspections += 1;

                let item = match monkey.operation {
                    Operation::Multiply(x) => item * x,
                    Operation::Add(x) => item + x,
                    Operation::Square => item * item,
                } % all_tests;
                if item % monkey.test == 0 {
                    if_true.items.push(item);
                } else {
                    if_false.items.push(item);
                }
            }
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(10605, solve_first(sample));
    assert_eq!(2713310158, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(90882, solve_first(input));
    assert_eq!(30893109657, solve_second(input));
}

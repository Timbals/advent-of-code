use itertools::Itertools;

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

pub fn parse(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|mut monkey| {
            monkey.next();

            let items = monkey.next().unwrap()[18..]
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            let operation = &monkey.next().unwrap()[23..];
            let operation = match (&operation[..1], &operation[2..]) {
                ("*", "old") => Operation::Square,
                ("*", value) => Operation::Multiply(value.parse::<u64>().unwrap()),
                ("+", value) => Operation::Add(value.parse::<u64>().unwrap()),
                _ => unreachable!(),
            };

            Monkey {
                items,
                operation,
                test: monkey.next().unwrap()[21..].parse().unwrap(),
                if_true: monkey.next().unwrap()[29..].parse().unwrap(),
                if_false: monkey.next().unwrap()[30..].parse().unwrap(),
            }
        })
        .collect()
}

pub fn solve<const ROUNDS: usize, const RELIEF: bool>(input: &str) -> u64 {
    let mut monkeys = parse(input);
    let mut inspections = vec![0; monkeys.len()];

    let all_tests = monkeys.iter().map(|x| x.test).product::<u64>();

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            let (left, right) = monkeys.split_at_mut(i);
            let (monkey, right) = right.split_at_mut(1);
            let monkey = &mut monkey[0];

            for old in monkey.items.drain(..) {
                inspections[i] += 1;

                let new = match monkey.operation {
                    Operation::Multiply(x) => old * x,
                    Operation::Add(x) => old + x,
                    Operation::Square => old * old,
                };

                let item = if RELIEF { new / 3 } else { new % all_tests };

                let next = if item % monkey.test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                if next < i {
                    left[next].items.push(item);
                } else {
                    right[next - i - 1].items.push(item);
                }
            }
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

pub fn solve_first(input: &str) -> u64 {
    solve::<20, true>(input)
}

pub fn solve_second(input: &str) -> u64 {
    solve::<10000, false>(input)
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

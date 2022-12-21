use itertools::Itertools;
use std::collections::HashMap;
use std::mem::swap;

#[derive(Copy, Clone, Debug)]
enum Monkey {
    Value(isize),
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
}

pub fn solve_first(input: &str) -> isize {
    let indices = input
        .lines()
        .enumerate()
        .map(|(i, x)| (&x[0..4], i))
        .collect::<HashMap<_, _>>();
    let monkeys = input
        .lines()
        .map(|line| {
            match line
                .split_at(6)
                .1
                .split_whitespace()
                .collect_vec()
                .as_slice()
            {
                [op1, operand, op2] => {
                    let op1 = indices[op1];
                    let op2 = indices[op2];
                    match *operand {
                        "+" => Monkey::Add(op1, op2),
                        "-" => Monkey::Sub(op1, op2),
                        "*" => Monkey::Mul(op1, op2),
                        "/" => Monkey::Div(op1, op2),
                        _ => unreachable!(),
                    }
                }
                [value] => Monkey::Value(value.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    fn calc_value(monkey: usize, monkeys: &[Monkey]) -> isize {
        match monkeys[monkey] {
            Monkey::Value(v) => v,
            Monkey::Add(op1, op2) => calc_value(op1, monkeys) + calc_value(op2, monkeys),
            Monkey::Sub(op1, op2) => calc_value(op1, monkeys) - calc_value(op2, monkeys),
            Monkey::Mul(op1, op2) => calc_value(op1, monkeys) * calc_value(op2, monkeys),
            Monkey::Div(op1, op2) => calc_value(op1, monkeys) / calc_value(op2, monkeys),
        }
    }

    calc_value(indices["root"], &monkeys)
}

pub fn solve_second(input: &str) -> isize {
    let indices = input
        .lines()
        .enumerate()
        .map(|(i, x)| (&x[0..4], i))
        .collect::<HashMap<_, _>>();
    let mut monkeys = input
        .lines()
        .map(|line| {
            match line
                .split_at(6)
                .1
                .split_whitespace()
                .collect_vec()
                .as_slice()
            {
                [op1, operand, op2] => {
                    let op1 = indices[op1];
                    let op2 = indices[op2];
                    match *operand {
                        "+" => Monkey::Add(op1, op2),
                        "-" => Monkey::Sub(op1, op2),
                        "*" => Monkey::Mul(op1, op2),
                        "/" => Monkey::Div(op1, op2),
                        _ => unreachable!(),
                    }
                }
                [value] => Monkey::Value(value.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    fn calc_value_2<'a>(
        monkey: usize,
        monkeys: &'a mut Vec<Monkey>,
        new_monkeys: &'a mut Vec<Monkey>,
        cache: &mut Vec<Option<isize>>,
        count: usize,
    ) -> isize {
        if let Some(value) = cache[monkey] {
            return value;
        }

        dbg!(monkey, &monkeys[monkey], count);

        if count == 5000 {
            panic!();
        }

        let root_monkey = 779; // 0
        let switch_monkey = 354; // 11

        if monkey == 455 && count == 129 {
            //return calc_value(monkey, new_monkeys, monkeys, cache, count + 1);
        }

        let value = match monkeys[monkey] {
            Monkey::Value(v) => v,
            Monkey::Add(op1, op2) => {
                calc_value_2(op1, monkeys, new_monkeys, cache, count + 1)
                    + calc_value_2(op2, monkeys, new_monkeys, cache, count + 1)
            }
            Monkey::Sub(op1, op2) => {
                if op1 == root_monkey || op2 == root_monkey {
                    dbg!("swap");
                    swap(monkeys, new_monkeys);
                    if op1 == root_monkey {
                        calc_value_2(op2, monkeys, new_monkeys, cache, count + 1)
                    } else {
                        calc_value_2(op1, monkeys, new_monkeys, cache, count + 1)
                    }
                } else {
                    calc_value_2(op1, monkeys, new_monkeys, cache, count + 1)
                        - calc_value_2(op2, monkeys, new_monkeys, cache, count + 1)
                }
            }
            Monkey::Mul(op1, op2) => {
                calc_value_2(op1, monkeys, new_monkeys, cache, count + 1)
                    * calc_value_2(op2, monkeys, new_monkeys, cache, count + 1)
            }
            Monkey::Div(op1, op2) => {
                calc_value_2(op1, monkeys, new_monkeys, cache, count + 1)
                    / calc_value_2(op2, monkeys, new_monkeys, cache, count + 1)
            }
        };

        cache[monkey] = Some(value);
        dbg!(monkey, value);

        if monkey == switch_monkey {
            //swap(monkeys, new_monkeys);
            //return calc_value(monkey, new_monkeys, monkeys, cache, count + 1);
        }

        value
    }
    fn value(monkey: usize, monkeys: &[Monkey]) -> Option<isize> {
        if monkey == 1482 {
            return None;
        }
        match monkeys[monkey] {
            Monkey::Value(v) => Some(v),
            Monkey::Add(op1, op2) => {
                value(op1, monkeys).and_then(|v| value(op2, monkeys).map(|w| v + w))
            }
            Monkey::Sub(op1, op2) => {
                value(op1, monkeys).and_then(|v| value(op2, monkeys).map(|w| v - w))
            }
            Monkey::Mul(op1, op2) => {
                value(op1, monkeys).and_then(|v| value(op2, monkeys).map(|w| v * w))
            }
            Monkey::Div(op1, op2) => {
                value(op1, monkeys).and_then(|v| value(op2, monkeys).map(|w| v / w))
            }
        }
    }

    let mut new_monkeys = monkeys.clone();
    for (i, monkey) in monkeys.iter().enumerate() {
        match monkey {
            Monkey::Add(op1, op2) => {
                new_monkeys[*op1] = Monkey::Sub(i, *op2);
                new_monkeys[*op2] = Monkey::Sub(i, *op1);
            }
            Monkey::Sub(op1, op2) => {
                new_monkeys[*op1] = Monkey::Add(i, *op2);
                new_monkeys[*op2] = Monkey::Sub(*op1, i);
            }
            Monkey::Mul(op1, op2) => {
                new_monkeys[*op1] = Monkey::Div(i, *op2);
                new_monkeys[*op2] = Monkey::Div(i, *op1);
            }
            Monkey::Div(op1, op2) => {
                new_monkeys[*op1] = Monkey::Mul(i, *op2);
                new_monkeys[*op2] = Monkey::Div(*op1, i);
            }
            Monkey::Value(_) => {}
        }
    }
    let humn = indices["humn"];
    for (i, monkey) in monkeys.iter().enumerate() {
        match monkey {
            Monkey::Value(_) if i != humn => new_monkeys[i] = *monkey,
            _ => {}
        }
    }

    //dbg!(&new_monkeys);
    //let mut cache = monkeys.iter().map(|_| None).collect_vec();

    //calc_value_2(
    // indices["nfvg"],
    //&mut monkeys,
    // &mut new_monkeys,
    //  &mut cache,
    //   0,
    //);
    //assert!(cache[humn].is_none());

    new_monkeys[indices["hqpw"]] = Monkey::Value(52716091087786);

    for monkey in monkeys.iter() {
        match monkey {
            Monkey::Value(_) => {}
            Monkey::Add(op1, op2) => {
                assert!(value(*op1, &monkeys).is_some() || value(*op2, &monkeys).is_some())
            }
            Monkey::Sub(op1, op2) => {
                assert!(value(*op1, &monkeys).is_some() || value(*op2, &monkeys).is_some())
            }
            Monkey::Mul(op1, op2) => {
                assert!(value(*op1, &monkeys).is_some() || value(*op2, &monkeys).is_some())
            }
            Monkey::Div(op1, op2) => {
                assert!(value(*op1, &monkeys).is_some() || value(*op2, &monkeys).is_some())
            }
        }
    }

    let mut current = monkeys[indices["hqpw"]];
    let mut current_value = 52716091087786;
    loop {
        match current {
            Monkey::Value(_) => return current_value,
            Monkey::Add(op1, op2) => {
                if let Some(x1) = value(op1, &monkeys) {
                    current_value = current_value - x1;
                    current = monkeys[op2];
                } else if let Some(x2) = value(op2, &monkeys) {
                    current_value = current_value - x2;
                    current = monkeys[op1];
                }
            }
            Monkey::Sub(op1, op2) => {
                if let Some(x1) = value(op1, &monkeys) {
                    current_value = x1 - current_value;
                    current = monkeys[op2];
                } else if let Some(x2) = value(op2, &monkeys) {
                    current_value = current_value + x2;
                    current = monkeys[op1];
                }
            }
            Monkey::Mul(op1, op2) => {
                if let Some(x1) = value(op1, &monkeys) {
                    current_value = current_value / x1;
                    current = monkeys[op2];
                } else if let Some(x2) = value(op2, &monkeys) {
                    current_value = current_value / x2;
                    current = monkeys[op1];
                }
            }
            Monkey::Div(op1, op2) => {
                if let Some(x1) = value(op1, &monkeys) {
                    current_value = x1 / current_value;
                    current = monkeys[op2];
                } else if let Some(x2) = value(op2, &monkeys) {
                    current_value = current_value * x2;
                    current = monkeys[op1];
                }
            }
        }
    }
    //value(indices["qcjd"], &new_monkeys, 0) - 203

    //calc_value_2(
    //    indices["qcjd"],
    //    &mut new_monkeys,
    //   &mut monkeys,
    //   &mut cache,
    //   0,
    //) - calc_value_2(
    //   indices["nfvg"],
    //   &mut monkeys,
    //   &mut new_monkeys,
    //   &mut cache,
    //   0,
    //)

    //calc_value(humn, &mut new_monkeys, &mut monkeys, &mut cache, 0)
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
    assert_eq!(3352886133831, solve_second(input)); // 2313 too low
}

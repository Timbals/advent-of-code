use crate::day20::ModuleType::{Broadcast, Conjunction, FlipFlop};
use num::Integer;
use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
enum ModuleType<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

fn parse(input: &str) -> HashMap<&str, (ModuleType, Vec<&str>)> {
    let mut inputs_by_label = HashMap::<_, Vec<&str>>::new();
    let mut modules = input
        .lines()
        .map(|line| {
            let (label, destinations) = line.split_once(" -> ").unwrap();
            let destinations = destinations.split(", ").collect::<Vec<_>>();

            let (label, module) = match label.chars().next().unwrap() {
                '%' => (&label[1..], FlipFlop(false)),
                '&' => (&label[1..], Conjunction(HashMap::new())),
                'b' => (label, Broadcast),
                _ => unreachable!(),
            };

            for &destination in &destinations {
                inputs_by_label.entry(destination).or_default().push(label);
            }

            (label, (module, destinations))
        })
        .collect::<HashMap<_, _>>();

    // initialize conjunction modules
    for (label, (module, _)) in modules.iter_mut() {
        if let Conjunction(inputs) = module {
            *inputs = inputs_by_label[label].iter().map(|&x| (x, false)).collect();
        }
    }

    modules
}

fn step<'a>(
    modules: &mut HashMap<&str, (ModuleType<'a>, Vec<&'a str>)>,
    initial_pulse: (&'a str, bool, &'a str),
    early_stop_module: Option<&str>,
) -> Option<(usize, usize)> {
    let mut pulses = VecDeque::new();
    pulses.push_back(initial_pulse);

    let mut low_count = 0;
    let mut high_count = 0;

    while let Some((label, pulse, from)) = pulses.pop_front() {
        if pulse {
            high_count += 1;
        } else {
            low_count += 1;
        }

        if let Some(early_stop_module) = early_stop_module {
            if label == early_stop_module && pulse {
                return None;
            }
        }

        match modules.get_mut(label) {
            Some((Broadcast, ref destinations)) => {
                for &destination in destinations {
                    pulses.push_back((destination, pulse, label));
                }
            }
            Some((FlipFlop(state), ref destinations)) => {
                if !pulse {
                    *state = !*state;
                    for &destination in destinations {
                        pulses.push_back((destination, *state, label));
                    }
                }
            }
            Some((Conjunction(state), ref destinations)) => {
                *state.get_mut(&from).unwrap() = pulse;

                let out = !state.values().all(|&x| x);
                for &destination in destinations {
                    pulses.push_back((destination, out, label));
                }
            }
            None => {} // untyped module
        }
    }

    Some((low_count, high_count))
}

pub fn solve_first(input: &str) -> usize {
    let mut modules = parse(input);

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let (low_inc, high_inc) = step(&mut modules, ("broadcaster", false, ""), None).unwrap();
        low_count += low_inc;
        high_count += high_inc;
    }

    low_count * high_count
}

pub fn solve_second(input: &str) -> usize {
    let modules = parse(input);

    // this assumes quite a bit about the input
    // basically that the broadcaster feeds into binary counters
    // and the rx module gets a low pulse when all binary counters reach their final values at the same time
    modules["broadcaster"]
        .1
        .iter()
        .map(|&origin| {
            let mut modules = modules.clone();
            let mut count = 1;
            while step(&mut modules, (origin, false, "broadcaster"), Some("df")).is_some() {
                count += 1;
            }
            count
        })
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}

#[test]
pub fn sample() {
    assert_eq!(32000000, solve_first(include_str!("sample1.txt")));
    assert_eq!(11687500, solve_first(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(666795063, solve_first(input));
    assert_eq!(253302889093151, solve_second(input));
}

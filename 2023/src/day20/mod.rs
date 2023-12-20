use crate::day20::ModuleType::{Broadcast, Conjunction, FlipFlop};
use num::Integer;
use std::collections::{HashMap, VecDeque};

#[derive(Eq, PartialEq, Clone)]
enum ModuleType<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

pub fn solve_first(input: &str) -> usize {
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

    let mut pulses = VecDeque::new();

    let mut count = 0;

    let mut low_count = 0;
    let mut high_count = 0;

    while count < 1000 {
        count += 1;

        pulses.push_back(("broadcaster", false, ""));

        while let Some((label, pulse, from)) = pulses.pop_front() {
            if pulse {
                high_count += 1;
            } else {
                low_count += 1;
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
    }

    low_count * high_count
}

pub fn solve_second(input: &str) -> usize {
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

    // this assumes quite a bit about the input
    // basically that the broadcaster feeds into binary counters
    // and the rx module gets a low pulse when all binary counters reach their final values at the same time
    (0..modules["broadcaster"].1.len())
        .map(|i| find_cycle_length(modules.clone(), i))
        .reduce(|a, b| a.lcm(&b))
        .unwrap()
}

fn find_cycle_length<'a>(
    mut modules: HashMap<&str, (ModuleType<'a>, Vec<&'a str>)>,
    i: usize,
) -> usize {
    let mut pulses = VecDeque::new();
    let mut count = 0;
    loop {
        pulses.push_back(("broadcaster", false, ""));
        count += 1;

        while let Some((label, pulse, from)) = pulses.pop_front() {
            if label == "df" && pulse {
                return count;
            }

            match modules.get_mut(label) {
                Some((Broadcast, ref destinations)) => {
                    pulses.push_back((destinations[i], pulse, label));
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
    }
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(11687500, solve_first(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(666795063, solve_first(input));
    assert_eq!(253302889093151, solve_second(input));
}

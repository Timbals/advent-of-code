use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

pub fn solve_first(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut states = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (wire, state) = line.split_once(": ").unwrap();
            let state = state.parse::<u8>().unwrap() == 1;
            (wire, state)
        })
        .collect::<HashMap<&str, bool>>();

    let gates = lines
        .map(|line| {
            let (input, out) = line.split_once(" -> ").unwrap();
            let mut input = input.split_whitespace();
            let a = input.next().unwrap();
            let op = input.next().unwrap();
            let b = input.next().unwrap();
            (a, op, b, out)
        })
        .collect::<Vec<_>>();

    let mut done = false;
    while !done {
        done = true;
        for (a, op, b, out) in &gates {
            if !states.contains_key(out) && states.contains_key(a) && states.contains_key(b) {
                let state = match *op {
                    "AND" => states[a] && states[b],
                    "OR" => states[a] || states[b],
                    "XOR" => states[a] != states[b],
                    _ => unreachable!(),
                };
                states.insert(out, state);
                done = false;
            }
        }
    }

    states
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .sorted_unstable()
        .rev()
        .fold(0, |acc, (_, v)| (acc << 1) | v as u64)
}

pub fn solve_second(input: &str) -> String {
    let mut lines = input.lines();
    let mut _states = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (wire, state) = line.split_once(": ").unwrap();
            let state = state.parse::<u8>().unwrap() == 1;
            (wire, state)
        })
        .collect::<HashMap<&str, bool>>();

    let mut gates = lines
        .map(|line| {
            let (input, out) = line.split_once(" -> ").unwrap();
            let mut input = input.split_whitespace();
            let a = input.next().unwrap();
            let op = input.next().unwrap();
            let b = input.next().unwrap();
            (out, (op, BTreeSet::from([a, b])))
        })
        .collect::<HashMap<_, _>>();
    let mut gates_reverse = gates.iter().map(|(&k, v)| (v.clone(), k)).collect::<HashMap<_, _>>();

    let mut result = Vec::<&str>::new();
    let mut carry = gates_reverse[&("AND", BTreeSet::from(["x00", "y00"]))];
    for i in 1..45 {
        let x = format!("x{i:02}");
        let y = format!("y{i:02}");
        let z = format!("z{i:02}");
        println!("{x} {y} {z}");
        let mut v1 = gates_reverse[&("XOR", BTreeSet::from([x.as_str(), y.as_str()]))];
        if gates[z.as_str()] != ("XOR", BTreeSet::from([v1, carry])) {
            let left = z.as_str();
            let right = ("XOR", BTreeSet::from([v1, carry]));
            println!("{x} {y} {z} {v1} {carry}");
            if gates_reverse.contains_key(&right) {
                let a = gates.remove(left).unwrap();
                let b = gates_reverse.remove(&a).unwrap();
                let c = gates_reverse.remove(&right).unwrap();
                let d = gates.remove(c).unwrap();
                result.push(c);
                result.push(b);
                gates.insert(c, a.clone());
                gates.insert(b, d.clone());
                gates_reverse.insert(a, c);
                gates_reverse.insert(d, b);
            } else {
                let left = v1;
                let right = *gates[z.as_str()].1.iter().find(|val| **val != carry).unwrap();
                let a = gates.remove(left).unwrap();
                let d = gates.remove(right).unwrap();
                let b = gates_reverse.remove(&a).unwrap();
                let c = gates_reverse.remove(&d).unwrap();
                result.push(c);
                result.push(b);
                gates.insert(c, a.clone());
                gates.insert(b, d.clone());
                gates_reverse.insert(a, c);
                gates_reverse.insert(d, b);
                v1 = right;
            }
        }
        let new_carry = gates_reverse[&("AND", BTreeSet::from([x.as_str(), y.as_str()]))];
        let v2 = gates_reverse[&("AND", BTreeSet::from([v1, carry]))];

        println!("{x} {y} {z} {v1} {v2} {carry} {new_carry}");

        if !gates_reverse.contains_key(&("OR", BTreeSet::from([new_carry, v2]))) {
            let left = {
                let vals = &gates_reverse
                    .keys()
                    .find(|(op, vals)| *op == "AND" && vals.contains(carry))
                    .unwrap()
                    .1;
                *vals.iter().find(|val| **val != carry).unwrap()
            };
            let right = ("AND", BTreeSet::from([v1, carry]));
            let a = gates.remove(left).unwrap();
            let b = gates_reverse.remove(&a).unwrap();
            let c = gates_reverse.remove(&right).unwrap();
            let d = gates.remove(c).unwrap();
            result.push(c);
            result.push(b);
            gates.insert(c, a.clone());
            gates.insert(b, d.clone());
            gates_reverse.insert(a, c);
            gates_reverse.insert(d, b);
        }
        carry = gates_reverse[&("OR", BTreeSet::from([new_carry, v2]))];
    }

    result.sort_unstable();
    result.join(",")
}

#[test]
pub fn sample() {
    assert_eq!(4, solve_first(include_str!("sample.txt")));
    assert_eq!(2024, solve_first(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(53755311654662, solve_first(input));
    assert_eq!("dkr,ggk,hhh,htp,rhv,z05,z15,z20", solve_second(input));
}

use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

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
    let mut gates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let (input, out) = line.split_once(" -> ").unwrap();
            let mut input = input.split_whitespace();
            let a = input.next().unwrap();
            let op = input.next().unwrap();
            let b = input.next().unwrap();
            ((op, BTreeSet::from([a, b])), out)
        })
        .collect::<HashMap<_, _>>();

    let op_map = gates
        .iter()
        .flat_map(|((op, ops), out)| {
            let ops = ops.iter().copied().collect::<Vec<_>>();
            [
                (BTreeSet::from([out, ops[1], op]), ops[0]),
                (BTreeSet::from([out, ops[0], op]), ops[1]),
            ]
        })
        .collect::<HashMap<_, _>>();

    let mut result = Vec::<&str>::new();
    let mut carry = gates[&("AND", BTreeSet::from(["x00", "y00"]))];

    assert_eq!("z00", gates[&("XOR", BTreeSet::from(["x00", "y00"]))]);
    for i in 1..45 {
        let x = format!("x{i:02}");
        let y = format!("y{i:02}");
        let z = format!("z{i:02}");

        let mut x_and_y = gates[&("AND", BTreeSet::from([x.as_str(), y.as_str()]))];

        let mut x_xor_y = gates[&("XOR", BTreeSet::from([x.as_str(), y.as_str()]))];
        if let Some(&swap_x_xor_y) = op_map.get(&BTreeSet::from([z.as_str(), "XOR", carry])) {
            if x_xor_y != swap_x_xor_y {
                result.extend([x_xor_y, swap_x_xor_y]);
                if x_and_y == swap_x_xor_y {
                    x_and_y = x_xor_y;
                }
                x_xor_y = swap_x_xor_y;
            }
        } else if let Some(&z_swap) = gates.get(&("XOR", BTreeSet::from([x_xor_y, carry]))) {
            if x_and_y == z {
                x_and_y = z_swap;
            }
            if gates[&("AND", BTreeSet::from([x_xor_y, carry]))] == z {
                gates.insert(("AND", BTreeSet::from([x_xor_y, carry])), z_swap);
            }
            let new_carry = gates[&("AND", BTreeSet::from([x_xor_y, carry]))];
            if gates[&("OR", BTreeSet::from([x_and_y, new_carry]))] == z {
                gates.insert(("OR", BTreeSet::from([x_and_y, new_carry])), z_swap);
            }
            result.extend([z_swap, &*z.leak()]);
        } else {
            result.push(carry);
            result.push(op_map[&BTreeSet::from(["XOR", x_xor_y, z.as_str()])]);
        }

        let new_carry = gates[&("AND", BTreeSet::from([x_xor_y, carry]))];
        carry = gates[&("OR", BTreeSet::from([x_and_y, new_carry]))];
    }
    assert_eq!("z45", carry);

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

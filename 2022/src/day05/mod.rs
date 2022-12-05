use itertools::Itertools;
use std::array::from_fn;
use std::collections::VecDeque;

fn solve_first(input: &str) -> String {
    let mut lines = input.lines();

    let mut stacks = Vec::new();
    for line in (&mut lines).take_while(|x| !x.starts_with(" 1")) {
        for (i, cargo) in line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, cargo)| *cargo != ' ')
        {
            while stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            stacks[i].push_back(cargo);
        }
    }
    lines.next();

    for line in lines {
        let mut line = line.split(' ');
        let [quantity, from, to] = from_fn(|_| line.nth(1).unwrap().parse::<usize>().unwrap());

        for _ in 0..quantity {
            let cargo = stacks[from - 1].pop_front().unwrap();
            stacks[to - 1].push_front(cargo);
        }
    }

    stacks.iter().map(|x| x.front().unwrap()).collect()
}

fn solve_second(input: &str) -> String {
    let mut lines = input.lines();

    let mut stacks = Vec::new();
    for line in (&mut lines).take_while(|x| !x.starts_with(" 1")) {
        for (i, cargo) in line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, cargo)| *cargo != ' ')
        {
            while stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            stacks[i].push_back(cargo);
        }
    }
    lines.next();

    for line in lines {
        let mut line = line.split(' ');
        let [quantity, from, to] = from_fn(|_| line.nth(1).unwrap().parse::<usize>().unwrap());

        let cargos = stacks[from - 1].drain(0..quantity).collect_vec();
        for cargo in cargos.into_iter().rev() {
            stacks[to - 1].push_front(cargo);
        }
    }

    stacks.iter().map(|x| x.front().unwrap()).collect()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!("CMZ", solve_first(sample));
    assert_eq!("MCD", solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!("CFFHVVHNC", solve_first(input));
    assert_eq!("FSZWBPTBG", solve_second(input));
}

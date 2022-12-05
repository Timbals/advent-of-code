use itertools::{repeat_n, Itertools};
use std::array::from_fn;

fn parse(input: &str) -> (Vec<Vec<char>>, impl Iterator<Item = [usize; 3]> + '_) {
    let mut lines = input.lines().peekable();
    let mut stacks = repeat_n(Vec::new(), (lines.peek().unwrap().len() + 1) / 4).collect_vec();

    for line in (&mut lines).take_while(|x| !x.starts_with(" 1")) {
        for (i, cargo) in line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, cargo)| *cargo != ' ')
        {
            stacks[i].push(cargo);
        }
    }

    // reverse each stack because we parsed from top-to-bottom
    for stack in &mut stacks {
        let n = stack.len();
        for i in 0..n / 2 {
            stack.swap(i, n - i - 1);
        }
    }

    // skip empty line
    lines.next();

    let instructions = lines.map(|line| {
        let mut line = line.split(' ');
        from_fn(|_| line.nth(1).unwrap().parse::<usize>().unwrap())
    });

    (stacks, instructions)
}

fn solve_first(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);

    for [quantity, from, to] in instructions {
        for _ in 0..quantity {
            let cargo = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(cargo);
        }
    }

    stacks.iter().map(|x| x.last().unwrap()).collect()
}

fn solve_second(input: &str) -> String {
    let (mut stacks, instructions) = parse(input);

    for [quantity, from, to] in instructions {
        let n = stacks[from - 1].len();
        let moving = stacks[from - 1].split_off(n - quantity);
        stacks[to - 1].extend_from_slice(&moving);
    }

    stacks.iter().map(|x| x.last().unwrap()).collect()
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

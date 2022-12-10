use itertools::Itertools;
use std::iter::once;

pub fn solve_first(input: &str) -> isize {
    let mut x = 1;
    input
        .lines()
        .map(str::split_whitespace)
        .map(|x| x.collect::<Vec<_>>())
        .flat_map(|instruction| match instruction.as_slice() {
            ["addx", v] => {
                let v = v.parse::<isize>().unwrap();
                x += v;
                vec![x - v, x]
            }
            ["noop"] => vec![x],
            _ => unreachable!(),
        })
        .enumerate()
        .filter(|(cycle, _)| (*cycle as isize + 2 - 20) % 40 == 0)
        .map(|(cycle, x)| (cycle + 2) as isize * x)
        .sum()
}

pub fn solve_second(input: &str) -> String {
    let mut x = 1;
    once(x)
        .chain(
            input
                .lines()
                .map(str::split_whitespace)
                .map(|x| x.collect::<Vec<_>>())
                .flat_map(|instruction| match instruction.as_slice() {
                    ["addx", v] => {
                        let v = v.parse::<isize>().unwrap();
                        x += v;
                        vec![x - v, x]
                    }
                    ["noop"] => vec![x],
                    _ => unreachable!(),
                }),
        )
        .enumerate()
        .map(|(cycle, x)| ((cycle % 40) as isize, x))
        .chunks(40)
        .into_iter()
        .flat_map(|line| {
            line.map(
                |(position, x)| {
                    if (position - x).abs() <= 1 {
                        '#'
                    } else {
                        '.'
                    }
                },
            )
            .chain(once('\n'))
        })
        .collect()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(13140, solve_first(sample));
    println!("{}", solve_second(sample));
    // assert_eq!("", solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(11220, solve_first(input));
    println!("{}", solve_second(input));
    // BZPAJELK
    // assert_eq!(0, solve_second(input));
}

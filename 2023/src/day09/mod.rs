use itertools::Itertools;

pub fn solve_first(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            let mut sequences = vec![sequence];
            while !sequences.last().unwrap().iter().all(|&x| x == 0) {
                sequences.push(
                    sequences
                        .last()
                        .unwrap()
                        .iter()
                        .tuple_windows()
                        .map(|(x, y)| y - x)
                        .collect(),
                );
            }

            sequences.last_mut().unwrap().push(0);
            for i in (1..sequences.len()).rev() {
                let next_value = sequences[i - 1].last().unwrap() + sequences[i].last().unwrap();
                sequences[i - 1].push(next_value);
            }

            *sequences.first().unwrap().last().unwrap()
        })
        .sum()
}

pub fn solve_second(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let sequence = line
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            let mut sequences = vec![sequence];
            while !sequences.last().unwrap().iter().all(|&x| x == 0) {
                sequences.push(
                    sequences
                        .last()
                        .unwrap()
                        .iter()
                        .tuple_windows()
                        .map(|(x, y)| y - x)
                        .collect(),
                );
            }

            sequences.last_mut().unwrap().insert(0, 0);
            for i in (1..sequences.len()).rev() {
                let next_value = sequences[i - 1].first().unwrap() - sequences[i].first().unwrap();
                sequences[i - 1].insert(0, next_value);
            }

            *sequences.first().unwrap().first().unwrap()
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(114, solve_first(sample));
    assert_eq!(2, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1762065988, solve_first(input));
    assert_eq!(1066, solve_second(input));
}

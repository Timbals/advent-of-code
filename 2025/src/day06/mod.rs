use std::ops::{Add, Mul};

pub fn solve_first(input: &str) -> usize {
    let mut total = 0;
    let mut problems =
        vec![Vec::<usize>::new(); input.lines().next().unwrap().split_whitespace().count()];
    for line in input.lines() {
        for (i, x) in line.split_whitespace().enumerate() {
            match x.parse::<usize>() {
                Ok(x) => {
                    problems[i].push(x);
                }
                Err(_) if x == "*" => {
                    total += std::mem::take(&mut problems[i]).into_iter().product::<usize>();
                }
                Err(_) if x == "+" => {
                    total += std::mem::take(&mut problems[i]).into_iter().sum::<usize>();
                }
                _ => unreachable!(),
            }
        }
    }
    total
}

pub fn solve_second(input: &str) -> usize {
    let mut total = 0;

    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let mut acc = 0;
    let mut op: fn(usize, usize) -> usize = usize::add;
    for (i, x) in grid[grid.len() - 1].iter().enumerate() {
        match *x {
            b'*' => {
                total += acc;
                acc = 1;
                op = usize::mul;
            }
            b'+' => {
                total += acc;
                acc = 0;
                op = usize::add;
            }
            b' ' => {}
            _ => unreachable!(),
        }

        let mut number = 0;
        let mut any = false;
        for y in 0..grid.len() - 1 {
            if grid[y].get(i).map(|x| x.is_ascii_digit()).unwrap_or_default() {
                number = number * 10 + (grid[y][i] - b'0') as usize;
                any = true;
            }
        }
        if any {
            acc = op(acc, number);
        }
    }
    total += acc;
    total
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(4277556, solve_first(sample));
    assert_eq!(3263827, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(5552221122013, solve_first(input));
    assert_eq!(11371597126232, solve_second(input));
}

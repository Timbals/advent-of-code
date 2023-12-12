use itertools::{repeat_n, Itertools};
use std::collections::HashMap;

pub fn solve(input: &str, repeat_count: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();

            let springs = repeat_n(springs, repeat_count)
                .join("?")
                .chars()
                .collect::<Vec<_>>();
            let runs = repeat_n(groups, repeat_count)
                .join(",")
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut cache = HashMap::new();

            arrangements(&springs, &runs, None, &mut cache)
        })
        .sum()
}

fn arrangements(
    springs: &[char],
    runs: &[usize],
    run_size: Option<usize>,
    cache: &mut HashMap<(*const char, *const usize, Option<usize>), usize>,
) -> usize {
    if springs.is_empty() && run_size.is_none() {
        return runs.is_empty() as usize;
    }

    if let Some(&count) = cache.get(&(springs.as_ptr(), runs.as_ptr(), run_size)) {
        return count;
    }

    let mut count = 0;
    if let Some(run_size) = run_size {
        if (springs.is_empty() || springs[0] == '.' || springs[0] == '?') && runs[0] == run_size {
            count += arrangements(&springs[1.min(springs.len())..], &runs[1..], None, cache);
        }
        if !springs.is_empty() && (springs[0] == '#' || springs[0] == '?') && run_size < runs[0] {
            count += arrangements(&springs[1..], runs, Some(run_size + 1), cache);
        }
    } else {
        if springs[0] == '.' || springs[0] == '?' {
            count += arrangements(&springs[1..], runs, run_size, cache);
        }
        if (springs[0] == '#' || springs[0] == '?') && !runs.is_empty() {
            count += arrangements(&springs[1..], runs, Some(1), cache);
        }
    }

    cache.insert((springs.as_ptr(), runs.as_ptr(), run_size), count);

    count
}

pub fn solve_first(input: &str) -> usize {
    solve(input, 1)
}

pub fn solve_second(input: &str) -> usize {
    solve(input, 5)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(21, solve_first(sample));
    assert_eq!(525152, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(8075, solve_first(input));
    assert_eq!(4232520187524, solve_second(input));
}

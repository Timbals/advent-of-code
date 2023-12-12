use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat;

pub fn solve(input: &str, repeat_count: usize) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();

            let springs = repeat(springs).take(repeat_count).join("?");
            let groups = repeat(groups).take(repeat_count).join(",");

            let springs = springs
                .chars()
                .map(|c| match c {
                    '.' => Some(false), // operational
                    '#' => Some(true),  // damaged
                    '?' => None,        // unknown
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            let runs = groups
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut cache = HashMap::new();

            fn arrangements(
                index: usize,
                mut run_index: usize,
                run_size: Option<usize>,
                springs: &[Option<bool>],
                runs: &[usize],
                cache: &mut HashMap<(usize, usize, Option<usize>), usize>,
            ) -> usize {
                if index >= springs.len() {
                    if let Some(run_size) = run_size {
                        if run_index < runs.len() && runs[run_index] == run_size {
                            run_index += 1;
                        } else {
                            return 0;
                        }
                    }

                    if run_index != runs.len() {
                        return 0;
                    }

                    return 1;
                }

                if let Some(&x) = cache.get(&(index, run_index, run_size)) {
                    return x;
                }

                let res = match (springs[index], run_size) {
                    (Some(false), None) => {
                        arrangements(index + 1, run_index, run_size, springs, runs, cache)
                    }
                    (Some(true), None) if run_index < runs.len() => {
                        arrangements(index + 1, run_index, Some(1), springs, runs, cache)
                    }
                    (None, None) => {
                        arrangements(index + 1, run_index, run_size, springs, runs, cache)
                            + if run_index < runs.len() {
                                arrangements(index + 1, run_index, Some(1), springs, runs, cache)
                            } else {
                                0
                            }
                    }
                    (Some(false), Some(run_size))
                        if run_index < runs.len() && runs[run_index] == run_size =>
                    {
                        arrangements(index + 1, run_index + 1, None, springs, runs, cache)
                    }
                    (Some(true), Some(run_size)) if run_size < runs[run_index] => arrangements(
                        index + 1,
                        run_index,
                        Some(run_size + 1),
                        springs,
                        runs,
                        cache,
                    ),
                    (None, Some(run_size)) => {
                        let mut tmp = 0;
                        if run_size < runs[run_index] {
                            tmp += arrangements(
                                index + 1,
                                run_index,
                                Some(run_size + 1),
                                springs,
                                runs,
                                cache,
                            );
                        }
                        if run_index < runs.len() && runs[run_index] == run_size {
                            tmp +=
                                arrangements(index + 1, run_index + 1, None, springs, runs, cache);
                        }
                        tmp
                    }
                    _ => 0,
                };

                cache.insert((index, run_index, run_size), res);

                res
            }

            arrangements(0, 0, None, &springs, &runs, &mut cache)
        })
        .sum()
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

#![allow(clippy::needless_range_loop, reason = "doesn't work well with multi-dimensional arrays")]

use itertools::Itertools;
use std::collections::HashSet;
use std::iter::zip;

const MAX_BUTTONS: usize = 16; // 13 in my input

pub fn solve_first(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (lights, rest) = line.split_once(' ').unwrap();
            let (buttons, _) = rest.rsplit_once(' ').unwrap();

            let lights = lights.trim_matches(['[', ']']);
            assert!(lights.len() <= 16);
            let lights = lights.bytes().rev().fold(0_u16, |acc, byte| match byte {
                b'.' => acc << 1,
                b'#' => (acc << 1) | 1,
                _ => unreachable!(),
            });

            let buttons = buttons
                .split(' ')
                .map(|button| {
                    button
                        .trim_matches(['(', ')'])
                        .split(',')
                        .map(|i| i.parse::<u8>().unwrap())
                        .fold(0_u16, |acc, i| acc | (1 << i))
                })
                .collect::<Vec<_>>();

            for set in buttons.into_iter().powerset() {
                let len = set.len();
                let state = set.into_iter().reduce(|a, b| a ^ b).unwrap_or_default();
                if state == lights {
                    return len;
                }
            }

            unreachable!()
        })
        .sum()
}

pub fn solve_second(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(' ').unwrap();
            let (buttons, levels) = rest.rsplit_once(' ').unwrap();

            let mut joltages = levels
                .trim_matches(['{', '}'])
                .split(',')
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let buttons = buttons
                .split(' ')
                .map(|button| {
                    let mut column = vec![false; levels.len()];
                    for i in button
                        .trim_matches(['(', ')'])
                        .split(',')
                        .map(|i| i.parse::<usize>().unwrap())
                    {
                        column[i] = true;
                    }
                    column
                })
                .collect::<Vec<_>>();

            let mut matrix = joltages
                .iter()
                .enumerate()
                .map(|(i, _)| buttons.iter().map(|b| i32::from(b[i])).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let height = joltages.len();
            let width = buttons.len();

            let button_max = (0..width)
                .map(|button| {
                    joltages
                        .iter()
                        .enumerate()
                        .filter(|&(joltage_index, _)| matrix[joltage_index][button] == 1)
                        .map(|(_, &joltage)| joltage)
                        .min()
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>();

            // gaussian elimination
            let mut i = 0;
            while let Some((pivot_column, pivot_row)) =
                Itertools::cartesian_product(i..width, i..height)
                    .find(|&(column, row)| matrix[row][column].abs() != 0)
            {
                // swap rows to get a nice empty triangle in the bottom left
                matrix.swap(i, pivot_row);
                joltages.swap(i, pivot_row);

                // we only want integers, so we multiply all relevant rows by a big enough integer
                // to ensure division by `matrix[i][pivot_column]` only produces integers
                if matrix[i][pivot_column].abs() != 1 {
                    // TODO use least common multiple algorithm
                    let multiple = matrix[i..]
                        .iter()
                        .map(|row| row[pivot_column].abs())
                        .filter(|&x| x != 0)
                        .collect::<HashSet<_>>()
                        .into_iter()
                        .product::<i32>();

                    for j in i..height {
                        if matrix[j][pivot_column] == 0 {
                            continue;
                        }

                        let multiplier = multiple / matrix[j][pivot_column];
                        for k in i..width {
                            matrix[j][k] *= multiplier;
                        }
                        joltages[j] *= multiplier;
                    }
                }

                // eliminate everything below the pivot element `matrix[i][pivot_column]`
                for j in (i + 1)..height {
                    let multiplier = matrix[j][pivot_column] / matrix[i][pivot_column];
                    for k in i..width {
                        matrix[j][k] -= matrix[i][k] * multiplier;
                    }
                    joltages[j] -= joltages[i] * multiplier;
                }

                i += 1;
            }

            back_substitution(width - 1, [None; MAX_BUTTONS], &matrix, &joltages, &button_max)
                .unwrap()
        })
        .sum()
}

fn back_substitution(
    button: usize,
    mut state: [Option<i32>; MAX_BUTTONS],
    matrix: &Vec<Vec<i32>>,
    joltages: &Vec<i32>,
    button_max: &Vec<i32>,
) -> Option<i32> {
    if button >= button_max.len() {
        return Some(state.into_iter().flatten().sum());
    }

    let (row, &joltage) = zip(matrix, joltages).rev().find(|(row, _)| row[button] != 0).unwrap();

    if row[..button].iter().all(|&v| v == 0) {
        // unique solution
        let substitution =
            zip(&row[button + 1..], &state[button + 1..]).map(|(a, b)| a * b.unwrap()).sum::<i32>();
        let value = joltage - substitution;
        let non_integer = value % row[button] != 0;
        let value = value / row[button];
        if non_integer || value < 0 || value > button_max[button] {
            // solution is invalid
            return None;
        }

        state[button] = Some(value);
        back_substitution(button.wrapping_sub(1), state, matrix, joltages, button_max)
    } else {
        // no unique solution, so try all possibilities
        (0..=button_max[button])
            .filter_map(|presses| {
                state[button] = Some(presses);
                back_substitution(button.wrapping_sub(1), state, matrix, joltages, button_max)
            })
            .min()
    }
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(7, solve_first(sample));
    assert_eq!(33, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(578, solve_first(input));
    assert_eq!(20709, solve_second(input));
}

use itertools::Itertools;
use std::cmp::{Ordering, min};
use std::collections::HashSet;

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

fn solve_recursive_levels(
    levels: &[u8; 10],
    level_dependents: &[Vec<usize>; 10],
    buttons: &[[bool; 10]],
    next_level: usize,
    state: [Option<u8>; 16], // TODO smallvec
) -> usize {
    // println!("level {next_level}");
    if next_level >= levels.len() {
        return state.into_iter().flatten().map(usize::from).sum();
    }

    let target = levels[next_level];
    let dependents = &level_dependents[next_level];
    let mut current_level = 0_u8;
    let mut remaining_dependents = Vec::new();
    for &button in dependents {
        if let Some(amount) = state[button] {
            current_level = current_level.saturating_add(amount); // TODO
        } else {
            remaining_dependents.push(button);
        }
    }

    match current_level.cmp(&target) {
        Ordering::Less => {
            let remaining_presses = target - current_level;

            // TODO don't use the allocating `combinations_with_replacement`, and just count up instead
            remaining_dependents
                .into_iter()
                .combinations_with_replacement(remaining_presses as usize)
                .map(|additional_presses| {
                    let mut new_state = state;
                    let additional_presses = additional_presses.into_iter().chunk_by(|&i| i);
                    for (button, presses) in &additional_presses {
                        new_state[button] = Some(presses.count().try_into().unwrap());
                    }
                    solve_recursive_levels(
                        levels,
                        level_dependents,
                        buttons,
                        next_level + 1,
                        new_state,
                    )
                })
                .min()
                .unwrap_or(usize::MAX)
        }
        Ordering::Equal => {
            solve_recursive_levels(levels, level_dependents, buttons, next_level + 1, state)
        }
        Ordering::Greater => usize::MAX,
    }
}

fn solve_recursive_buttons(
    levels: &[u8; 10],
    level_dependents: &[Vec<usize>; 10],
    buttons: &[[bool; 10]],
    button: usize,
    state: [u8; 10],
) -> usize {
    if button >= buttons.len() {
        if state == *levels {
            return 0;
        } else {
            return usize::MAX / 2;
        }
    }

    let mut max_remaining_presses = u8::MAX;
    for (i, &b) in buttons[button].iter().enumerate() {
        if b {
            max_remaining_presses = min(max_remaining_presses, levels[i] - state[i]);
        }
    }

    if max_remaining_presses == 0 {
        return solve_recursive_buttons(levels, level_dependents, buttons, button + 1, state);
    }

    (0..=max_remaining_presses)
        .map(|presses| {
            if button <= 5 {
                println!("{button}: {presses}/{max_remaining_presses}");
            }
            let mut new_state = state;
            for (i, &b) in buttons[button].iter().enumerate() {
                if b {
                    new_state[i] += presses;
                }
            }
            presses as usize
                + solve_recursive_buttons(levels, level_dependents, buttons, button + 1, new_state)
        })
        .min()
        .unwrap()
}

pub fn solve_matrix(mut matrix: Vec<Vec<i32>>, mut joltages: Vec<i32>) -> i32 {
    let original_matrix = matrix.clone();
    let mut original_joltages = joltages.clone();

    // let mut matrix = [[0, 0, 1, 1, 1], [1, 1, 0, 0, 0], [1, 0, 1, 0, 0], [0, 1, 0, 1, 0]];
    // let mut matrix = [
    //     [0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1],
    //     [0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1],
    //     [0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1],
    //     [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1],
    //     [0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1],
    //     [0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0],
    //     [1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0],
    //     [1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1],
    //     [0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
    //     [1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1],
    // ];
    let height = matrix.len();
    let width = matrix[0].len();
    // let mut joltages = [26, 21, 14, 14];
    // let mut joltages = [64, 68, 82, 53, 82, 69, 85, 121, 51, 77];
    // let mut buttons: [usize; 5] = std::array::from_fn(|i| i);
    // let mut buttons: [usize; 13] = std::array::from_fn(|i| i);
    let mut buttons = (0..width).map(|i| i).collect::<Vec<_>>();

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

    println!("--");
    for row in &matrix {
        println!("{row:?}");
    }
    println!("{joltages:?}");

    for i in 0..height {
        let mut pivot_row = None;
        let mut pivot_column = None;
        for column in i..width {
            if let Some((row, _)) =
                matrix.iter().enumerate().skip(i).find(|(_, row)| row[column].abs() != 0)
            {
                pivot_row = Some(row);
                pivot_column = Some(column);
                break;
            }
        }
        let (Some(pivot_row), Some(pivot_column)) = (pivot_row, pivot_column) else {
            break;
        };

        matrix.swap(i, pivot_row);
        joltages.swap(i, pivot_row);

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

        for j in (i + 1)..height {
            let multiplier = matrix[j][pivot_column] / matrix[i][pivot_column];
            for k in i..width {
                matrix[j][k] -= matrix[i][k] * multiplier;
            }
            joltages[j] -= joltages[i] * multiplier;
        }
    }

    println!("--");
    for row in &matrix {
        println!("{row:?}");
    }
    println!("{joltages:?}");

    for (i, row) in matrix.iter().enumerate() {
        if row.iter().all(|v| *v == 0) {
            assert!(joltages[i] == 0);
        }
    }

    // let matrix = matrix.into_iter().map(|row| row.into_iter().collect()).collect();
    // let joltages = joltages.into_iter().collect();
    // let button_max = button_max.into_iter().collect();

    let mut best_state = [None; 13];
    let result = dbg!(solve_buttons(
        [None; 13],
        &mut best_state,
        &matrix,
        &joltages,
        buttons.len() - 1,
        &buttons,
        &button_max
    ));

    println!("{:?}", best_state.iter().flatten().collect::<Vec<_>>());
    let mut actual_joltages = vec![0; original_joltages.len()];
    for button in 0..width {
        for row in 0..height {
            if original_matrix[row][button] == 1 {
                actual_joltages[row] += best_state[button].unwrap();
            }
        }
    }
    actual_joltages.sort_unstable();
    original_joltages.sort_unstable();
    assert_eq!(actual_joltages, original_joltages);

    result
}

fn solve_buttons(
    mut state: [Option<i32>; 13],
    best_state: &mut [Option<i32>; 13],
    matrix: &Vec<Vec<i32>>,
    joltages: &Vec<i32>,
    button_index: usize,
    buttons: &[usize],
    button_max: &Vec<i32>,
) -> i32 {
    if button_index >= buttons.len() {
        if best_state.iter().flatten().count() == 0
            || state.into_iter().flatten().sum::<i32>()
                < best_state.iter().copied().flatten().sum::<i32>()
        {
            *best_state = state;
        }

        return state.into_iter().flatten().sum();
    }

    let button = buttons[button_index];
    let (row_index, _) =
        matrix.iter().enumerate().rev().find(|(_, row)| row[button_index] != 0).unwrap();

    if matrix[row_index][..button_index].iter().all(|&v| v == 0) {
        let value = joltages[row_index]
            - matrix[row_index]
                .iter()
                .enumerate()
                .skip(button_index + 1)
                .map(|(i, &multiplier)| state[buttons[i]].unwrap() * multiplier)
                .sum::<i32>();
        let has_remainder = value % matrix[row_index][button_index] != 0;
        let value = value / matrix[row_index][button_index];
        if has_remainder || value < 0 || value > button_max[button] {
            // println!("impossible {button_index} {button} -> {value} ({})", button_max[button]);

            return i32::MAX;
        }

        // println!("direct {button_index} {button} -> {value}");

        // let mut state = state.clone();
        state[button] = Some(value);
        solve_buttons(
            state,
            best_state,
            matrix,
            joltages,
            button_index.wrapping_sub(1),
            buttons,
            button_max,
        )
    } else {
        // if button_index < 8 {
        //     panic!("{state:?} {button_index} {row_index}");
        // }

        // TODO take `value` into account to prune invalid presses(?)
        (0..=button_max[button])
            .map(|presses| {
                // println!("combinations {button_index} {button} -> {presses}");

                // let mut state = state.clone();
                state[button] = Some(presses);
                solve_buttons(
                    state,
                    best_state,
                    matrix,
                    joltages,
                    button_index.wrapping_sub(1),
                    buttons,
                    button_max,
                )
            })
            .min()
            .unwrap_or(i32::MAX)
    }
}

pub fn solve_second(input: &str) -> i32 {
    input
        .lines()
        // .skip(2)
        // .take(1)
        // .par_bridge()
        .map(|line| {
            println!("{line}");
            let (_, rest) = line.split_once(' ').unwrap();
            let (buttons, levels) = rest.rsplit_once(' ').unwrap();

            let levels = levels
                .trim_matches(['{', '}'])
                .split(',')
                .map(|level| level.parse::<i32>().unwrap())
                // .pad_using(10, |_| 0)
                .collect::<Vec<_>>();
            // .try_into()
            // .unwrap();

            let buttons = buttons
                .split(' ')
                .map(|button| {
                    let mut vector = vec![false; levels.len()];
                    for i in button
                        .trim_matches(['(', ')'])
                        .split(',')
                        .map(|i| i.parse::<usize>().unwrap())
                    {
                        assert!(!vector[i]);
                        vector[i] = true;
                    }
                    vector
                })
                .collect::<Vec<_>>();

            // let level_dependents = std::array::from_fn::<_, 10, _>(|level_index| {
            //     buttons
            //         .iter()
            //         .enumerate()
            //         .filter(|(_, button)| button[level_index])
            //         .map(|(button_index, _)| button_index)
            //         .collect::<Vec<_>>()
            // });
            // println!("{level_dependents:?}");
            //
            // for foo in &level_dependents {
            //     print!("[");
            //     for x in 0..14 {
            //         if foo.contains(&x) {
            //             print!("1,");
            //         } else {
            //             print!("0,");
            //         }
            //     }
            //     println!("],")
            // }

            // let mut solution = vec![None::<u16>; buttons.len()];

            // let mut k = 0;
            // loop {
            //     for presses in buttons.iter().combinations_with_replacement(k) {
            //         let state = presses.into_iter().fold([0_u8; 10], |acc, press| {
            //             std::array::from_fn(|i| if press[i] { acc[i] + 1 } else { acc[i] })
            //         });
            //         if state == levels {
            //             println!("solved {line}: {k}");
            //             return k;
            //         }
            //     }
            //     k += 1;
            // }

            // dbg!(solve_recursive_levels(&levels, &level_dependents, &buttons, 0, [None; 16]))
            // dbg!(solve_recursive_buttons(&levels, &level_dependents, &buttons, 0, [0; 10]))

            let matrix = levels
                .iter()
                .enumerate()
                .map(|(i, _)| buttons.iter().map(|b| i32::from(b[i])).collect())
                .collect();

            solve_matrix(matrix, levels)
        })
        .sum()
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

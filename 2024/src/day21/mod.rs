use itertools::Itertools;
use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::iter::once;

const NUM_KEYPAD: [&[Option<u8>]; 4] = [
    &[Some(b'7'), Some(b'8'), Some(b'9')],
    &[Some(b'4'), Some(b'5'), Some(b'6')],
    &[Some(b'1'), Some(b'2'), Some(b'3')],
    &[None, Some(b'0'), Some(b'A')],
];
const DIR_KEYPAD: [&[Option<u8>]; 2] =
    [&[None, Some(b'^'), Some(b'A')], &[Some(b'<'), Some(b'v'), Some(b'>')]];

fn press(
    button: u8,
    mut x: u8,
    mut y: u8,
    layout: &[&[Option<u8>]],
) -> Option<(u8, u8, Option<u8>)> {
    match button {
        b'A' => return Some((x, y, Some(layout[y as usize][x as usize].unwrap()))),
        b'^' => y = y.wrapping_add_signed(-1),
        b'<' => x = x.wrapping_add_signed(-1),
        b'v' => y += 1,
        b'>' => x += 1,
        _ => unreachable!(),
    }

    layout.get(y as usize).and_then(|row| row.get(x as usize)).copied().flatten()?;

    Some((x, y, None))
}

pub fn solve_first(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let numeric = line[..3].parse::<usize>().unwrap();

            let target = line.as_bytes();

            let mut stack = VecDeque::new();
            stack.push_back((0, 0, (2_u8, 0_u8), (2_u8, 0_u8), (2_u8, 3_u8)));
            let mut visited = HashSet::new();
            visited.insert((0, (2, 0), (2, 0), (2, 3)));
            while let Some((steps, target_index, (x1, y1), (x2, y2), (x3, y3))) = stack.pop_front()
            {
                if target_index >= 4 {
                    println!("{line}: {steps} * {numeric}");
                    return steps * numeric;
                }

                for button in [b'^', b'<', b'v', b'>', b'A'] {
                    if let Some((nx1, ny1, button)) = press(button, x1, y1, &DIR_KEYPAD) {
                        if let Some(button) = button {
                            if let Some((nx2, ny2, button)) = press(button, x2, y2, &DIR_KEYPAD) {
                                if let Some(button) = button {
                                    if let Some((nx3, ny3, button)) =
                                        press(button, x3, y3, &NUM_KEYPAD)
                                    {
                                        if let Some(button) = button {
                                            if button == target[target_index]
                                                && visited.insert((
                                                    target_index + 1,
                                                    (nx1, ny1),
                                                    (nx2, ny2),
                                                    (nx3, ny3),
                                                ))
                                            {
                                                println!("{target_index}: {}", steps + 1);
                                                stack.push_back((
                                                    steps + 1,
                                                    target_index + 1,
                                                    (nx1, ny1),
                                                    (nx2, ny2),
                                                    (nx3, ny3),
                                                ));
                                            }
                                        } else if visited.insert((
                                            target_index,
                                            (nx1, ny1),
                                            (nx2, ny2),
                                            (nx3, ny3),
                                        )) {
                                            stack.push_back((
                                                steps + 1,
                                                target_index,
                                                (nx1, ny1),
                                                (nx2, ny2),
                                                (nx3, ny3),
                                            ));
                                        }
                                    }
                                } else if visited.insert((
                                    target_index,
                                    (nx1, ny1),
                                    (nx2, ny2),
                                    (x3, y3),
                                )) {
                                    stack.push_back((
                                        steps + 1,
                                        target_index,
                                        (nx1, ny1),
                                        (nx2, ny2),
                                        (x3, y3),
                                    ));
                                }
                            }
                        } else if visited.insert((target_index, (nx1, ny1), (x2, y2), (x3, y3))) {
                            stack.push_back((
                                steps + 1,
                                target_index,
                                (nx1, ny1),
                                (x2, y2),
                                (x3, y3),
                            ));
                        }
                    }
                }
            }

            unreachable!("no solution")
        })
        .sum()
}

pub fn solve_second<const PADS: usize>(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let numeric = line[..3].parse::<usize>().unwrap();

            let target = line.as_bytes();

            let mut stack = VecDeque::new();
            stack.push_back((
                0,
                0,
                std::array::from_fn(|i| if i == PADS - 1 { (2_u8, 3_u8) } else { (2_u8, 0_u8) }),
            ));
            let mut visited = HashSet::new();
            visited.insert((
                0,
                std::array::from_fn(|i| if i == PADS - 1 { (2_u8, 3_u8) } else { (2_u8, 0_u8) }),
            ));
            while let Some((steps, target_index, coords)) = stack.pop_front() {
                if target_index > 0 {
                    panic!()
                }
                if target_index >= 4 {
                    println!("{line}: {steps} * {numeric}");
                    return steps * numeric;
                }

                for button in [b'^', b'<', b'v', b'>', b'A'] {
                    let mut button = button;
                    let mut i = 0;
                    loop {
                        if let Some((nx, ny, b)) = press(
                            button,
                            coords[i].0,
                            coords[i].1,
                            if i == PADS - 1 { &NUM_KEYPAD } else { &DIR_KEYPAD },
                        ) {
                            if let Some(b) = b {
                                if i == PADS - 1 {
                                    if b == target[target_index]
                                        && visited.insert((target_index + 1, coords))
                                    {
                                        stack.push_back((steps + 1, target_index + 1, coords));
                                    }
                                    break;
                                } else {
                                    button = b;
                                    i += 1;
                                }
                            } else {
                                let mut coords: [(u8, u8); PADS] = coords;
                                coords[i].0 = nx;
                                coords[i].1 = ny;
                                if visited.insert((target_index, coords)) {
                                    stack.push_back((steps + 1, target_index, coords));
                                }
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }

            unreachable!("no solution")
        })
        .sum()
}

pub fn solve_second_dp<const PADS: usize>(input: &str) -> u64 {
    let mut lookup = [[[0_u64; 5]; 5]; PADS];
    for (y, x) in Itertools::cartesian_product(0..5, 0..5) {
        lookup[0][y][x] = 1;
    }

    // 0: A
    // 1: ^
    // 2: <
    // 3: v
    // 4: >

    let paths = [
        ["", "<", "v<<", "v<", "v"],
        [">", "", "v<", "v", "v>"],
        [">>^", ">^", "", ">", ">>"],
        [">^", "^", "<", "", ">"],
        ["^", "^<", "<<", "<", ""],
    ];

    for i in 1..PADS {
        for from in 0..5 {
            for to in 0..5 {
                let path = paths[from][to].chars().map(|c| match c {
                    '^' => 1,
                    '<' => 2,
                    'v' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                });
                let path2 = paths[from][to].chars().rev().map(|c| match c {
                    '^' => 1,
                    '<' => 2,
                    'v' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                });
                if ((from == 0 || from == 1) && to == 2) || (from == 2 && (to == 0 || to == 1)) {
                    lookup[i][from][to] = once(0)
                        .chain(path)
                        .chain(once(0))
                        .tuple_windows()
                        .map(|(from, to)| lookup[i - 1][from][to])
                        .sum();
                } else {
                    lookup[i][from][to] = min(
                        once(0)
                            .chain(path)
                            .chain(once(0))
                            .tuple_windows()
                            .map(|(from, to)| lookup[i - 1][from][to])
                            .sum(),
                        once(0)
                            .chain(path2)
                            .chain(once(0))
                            .tuple_windows()
                            .map(|(from, to)| lookup[i - 1][from][to])
                            .sum(),
                    );
                }
            }
        }
        // lookup[i][0][0] = lookup[i - 1][0][0];
        // lookup[i][0][1] = lookup[i - 1][0][2] + lookup[i - 1][2][0];
        // lookup[i][0][2] =
        //     lookup[i - 1][0][3] + lookup[i - 1][3][2] + lookup[i - 1][2][2] + lookup[i - 1][2][0];
        // lookup[i][0][3] = lookup[i - 1][0][3] + lookup[i - 1][3][2] + lookup[i - 1][2][0];
        // lookup[i][0][4] = lookup[i - 1][0][3] + lookup[i - 1][3][0];
        //
        // lookup[i][1][0] = lookup[i - 1][0][4] + lookup[i - 1][4][0];
        // lookup[i][1][1] = lookup[i - 1][0][0];
        // lookup[i][1][2] = lookup[i - 1][0][3] + lookup[i - 1][3][2] + lookup[i - 1][2][0];
        // lookup[i][1][3] = lookup[i - 1][0][3] + lookup[i - 1][3][0];
        // lookup[i][1][4] = lookup[i - 1][0][3] + lookup[i - 1][3][4] + lookup[i - 1][4][0];
    }

    // TODO the order of operations is important?
    let paths2 = [
        ["", "<", "^<<", "^<", "^", "^^<<", "<^^", "^^", "^^^<<", "^^^<", "^^^"],
        [">", "", "^<", "^", "^>", "^^<", "^^", "^^>", "^^^<", "^^^", "^^^>"],
        [">>v", ">v", "", ">", ">>", "^", "^>", "^>>", "^^", "^^>", "^^>>"],
        [">v", "v", "<", "", ">", "^<", "^", "^>", "^^<", "^^", "^^>"],
        ["v", "v<", "<<", "<", "", "^<<", "^<", "^", "^^<<", "^^<", "^^"],
        [">>vv", ">vv", "v", "v>", "v>>", "", ">", ">>", "^", "^>", "^>>"],
        [">vv", "vv", "v<", "v", "v>", "<", "", ">", "^<", "^", "^>"],
        ["vv", "vv<", "v<<", "v<", "v", "<<", "<", "", "<<^", "^<", "^"],
        [">>vvv", ">vvv", "vv", ">vv", ">>vv", "v", ">v", ">>v", "", ">", ">>"],
        [">vvv", "vvv", "vv<", "vv", "vv>", "v<", "v", "v>", "<", "", ">"],
        ["vvv", "vvv<", "vv<<", "vv<", "vv", "v<<", "v<", "v", "<<", "<", ""],
    ];
    let mut lookup2 = [[0_u64; 11]; 11];
    for from in 0..11 {
        for to in 0..11 {
            let path = paths2[from][to].chars().map(|c| match c {
                '^' => 1,
                '<' => 2,
                'v' => 3,
                '>' => 4,
                _ => unreachable!(),
            });
            let path2 = paths2[from][to].chars().rev().map(|c| match c {
                '^' => 1,
                '<' => 2,
                'v' => 3,
                '>' => 4,
                _ => unreachable!(),
            });
            if ((from == 0 || from == 1) && (to == 2 || to == 5 || to == 8))
                || ((from == 2 || from == 5 || from == 8) && (to == 0 || to == 1))
            {
                lookup2[from][to] = once(0)
                    .chain(path)
                    .chain(once(0))
                    .tuple_windows()
                    .map(|(from, to)| lookup[PADS - 1][from][to])
                    .sum();
            } else {
                lookup2[from][to] = min(
                    once(0)
                        .chain(path)
                        .chain(once(0))
                        .tuple_windows()
                        .map(|(from, to)| lookup[PADS - 1][from][to])
                        .sum(),
                    once(0)
                        .chain(path2)
                        .chain(once(0))
                        .tuple_windows()
                        .map(|(from, to)| lookup[PADS - 1][from][to])
                        .sum(),
                );
            }
        }
    }

    println!("{lookup2:?}");

    input
        .lines()
        .map(|line| {
            let numeric = line[..3].parse::<u64>().unwrap();

            println!(
                "{:?}",
                once(0)
                    .chain(line.chars().map(|c| match c {
                        'A' => 0,
                        c => c.to_digit(10).unwrap() as usize + 1,
                    }))
                    .tuple_windows::<(_, _)>()
                    .map(|(from, to)| lookup2[from][to])
                    .collect::<Vec<_>>()
            );
            println!(
                "{}",
                once(0)
                    .chain(line.chars().map(|c| match c {
                        'A' => 0,
                        c => c.to_digit(10).unwrap() as usize + 1,
                    }))
                    .tuple_windows()
                    .map(|(from, to)| lookup2[from][to])
                    .sum::<u64>()
            );

            numeric
                * once(0)
                    .chain(line.chars().map(|c| match c {
                        'A' => 0,
                        c => c.to_digit(10).unwrap() as usize + 1,
                    }))
                    .tuple_windows()
                    .map(|(from, to)| lookup2[from][to])
                    .sum::<u64>()
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(126384, solve_first(sample));
    assert_eq!(126384, solve_second_dp::<3>(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(197560, solve_first(input));
    assert_eq!(197560, solve_second_dp::<3>(input));
    assert_eq!(242337182910752, solve_second_dp::<26>(input));
}

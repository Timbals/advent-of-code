use itertools::Itertools;
use std::cmp::max;
use std::iter::once;

pub fn solve(input: &str, end: usize) -> usize {
    let mut movements = input
        .trim()
        .bytes()
        .map(|x| match x {
            b'>' => (1, 0),
            b'<' => (-1, 0),
            _ => unreachable!(),
        })
        .cycle()
        .intersperse((0, -1));

    let mut rocks = [
        [(0, 0), (1, 0), (2, 0), (3, 0)].as_slice(),
        [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].as_slice(),
        [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].as_slice(),
        [(0, 0), (0, 1), (0, 2), (0, 3)].as_slice(),
        [(0, 0), (1, 0), (0, 1), (1, 1)].as_slice(),
    ]
    .into_iter()
    .cycle();

    let mut tallest = -1;
    let mut wait_for_tallest = None;
    let mut cycle_step = 0;
    let mut previous_cycle_i = 0;
    let mut board = vec![[false; 7]; 3 + 4];
    let mut additional = 0;

    let mut i = 0;
    while i < end {
        if wait_for_tallest.is_none() && tallest > 10000 {
            for cycle_length in 10..5000 {
                if board[tallest as usize - cycle_length..tallest as usize]
                    == board[tallest as usize - cycle_length - cycle_length
                        ..tallest as usize - cycle_length]
                {
                    println!("found cycle with length {} at {}", cycle_length, i);
                    wait_for_tallest = Some(tallest as usize + cycle_length);
                    cycle_step = cycle_length;
                    previous_cycle_i = i;
                }
            }
        }
        if additional == 0 && Some(tallest as usize) == wait_for_tallest {
            // TODO fix repeating `tallest` at cycle border
            //  sample needs +2 and input needs +1 because `tallest` repeats at cycle border
            let cycle_length = i - previous_cycle_i + 1;
            additional = cycle_step * ((end - i) / cycle_length);
            println!(
                "{} {} {} {} {} {} {}",
                previous_cycle_i,
                i,
                (end - i) / cycle_length,
                (end - i) % cycle_length,
                cycle_length,
                cycle_step,
                additional,
            );
            i += ((end - i) / cycle_length) * cycle_length;
        }

        let rock = rocks.next().unwrap();
        let (mut x, mut y) = (2, tallest + 4);
        board.extend(
            once([false; 7])
                .cycle()
                .take((tallest + 8) as usize - board.len()),
        );

        loop {
            let (dx, dy) = movements.next().unwrap();
            x += dx;
            y += dy;

            if y < 0
                || rock.iter().any(|(dx, dy)| {
                    *board[(y + dy) as usize]
                        .get((x + dx) as usize)
                        .unwrap_or(&true)
                })
            {
                x -= dx;
                y -= dy;

                if dy < 0 {
                    for (dx, dy) in rock {
                        board[(y + dy) as usize][(x + dx) as usize] = true;
                        tallest = max(tallest, y + dy);
                    }
                    break;
                }
            }
        }

        i += 1;
    }

    (tallest + 1) as usize + additional
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(3068, solve(sample, 2022));
    assert_eq!(1514285714288, solve(sample, 1000000000000_usize));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(3151, solve(input, 2022));
    assert_eq!(1560919540245, solve(input, 1000000000000_usize));
}

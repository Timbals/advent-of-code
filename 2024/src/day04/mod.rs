use itertools::chain;
use std::iter::once;

pub fn solve_first(input: &str) -> usize {
    let mut result = 0;

    // horizontal
    result += input.matches("XMAS").count();
    // horizontal backwards
    result += input.matches("SAMX").count();

    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();

    let lines_ref = &lines;
    let columns = (0..width).flat_map(|x| {
        (0..height).map(move |y| lines_ref[y].as_bytes()[x] as char).chain(once(' '))
    });
    let columns = columns.collect::<String>();

    // vertical
    result += columns.matches("XMAS").count();
    // vertical backwards
    result += columns.matches("SAMX").count();

    let diagonals_x_positive = (0..width).flat_map(|x| {
        let mut i = 0;
        std::iter::from_fn(move || {
            let y = i;
            let x = x + i;
            if x < width && y < height {
                i += 1;
                Some(lines_ref[y].as_bytes()[x] as char)
            } else {
                None
            }
        })
        .chain(once(' '))
    });
    let diagonals_y_positive = (1..height).flat_map(|y| {
        let mut i = 0;
        std::iter::from_fn(move || {
            let x = i;
            let y = y + i;
            if x < width && y < height {
                i += 1;
                Some(lines_ref[y].as_bytes()[x] as char)
            } else {
                None
            }
        })
        .chain(once(' '))
    });
    let diagonals_x_negative = (0..width).flat_map(|x| {
        let mut i = 0;
        std::iter::from_fn(move || {
            let y = i;
            let x = x as isize - i;
            if x >= 0 && y < height as isize {
                i += 1;
                Some(lines_ref[y as usize].as_bytes()[x as usize] as char)
            } else {
                None
            }
        })
        .chain(once(' '))
    });
    let diagonals_y_negative = (1..height).flat_map(|y| {
        let mut i = 0;
        std::iter::from_fn(move || {
            let x = width as isize - i - 1;
            let y = y as isize + i;
            if x >= 0 && y < height as isize {
                i += 1;
                Some(lines_ref[y as usize].as_bytes()[x as usize] as char)
            } else {
                None
            }
        })
        .chain(once(' '))
    });

    let diagonals = chain!(
        diagonals_x_positive,
        diagonals_y_positive,
        diagonals_x_negative,
        diagonals_y_negative
    )
    .collect::<String>();
    result += diagonals.matches("XMAS").count();
    result += diagonals.matches("SAMX").count();

    result
}

pub fn solve_second(input: &str) -> usize {
    let mut result = 0;

    let lines = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();

    let diff = b'M'.abs_diff(b'S');

    for y in 0..(height - 2) {
        for x in 0..(width - 2) {
            let top_left = lines[y][x];
            let top_right = lines[y][x + 2];
            let bottom_left = lines[y + 2][x];
            let bottom_right = lines[y + 2][x + 2];
            let middle = lines[y + 1][x + 1];
            if (top_left == b'M' || top_left == b'S')
                && (top_right == b'M' || top_right == b'S')
                && (bottom_left == b'M' || bottom_left == b'S')
                && (bottom_right == b'M' || bottom_right == b'S')
                && middle == b'A'
                && top_left.abs_diff(bottom_right) == diff
                && top_right.abs_diff(bottom_left) == diff
            {
                result += 1;
            }
        }
    }

    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(18, solve_first(sample));
    assert_eq!(9, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2336, solve_first(input));
    assert_eq!(1831, solve_second(input));
}

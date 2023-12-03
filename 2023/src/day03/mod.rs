use itertools::iproduct;
use std::collections::HashMap;

pub fn solve_first(input: &str) -> u32 {
    let mut sum = 0;

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (y, line) in input.lines().enumerate() {
        for number in line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|m| !m.is_empty())
        {
            let x_start = number.as_ptr() as usize - line.as_ptr() as usize;
            let x_end = x_start + number.len();

            if iproduct!(
                y.saturating_add_signed(-1)..=(y + 1),
                x_start.saturating_add_signed(-1)..=x_end
            )
            .map(|(y, x)| grid.get(y).and_then(|line| line.get(x)).unwrap_or(&'.'))
            .any(|c| *c != '.' && !c.is_ascii_digit())
            {
                sum += number.parse::<u32>().unwrap();
            }
        }
    }

    sum
}

pub fn solve_second(input: &str) -> u32 {
    let mut sum = 0;

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (y, line) in input.lines().enumerate() {
        for number in line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|m| !m.is_empty())
        {
            let x_start = number.as_ptr() as usize - line.as_ptr() as usize;
            let x_end = x_start + number.len();

            for (y, x) in iproduct!(
                y.saturating_add_signed(-1)..=(y + 1),
                x_start.saturating_add_signed(-1)..=x_end
            ) {
                if *grid.get(y).and_then(|line| line.get(x)).unwrap_or(&'.') == '*' {
                    let entry = gears.entry((x, y)).or_default();
                    entry.push(number.parse::<u32>().unwrap());
                }
            }
        }
    }

    for (_, numbers) in gears {
        if let [first, second] = numbers.as_slice() {
            sum += first * second
        }
    }

    sum
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(4361, solve_first(sample));
    assert_eq!(467835, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(543867, solve_first(input));
    assert_eq!(79613331, solve_second(input));
}

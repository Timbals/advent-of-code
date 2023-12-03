use std::collections::HashMap;

pub fn solve_first(input: &str) -> u32 {
    let mut sum = 0;

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    input.lines().enumerate().for_each(|(y, line)| {
        for number in line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|m| !m.is_empty())
        {
            let x_start = number.as_ptr() as usize - line.as_ptr() as usize;
            let x_end = x_start + number.len() - 1;

            let mut symbol = false;
            for y in y.saturating_add_signed(-1)..=(y + 1) {
                for x in x_start.saturating_add_signed(-1)..=(x_end + 1) {
                    let c = grid.get(y).and_then(|line| line.get(x)).unwrap_or(&'.');
                    if *c != '.' && !c.is_ascii_digit() {
                        symbol = true;
                    }
                }
            }

            if symbol {
                sum += number.parse::<u32>().unwrap();
            }
        }
    });

    sum
}

pub fn solve_second(input: &str) -> u32 {
    let mut sum = 0;

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    input.lines().enumerate().for_each(|(y, line)| {
        for number in line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|m| !m.is_empty())
        {
            let x_start = number.as_ptr() as usize - line.as_ptr() as usize;
            let x_end = x_start + number.len() - 1;

            for y in y.saturating_add_signed(-1)..=(y + 1) {
                for x in x_start.saturating_add_signed(-1)..=(x_end + 1) {
                    let c = grid.get(y).and_then(|line| line.get(x)).unwrap_or(&'.');
                    if *c == '*' {
                        let entry = gears.entry((x, y)).or_default();
                        entry.push(number.parse::<u32>().unwrap());
                    }
                }
            }
        }
    });

    for (_, numbers) in gears {
        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1]
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

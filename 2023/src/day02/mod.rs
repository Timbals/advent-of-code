use std::cmp::max;

pub fn solve_first(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let id = left[5..].parse::<u32>().unwrap();
            let samples = right.split("; ");
            let mut samples = samples.map(|sample| {
                sample
                    .split(", ")
                    .map(|cubes| {
                        let (number, color) = cubes.split_once(' ').unwrap();
                        let number = number.parse::<u32>().unwrap();
                        (number, color)
                    })
                    .all(|(number, color)| match color {
                        "red" => number <= 12,
                        "green" => number <= 13,
                        "blue" => number <= 14,
                        _ => unreachable!(),
                    })
            });
            let possible = samples.all(|x| x);

            possible.then_some(id)
        })
        .sum()
}

pub fn solve_second(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (mut red, mut green, mut blue) = (0, 0, 0);

            let (_, right) = line.split_once(": ").unwrap();
            let samples = right.split("; ");
            samples.for_each(|sample| {
                for (number, color) in sample.split(", ").map(|cubes| {
                    let (number, color) = cubes.split_once(' ').unwrap();
                    let number = number.parse::<u32>().unwrap();
                    (number, color)
                }) {
                    match color {
                        "red" => red = max(red, number),
                        "green" => green = max(green, number),
                        "blue" => blue = max(blue, number),
                        _ => unreachable!(),
                    }
                }
            });

            red * green * blue
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(8, solve_first(sample));
    assert_eq!(2286, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2105, solve_first(input));
    assert_eq!(72422, solve_second(input));
}

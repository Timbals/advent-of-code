use itertools::Itertools;

fn parse_samples(samples: &str) -> impl Iterator<Item = (&str, u32)> {
    let samples = samples.split("; ").flat_map(|sample| sample.split(", "));
    samples.map(|sample| {
        let (number, color) = sample.split_once(' ').unwrap();
        let number = number.parse::<u32>().unwrap();
        (color, number)
    })
}

pub fn solve_first(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let (id, samples) = line.split_once(": ").unwrap();
            let id = id[5..].parse::<u32>().unwrap();
            parse_samples(samples)
                .all(|(color, number)| match color {
                    "red" => number <= 12,
                    "green" => number <= 13,
                    "blue" => number <= 14,
                    _ => unreachable!(),
                })
                .then_some(id)
        })
        .sum()
}

pub fn solve_second(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, samples) = line.split_once(": ").unwrap();
            parse_samples(samples)
                .into_grouping_map()
                .max()
                .values()
                .product::<u32>()
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

use itertools::Itertools;

fn safe(report: impl IntoIterator<Item = usize>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for (a, b) in report.into_iter().tuple_windows() {
        increasing &= a < b && (1..=3).contains(&(b - a));
        decreasing &= a > b && (1..=3).contains(&(a - b));
    }
    increasing || decreasing
}

pub fn solve_first(input: &str) -> usize {
    input
        .lines()
        .filter(|report| safe(report.split_whitespace().map(|x| x.parse::<usize>().unwrap())))
        .count()
}

pub fn solve_second(input: &str) -> usize {
    input
        .lines()
        .filter(|report| {
            let levels =
                report.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
            (0..=levels.len()).any(|skip| {
                safe(levels.iter().enumerate().filter(|&(i, _)| skip != i).map(|(_, x)| *x))
            })
        })
        .count()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(2, solve_first(sample));
    assert_eq!(4, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(432, solve_first(input));
    assert_eq!(488, solve_second(input));
}

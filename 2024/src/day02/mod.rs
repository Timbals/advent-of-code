use itertools::Itertools;

pub fn solve_first(input: &str) -> u32 {
    input
        .lines()
        .filter(|report| {
            let mut levels =
                report.split_whitespace().map(|x| x.parse::<u32>().unwrap()).tuple_windows();

            (levels.clone().all(|(a, b)| a < b) || levels.clone().all(|(a, b)| a > b))
                && levels.all(|(a, b)| (1..=3).contains(&a.abs_diff(b)))
        })
        .count() as u32
}

pub fn solve_second(input: &str) -> u32 {
    input
        .lines()
        .filter(|report| {
            let levels =
                report.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

            let mut safe = false;
            for i in 0..levels.len() {
                let mut levels = levels.clone();
                levels.remove(i);
                safe |= (levels.iter().tuple_windows().all(|(a, b)| a < b)
                    || levels.iter().tuple_windows().all(|(a, b)| a > b))
                    && levels
                        .iter()
                        .tuple_windows()
                        .all(|(a, b)| (1..=3).contains(&a.abs_diff(*b)));
            }

            safe |= (levels.iter().tuple_windows().all(|(a, b)| a < b)
                || levels.iter().tuple_windows().all(|(a, b)| a > b))
                && levels.iter().tuple_windows().all(|(a, b)| (1..=3).contains(&a.abs_diff(*b)));

            safe
        })
        .count() as u32
}

#[test]
pub fn sample() {
    assert_eq!(2, solve_first(include_str!("sample.txt")));
    assert_eq!(4, solve_second(include_str!("sample.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(432, solve_first(input));
    assert_eq!(488, solve_second(input));
}

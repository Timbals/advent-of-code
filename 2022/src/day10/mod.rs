use itertools::Itertools;
use std::iter::once;

pub fn parse(input: &str) -> impl Iterator<Item = (usize, isize)> + '_ {
    input
        .lines()
        .flat_map(str::split_whitespace)
        .scan(1, |x, instruction| {
            let result = Some(*x);
            match instruction {
                "addx" => {}
                "noop" => {}
                v => *x += v.parse::<isize>().unwrap(),
            }
            result
        })
        .enumerate()
}

pub fn solve_first(input: &str) -> isize {
    parse(input)
        .filter(|(cycle, _)| (*cycle as isize + 1 - 20) % 40 == 0)
        .map(|(cycle, x)| (cycle + 1) as isize * x)
        .sum()
}

pub fn solve_second(input: &str) -> String {
    parse(input)
        .map(|(cycle, x)| ((cycle % 40) as isize, x))
        .chunks(40)
        .into_iter()
        .flat_map(|c| once('\n').chain(c.map(|(p, x)| if (p - x).abs() <= 1 { '#' } else { '.' })))
        .collect()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(13140, solve_first(sample));
    assert_eq!(
        "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
        solve_second(sample)
    );
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(11220, solve_first(input));
    // BZPAJELK
    assert_eq!(
        "
###..####.###...##....##.####.#....#..#.
#..#....#.#..#.#..#....#.#....#....#.#..
###....#..#..#.#..#....#.###..#....##...
#..#..#...###..####....#.#....#....#.#..
#..#.#....#....#..#.#..#.#....#....#.#..
###..####.#....#..#..##..####.####.#..#.",
        solve_second(input)
    );
}

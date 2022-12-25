const DIGITS: [(isize, char); 5] = [(0, '0'), (1, '1'), (2, '2'), (-1, '-'), (-2, '=')];
const BASE: isize = DIGITS.len() as isize;

fn from_snafu(number: &str) -> isize {
    number
        .chars()
        .map(|v| DIGITS.iter().find(|(_, c)| v == *c).unwrap().0)
        .rev()
        .enumerate()
        .map(|(i, v)| BASE.pow(i as _) * v)
        .sum()
}

fn to_snafu(mut number: isize) -> String {
    let mut out = Vec::new();
    while number != 0 {
        let digit = DIGITS
            .iter()
            .find(|digit| number.rem_euclid(BASE) == digit.0.rem_euclid(BASE))
            .unwrap();
        number -= digit.0;
        out.push(digit.1);
        number /= BASE;
    }
    out.into_iter().rev().collect()
}

pub fn solve(input: &str) -> String {
    to_snafu(input.lines().map(from_snafu).sum())
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!("2=-1=0", solve(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!("2-==10--=-0101==1201", solve(input));
}

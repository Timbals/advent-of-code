use regex::Regex;

pub fn solve(input: &str, enabler: bool) -> usize {
    let mut result = 0;
    let mut enabled = true;

    let re = Regex::new("mul\\(([0-9]+),([0-9]+)\\)|do\\(\\)|don't\\(\\)").unwrap();
    for captures in re.captures_iter(input) {
        if enabled && captures.get(1).is_some() {
            result += captures.get(1).unwrap().as_str().parse::<usize>().unwrap()
                * captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        } else if enabler && captures.get(0).unwrap().as_str() == "do()" {
            enabled = true;
        } else if enabler && captures.get(0).unwrap().as_str() == "don't()" {
            enabled = false;
        }
    }

    result
}

#[test]
pub fn sample() {
    assert_eq!(161, solve(include_str!("sample1.txt"), false));
    assert_eq!(48, solve(include_str!("sample2.txt"), true));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(160672468, solve(input, false));
    assert_eq!(84893551, solve(input, true));
}

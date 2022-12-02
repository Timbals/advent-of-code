fn solve_first(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace().map(|x| x.chars().next().unwrap());
            let theirs = line.next().unwrap() as i32 - 'A' as i32;
            let ours = line.next().unwrap() as i32 - 'X' as i32;
            let diff = (ours - theirs).rem_euclid(3);

            let round_score = (diff + 1).rem_euclid(3) * 3;
            let selection_score = ours + 1;

            round_score + selection_score
        })
        .sum()
}

fn solve_second(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace().map(|x| x.chars().next().unwrap());
            let theirs = line.next().unwrap() as i32 - 'A' as i32;
            let ours = (theirs + line.next().unwrap() as i32 - 'X' as i32 - 1).rem_euclid(3);
            let diff = (ours - theirs).rem_euclid(3);

            let round_score = (diff + 1).rem_euclid(3) * 3;
            let selection_score = ours + 1;

            round_score + selection_score
        })
        .sum()
}

#[test]
pub fn day02_sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(15, solve_first(sample));
    assert_eq!(12, solve_second(sample));
}

#[test]
pub fn day02() {
    let input = include_str!("input.txt");
    assert_eq!(12535, solve_first(input));
    assert_eq!(15457, solve_second(input));
}

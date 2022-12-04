fn parse(input: &str) -> impl Iterator<Item = [(usize, usize); 2]> + '_ {
    input.lines().map(|line| {
        let mut line = line.split(',');
        std::array::from_fn(|_| {
            let mut range = line
                .next()
                .unwrap()
                .split('-')
                .map(|x| x.parse::<usize>().unwrap());
            (range.next().unwrap(), range.next().unwrap())
        })
    })
}

fn solve_first(input: &str) -> usize {
    parse(input)
        .filter(|[(l1, r1), (l2, r2)]| (l1 <= l2 && r2 <= r1) || (l2 <= l1 && r1 <= r2))
        .count()
}

fn solve_second(input: &str) -> usize {
    parse(input)
        .filter(|[(l1, r1), (l2, r2)]| {
            (l2 <= l1 && l1 <= r2)
                || (l2 <= r1 && r1 <= r2)
                || (l1 <= l2 && l2 <= r1)
                || (l1 <= r2 && r2 <= r1)
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
    assert_eq!(595, solve_first(input));
    assert_eq!(952, solve_second(input));
}

use itertools::Itertools;

pub fn solve_first(input: &str) -> u32 {
    let mut result = 0;
    for (i, _) in input.match_indices("mul(") {
        let input = &input[(i + 4)..];
        let mut input = input.chars().peekable();
        if let Some(x) = input.next().and_then(|x| x.to_digit(10)) {
            let mut first = x;
            if let Some(x) = input.peek().and_then(|x| x.to_digit(10)) {
                input.next();
                first *= 10;
                first += x;

                if let Some(x) = input.peek().and_then(|x| x.to_digit(10)) {
                    input.next();
                    first *= 10;
                    first += x;
                }
            }

            if input.next() != Some(',') {
                continue;
            }

            if let Some(x) = input.next().and_then(|x| x.to_digit(10)) {
                let mut second = x;
                if let Some(x) = input.peek().and_then(|x| x.to_digit(10)) {
                    input.next();
                    second *= 10;
                    second += x;

                    if let Some(x) = input.peek().and_then(|x| x.to_digit(10)) {
                        input.next();
                        second *= 10;
                        second += x;
                    }
                }

                if input.next() != Some(')') {
                    continue;
                }

                result += first * second;
            }
        }
    }
    result
}

pub fn solve_second(input: &str) -> u32 {
    let mut enabled = true;
    let enables = input.match_indices("do()");
    let disables = input.match_indices("don't()");
    let mut enables_disables = enables.merge_by(disables, |(a, _), (b, _)| a < b).peekable();

    let mut result = 0;
    for (i, _) in input.match_indices("mul(") {
        while enables_disables.peek().map(|(j, _)| *j).unwrap_or(usize::MAX) < i {
            let (_, action) = enables_disables.next().unwrap();
            print!("{action}");
            match action {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => unreachable!(),
            }
        }

        if !enabled {
            continue;
        }

        let input = &input[(i + 4)..];
        let mut input = input.chars().peekable();
        if let Some(x) = input.next().and_then(|x| x.to_digit(10)) {
            let mut first = x;
            if let Some(x) = input.peek().and_then(|x| x.to_digit(10)) {
                input.next();
                first *= 10;
                first += x;

                if let Some(x) = input.peek().and_then(|x| x.to_digit(10)) {
                    input.next();
                    first *= 10;
                    first += x;
                }
            }

            if input.next() != Some(',') {
                continue;
            }

            if let Some(x) = input.next().and_then(|x| x.to_digit(10)) {
                let mut second = x;
                if let Some(x) = input.peek().and_then(|x| x.to_digit(10)) {
                    input.next();
                    second *= 10;
                    second += x;

                    if let Some(x) = input.peek().and_then(|x| x.to_digit(10)) {
                        input.next();
                        second *= 10;
                        second += x;
                    }
                }

                if input.next() != Some(')') {
                    continue;
                }

                result += first * second;
            }
        }
    }
    result
}

#[test]
pub fn sample() {
    assert_eq!(161, solve_first(include_str!("sample1.txt")));
    assert_eq!(48, solve_second(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(160672468, solve_first(input));
    assert_eq!(84893551, solve_second(input));
}

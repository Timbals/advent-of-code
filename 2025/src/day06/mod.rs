pub fn solve_first(input: &str) -> usize {
    let mut total = 0;
    let mut problems =
        vec![Vec::<usize>::new(); input.lines().next().unwrap().split_whitespace().count()];
    for line in input.lines() {
        for (i, x) in line.split_whitespace().enumerate() {
            match x.parse::<usize>() {
                Ok(x) => {
                    problems[i].push(x);
                }
                Err(_) if x == "*" => {
                    total += std::mem::take(&mut problems[i]).into_iter().product::<usize>();
                }
                Err(_) if x == "+" => {
                    total += std::mem::take(&mut problems[i]).into_iter().sum::<usize>();
                }
                _ => unreachable!(),
            }
        }
    }
    total
}

pub fn solve_second(input: &str) -> usize {
    let mut total = 0;

    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let width = grid.iter().map(|line| line.len()).max().unwrap_or_default();

    let mut sum = 0;
    let mut product = 1;
    for x in (0..width).rev() {
        let mut number = 0;
        for line in &grid {
            if let Some(digit) = line.get(x)
                && digit.is_ascii_digit()
            {
                number = number * 10 + (digit - b'0') as usize;
            }
        }
        if number != 0 {
            sum += number;
            product *= number;
        }

        match grid[grid.len() - 1].get(x) {
            Some(b'*') => {
                total += product;
                sum = 0;
                product = 1;
            }
            Some(b'+') => {
                total += sum;
                sum = 0;
                product = 1;
            }
            _ => {}
        }
    }

    total
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(4277556, solve_first(sample));
    assert_eq!(3263827, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(5552221122013, solve_first(input));
    assert_eq!(11371597126232, solve_second(input));
}

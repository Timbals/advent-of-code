pub fn solve_first(input: &str) -> usize {
    let mut dial = 50;
    let mut count = 0;

    for line in input.lines() {
        let (direction, amount) = line.split_at(1);

        let amount: isize = amount.parse().unwrap();
        let operation = match direction {
            "L" => isize::strict_sub,
            "R" => isize::strict_add,
            _ => unreachable!(),
        };

        dial = operation(dial, amount).rem_euclid(100);

        if dial == 0 {
            count += 1;
        }
    }

    count
}

pub fn solve_second(input: &str) -> usize {
    let mut dial = 50;
    let mut count = 0;

    for line in input.lines() {
        let (direction, amount) = line.split_at(1);

        let mut amount: isize = amount.parse().unwrap();
        let operation = match direction {
            "L" => isize::strict_sub,
            "R" => isize::strict_add,
            _ => unreachable!(),
        };

        while amount > 0 {
            amount -= 1;
            dial = operation(dial, 1).rem_euclid(100);
            if dial == 0 {
                count += 1;
            }
        }
    }

    count
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(3, solve_first(sample));
    assert_eq!(6, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1165, solve_first(input));
    assert_eq!(6496, solve_second(input));
}

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

        let amount: isize = amount.parse().unwrap();
        let operation = match direction {
            "L" => isize::strict_sub,
            "R" => isize::strict_add,
            _ => unreachable!(),
        };

        let new_dial = operation(dial, amount);
        count += (new_dial / 100).unsigned_abs() + usize::from(new_dial <= 0 && dial != 0);
        dial = new_dial.rem_euclid(100);
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

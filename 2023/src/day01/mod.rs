pub fn solve_first(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last = line
                .chars()
                .rfind(|c| c.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap();
            first * 10 + last
        })
        .sum()
}

pub fn solve_second(input: &str) -> u32 {
    let numbers = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    input
        .lines()
        .map(|line| {
            let mut best_index = line.find(char::is_numeric).unwrap_or(usize::MAX);
            let mut first = line
                .chars()
                .nth(best_index)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap_or(0);

            for (s, i) in numbers {
                if let Some(index) = line.find(s) {
                    if index < best_index {
                        best_index = index;
                        first = i;
                    }
                }
            }

            let mut best_index = line.rfind(char::is_numeric).unwrap_or(0);
            let mut last = line
                .chars()
                .nth(best_index)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap_or(0);

            for (s, i) in numbers {
                if let Some(index) = line.rfind(s) {
                    if index > best_index {
                        best_index = index;
                        last = i;
                    }
                }
            }

            first * 10 + last
        })
        .sum()
}

#[test]
pub fn sample() {
    assert_eq!(142, solve_first(include_str!("sample1.txt")));
    assert_eq!(281, solve_second(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(56049, solve_first(input));
    assert_eq!(54530, solve_second(input));
}

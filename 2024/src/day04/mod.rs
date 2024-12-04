pub fn solve_first(input: &str) -> usize {
    let lines = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();

    let mut result = 0;

    let steps = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    for y in 0..height {
        for x in 0..width {
            if lines.get(y).and_then(|row| row.get(x)) == Some(&b'X') {
                for (dx, dy) in steps {
                    if lines
                        .get(y.wrapping_add_signed(dy))
                        .and_then(|row| row.get(x.wrapping_add_signed(dx)))
                        == Some(&b'M')
                        && lines
                            .get(y.wrapping_add_signed(dy * 2))
                            .and_then(|row| row.get(x.wrapping_add_signed(dx * 2)))
                            == Some(&b'A')
                        && lines
                            .get(y.wrapping_add_signed(dy * 3))
                            .and_then(|row| row.get(x.wrapping_add_signed(dx * 3)))
                            == Some(&b'S')
                    {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

pub fn solve_second(input: &str) -> usize {
    let lines = input.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();

    let mut result = 0;

    for y in 0..(height - 2) {
        for x in 0..(width - 2) {
            let top_left = lines[y][x];
            let top_right = lines[y][x + 2];
            let bottom_left = lines[y + 2][x];
            let bottom_right = lines[y + 2][x + 2];
            let middle = lines[y + 1][x + 1];
            if let (b'M' | b'S', b'M' | b'S', b'M' | b'S', b'M' | b'S', b'A') =
                (top_left, top_right, bottom_left, bottom_right, middle)
            {
                if top_left != bottom_right && top_right != bottom_left {
                    result += 1;
                }
            }
        }
    }

    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(18, solve_first(sample));
    assert_eq!(9, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(2336, solve_first(input));
    assert_eq!(1831, solve_second(input));
}

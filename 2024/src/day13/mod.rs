pub fn solve(input: &str, offset: isize) -> isize {
    let mut lines = std::iter::once("").chain(input.lines());
    let mut result = 0;

    while lines.next().is_some() {
        let (ax, ay) = lines.next().unwrap()[12..].split_once(", Y+").unwrap();
        let (ax, ay) = (ax.parse::<isize>().unwrap(), ay.parse::<isize>().unwrap());

        let (bx, by) = lines.next().unwrap()[12..].split_once(", Y+").unwrap();
        let (bx, by) = (bx.parse::<isize>().unwrap(), by.parse::<isize>().unwrap());

        let (px, py) = lines.next().unwrap()[9..].split_once(", Y=").unwrap();
        let (px, py) = (px.parse::<isize>().unwrap(), py.parse::<isize>().unwrap());
        let (px, py) = (px + offset, py + offset);

        // | ax bx | * | a | = | px |
        // | ay by |   | b |   | py |
        // 2x2 matrix inverse:
        let d = ax * by - bx * ay;
        let a = by * px + -bx * py;
        let b = -ay * px + ax * py;

        if a % d == 0 && b % d == 0 {
            result += a / d * 3 + b / d;
        }
    }

    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(480, solve(sample, 0));
    assert_eq!(875318608908, solve(sample, 10000000000000));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(36954, solve(input, 0));
    assert_eq!(79352015273424, solve(input, 10000000000000));
}

use itertools::Itertools;

pub fn solve_first(input: &str) -> usize {
    let mut lines = input.lines();
    let mut result = 0;

    loop {
        let (ax, ay) = lines.next().unwrap().split_once(", Y+").unwrap();
        let (_, ax) = ax.split_once(" X+").unwrap();
        let (ax, ay) = (ax.parse::<usize>().unwrap(), ay.parse::<usize>().unwrap());

        let (bx, by) = lines.next().unwrap().split_once(", Y+").unwrap();
        let (_, bx) = bx.split_once(" X+").unwrap();
        let (bx, by) = (bx.parse::<usize>().unwrap(), by.parse::<usize>().unwrap());

        let (px, py) = lines.next().unwrap().split_once(", Y=").unwrap();
        let (_, px) = px.split_once(" X=").unwrap();
        let (px, py) = (px.parse::<usize>().unwrap(), py.parse::<usize>().unwrap());

        if let Some(min) = (0..100)
            .cartesian_product(0..100)
            .filter_map(|(a, b)| {
                if a * ax + b * bx == px && a * ay + b * by == py {
                    println!("yes {a} {b}");
                    Some(3 * a + b)
                } else {
                    None
                }
            })
            .min()
        {
            result += min;
        }

        if lines.next().is_none() {
            break;
        }
    }

    result
}

pub fn solve_second(input: &str, offset: usize) -> usize {
    let mut lines = input.lines();
    let mut result = 0;

    loop {
        let (ax, ay) = lines.next().unwrap().split_once(", Y+").unwrap();
        let (_, ax) = ax.split_once(" X+").unwrap();
        let (ax, ay) = (ax.parse::<usize>().unwrap(), ay.parse::<usize>().unwrap());

        let (bx, by) = lines.next().unwrap().split_once(", Y+").unwrap();
        let (_, bx) = bx.split_once(" X+").unwrap();
        let (bx, by) = (bx.parse::<usize>().unwrap(), by.parse::<usize>().unwrap());

        let (px, py) = lines.next().unwrap().split_once(", Y=").unwrap();
        let (_, px) = px.split_once(" X=").unwrap();
        let (px, py) = (px.parse::<usize>().unwrap(), py.parse::<usize>().unwrap());
        let (px, py) = (px + offset, py + offset);

        // | ax bx | * | a | = | px |
        // | ay by |   | b |   | py |

        let m = 1.0 / ((ax * by) as isize - (bx * ay) as isize) as f64;
        let top_left = by as f64;
        let top_right = -(bx as f64);
        let bottom_left = -(ay as f64);
        let bottom_right = ax as f64;

        let a = m * (top_left * px as f64 + top_right * py as f64);
        let b = m * (bottom_left * px as f64 + bottom_right * py as f64);

        if a.is_sign_positive()
            && b.is_sign_positive()
            && (a.fract() < 0.01 || a.fract() > 0.99)
            && (b.fract() < 0.01 || b.fract() > 0.99)
        {
            // println!("yes {} {}", a.round() as usize, b.round() as usize);
            println!("{a} {b} yes {} {}", a.round() as usize, b.round() as usize);
            result += a.round() as usize * 3 + b.round() as usize;
        } else {
            // println!("{a} {b} no");
        }

        if lines.next().is_none() {
            break;
        }
    }

    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(480, solve_second(sample, 0));
    assert_eq!(875318608908, solve_second(sample, 10000000000000));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(36954, solve_second(input, 0));
    assert_eq!(79352015273424, solve_second(input, 10000000000000));
}

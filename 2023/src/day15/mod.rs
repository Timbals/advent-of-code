use std::array::from_fn;

pub fn solve_first(input: &str) -> usize {
    input
        .split(',')
        .map(|string| {
            string
                .bytes()
                .fold(0_u8, |acc, c| acc.wrapping_add(c).wrapping_mul(17)) as usize
        })
        .sum()
}

pub fn solve_second(input: &str) -> usize {
    let mut boxes: [Vec<_>; 256] = from_fn(|_| Vec::new());

    for (label, focal_length, hash) in input.split(',').map(|string| {
        let step = string.as_bytes();
        let (label, focal_length) = if step.last().unwrap() == &b'-' {
            (&step[..step.len() - 1], None)
        } else {
            (&step[..step.len() - 2], Some(step[step.len() - 1] - b'0'))
        };
        let hash = label
            .iter()
            .fold(0_u8, |acc, &c| acc.wrapping_add(c).wrapping_mul(17));

        (label, focal_length, hash)
    }) {
        let boxx = &mut boxes[hash as usize];
        let pos = boxx.iter().position(|(l, _)| l == &label);

        match (pos, focal_length) {
            (Some(pos), Some(focal_length)) => {
                boxx[pos].1 = focal_length;
            }
            (None, Some(focal_length)) => {
                boxx.push((label, focal_length));
            }
            (Some(pos), None) => {
                boxx.remove(pos);
            }
            _ => {}
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_number, boxx)| {
            boxx.into_iter()
                .enumerate()
                .map(move |(slot, (_, focal_length))| {
                    (box_number + 1) * (slot + 1) * (focal_length as usize)
                })
        })
        .sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(1320, solve_first(sample));
    assert_eq!(145, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(494980, solve_first(input));
    assert_eq!(247933, solve_second(input));
}

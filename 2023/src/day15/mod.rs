use std::array::from_fn;

fn hash(data: &str) -> usize {
    data.bytes()
        .fold(0_u8, |acc, c| acc.wrapping_add(c).wrapping_mul(17)) as usize
}

pub fn solve_first(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

pub fn solve_second(input: &str) -> usize {
    let mut boxes: [Vec<_>; 256] = from_fn(|_| Vec::new());

    for step in input.split(',') {
        let (label, focal) = match step.trim_end_matches('-').split_once('=') {
            Some((label, focal)) => (label, Some(focal.parse::<usize>().unwrap())),
            None => (&step[..step.len() - 1], None),
        };
        let hash = hash(label);

        match (boxes[hash].iter().position(|(l, _)| l == &label), focal) {
            (Some(pos), Some(focal)) => {
                boxes[hash][pos].1 = focal;
            }
            (None, Some(focal)) => {
                boxes[hash].push((label, focal));
            }
            (Some(pos), None) => {
                boxes[hash].remove(pos);
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
                .map(move |(slot, (_, focal))| (box_number + 1) * (slot + 1) * focal)
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

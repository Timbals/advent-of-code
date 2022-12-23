use ahash::{AHashMap, AHashSet};

pub fn solve(input: &str, round_limit: Option<usize>) -> usize {
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, v)| **v == b'#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect::<AHashSet<_>>();

    for i in 0..round_limit.unwrap_or(usize::MAX) {
        let mut movements = AHashMap::new();

        for (x, y) in elves.iter().copied() {
            let neighbors = [
                elves.contains(&(x - 1, y - 1)),
                elves.contains(&(x - 1, y)),
                elves.contains(&(x - 1, y + 1)),
                elves.contains(&(x, y - 1)),
                elves.contains(&(x, y + 1)),
                elves.contains(&(x + 1, y - 1)),
                elves.contains(&(x + 1, y)),
                elves.contains(&(x + 1, y + 1)),
            ];

            if neighbors.iter().all(|&v| !v) {
                continue;
            }

            let dirs = [
                (!neighbors[3] && !neighbors[0] && !neighbors[5], (0, -1)),
                (!neighbors[4] && !neighbors[2] && !neighbors[7], (0, 1)),
                (!neighbors[1] && !neighbors[0] && !neighbors[2], (-1, 0)),
                (!neighbors[6] && !neighbors[5] && !neighbors[7], (1, 0)),
            ];
            if let Some((_, (dx, dy))) = (0..4).map(|j| dirs[(i + j) % 4]).find(|(v, _)| *v) {
                if let Some((ox, oy)) = movements.insert((x + dx, y + dy), (x, y)) {
                    movements.remove(&(x + dx, y + dy));
                    movements.extend([((x, y), (x, y)), ((ox, oy), (ox, oy))]);
                }
            }
        }

        let mut any = false;
        for (to, from) in movements.iter() {
            elves.remove(from);
            elves.insert(*to);
            any = true;
        }
        if round_limit.is_none() && !any {
            return i + 1;
        }
    }

    let y = elves.iter().map(|(_, y)| y);
    let height = (y.clone().max().unwrap() - y.min().unwrap()).unsigned_abs() + 1;
    let x = elves.iter().map(|(x, _)| x);
    let width = (x.clone().max().unwrap() - x.min().unwrap()).unsigned_abs() + 1;

    width * height - elves.len()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(110, solve(sample, Some(10)));
    assert_eq!(20, solve(sample, None));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(4052, solve(input, Some(10)));
    assert_eq!(978, solve(input, None));
}

use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &str) -> usize {
    let mut components = HashMap::<_, Vec<_>>::new();
    for line in input.lines() {
        let (from, to) = line.split_once(": ").unwrap();
        for to in to.split_whitespace() {
            components.entry(from).or_default().push(to);
            components.entry(to).or_default().push(from);
        }
    }
    let mut name_to_index = HashMap::new();
    let components = components
        .into_iter()
        .enumerate()
        .map(|(i, (name, to))| {
            name_to_index.insert(name, i);
            to
        })
        .collect::<Vec<_>>();
    let components = components
        .into_iter()
        .map(|to| {
            to.into_iter()
                .map(|to| name_to_index[to])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm
    // There is probably a faster way

    let start = 0;
    let mut count = 1;
    'outer: for end in 1..components.len() {
        // try to find 4 unique paths from `start` to `end`
        let mut removed_edges = HashSet::<(usize, usize)>::new();

        for _ in 0..4 {
            let mut queue = VecDeque::new();
            let mut pred = vec![None; components.len()];
            queue.push_back(start);
            while let Some(current) = queue.pop_front() {
                if current == end {
                    break;
                }

                for &neighbor in &components[current] {
                    if pred[neighbor].is_none()
                        && !removed_edges.contains(&(current, neighbor))
                        && !removed_edges.contains(&(neighbor, current))
                    {
                        pred[neighbor] = Some(current);
                        queue.push_back(neighbor);
                    }
                }
            }

            if pred[end].is_none() {
                // No 4 paths from `start` to `end`
                continue 'outer;
            }

            // "remove" edges by walking the path from the end
            let mut current = end;
            while current != start {
                let pred = pred[current].unwrap();

                removed_edges.insert((pred, current));

                current = pred;
            }
        }

        count += 1;
    }

    count * (components.len() - count)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(54, solve(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(571753, solve(input));
}

use std::collections::{HashMap, VecDeque};

pub fn solve(input: &str) -> usize {
    let mut name_to_index = HashMap::new();
    let mut components = Vec::new();
    let insert_index = |components: &mut Vec<Vec<usize>>| {
        let index = components.len();
        components.push(Vec::new());
        index
    };
    for line in input.lines() {
        let (from, to) = line.split_once(": ").unwrap();
        let from = *name_to_index
            .entry(from)
            .or_insert_with(|| insert_index(&mut components));

        for to in to.split_whitespace() {
            let to = *name_to_index
                .entry(to)
                .or_insert_with(|| insert_index(&mut components));

            components[from].push(to);
            components[to].push(from);
        }
    }

    // https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm
    // There is probably a faster way

    let start = 0;
    let mut count = 1;

    // `queue` and `removed_edges` get reset inside the loop to save allocations
    let mut queue = VecDeque::new();
    let mut removed_edges = vec![false; components.len() * components.len()];

    'outer: for end in 1..components.len() {
        // try to find 4 paths with unique edges from `start` to `end`

        // reset `removed_edges` -> all edges are active again
        for removed in &mut removed_edges {
            *removed = false;
        }

        for _ in 0..4 {
            let mut predecessor = vec![None; components.len()];

            queue.clear();
            queue.push_back(start);

            while let Some(current) = queue.pop_front() {
                if current == end {
                    break;
                }

                for &neighbor in &components[current] {
                    if predecessor[neighbor].is_none()
                        && !removed_edges[current + neighbor * components.len()]
                    {
                        predecessor[neighbor] = Some(current);
                        queue.push_back(neighbor);
                    }
                }
            }

            if predecessor[end].is_none() {
                // no 4 paths from `start` to `end`
                continue 'outer;
            }

            // "remove" edges walking the path from the end
            let mut current = end;
            while current != start {
                let pred = predecessor[current].unwrap();

                removed_edges[pred + current * components.len()] = true;
                removed_edges[current + pred * components.len()] = true;

                current = pred;
            }
        }

        // successfully found 4 paths with unique edges from `start` to `end`
        // -> they are part of the same component in the solution
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

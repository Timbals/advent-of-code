use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn solve_first(input: &str) -> usize {
    let mut lines = input.lines();
    let available = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    lines.next().unwrap();

    let mut result = 0;
    for design in lines {
        let mut visited = HashSet::new();
        let mut stack = vec![design];
        while let Some(design) = stack.pop() {
            if design.is_empty() {
                result += 1;
                break;
            }

            for &towel in &available {
                if let Some(remaining) = design.strip_prefix(towel) {
                    if visited.insert(remaining) {
                        stack.push(remaining);
                    }
                }
            }
        }
    }
    result
}

pub fn solve_second(input: &str) -> usize {
    let mut lines = input.lines();
    let available = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    lines.next().unwrap();

    let mut result = 0;
    for design in lines {
        let mut visited = HashMap::new();
        let mut heap = BinaryHeap::new();
        heap.push((design.len(), design));
        visited.insert(design, 1);
        while let Some((_, design)) = heap.pop() {
            for &towel in &available {
                if let Some(remaining) = design.strip_prefix(towel) {
                    if !visited.contains_key(remaining) {
                        heap.push((remaining.len(), remaining));
                    }
                    *visited.entry(remaining).or_default() += visited[design];
                }
            }
        }

        result += visited.get("").unwrap_or(&0);
    }
    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(6, solve_first(sample));
    assert_eq!(16, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(367, solve_first(input));
    assert_eq!(724388733465031, solve_second(input));
}

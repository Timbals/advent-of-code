use std::collections::{BinaryHeap, HashMap};

pub fn solve(input: &str, possibilities: bool) -> usize {
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

        if possibilities {
            result += visited.get("").unwrap_or(&0);
        } else {
            result += usize::from(visited.contains_key(""));
        }
    }
    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(6, solve(sample, false));
    assert_eq!(16, solve(sample, true));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(367, solve(input, false));
    assert_eq!(724388733465031, solve(input, true));
}

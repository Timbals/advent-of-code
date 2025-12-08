use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Box {
    index: usize,
    x: usize,
    y: usize,
    z: usize,

    parent: Option<usize>, // disjoint-set data structure
    circuit_size: usize,
}

impl Box {
    fn circuit(&self, boxes: &[Box]) -> usize {
        let mut current = self.index;
        let mut parent = self.parent;
        while let Some(parent_index) = parent {
            current = boxes[parent_index].index;
            parent = boxes[parent_index].parent;
        }
        current
    }

    fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

pub fn solve_first(input: &str, connection_count: usize) -> usize {
    let mut boxes = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (x, rest) = line.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();
            Box {
                index,
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
                z: z.parse::<usize>().unwrap(),
                parent: None,
                circuit_size: 1,
            }
        })
        .collect::<Vec<_>>();

    let mut distances = boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((a_index, a), (b_index, b))| (Reverse(a.dist(b)), a_index, b_index))
        .collect::<BinaryHeap<_>>();

    for _ in 0..connection_count {
        let (_, a_index, b_index) = distances.pop().unwrap();
        let a = &boxes[a_index];
        let a_circuit = a.circuit(&boxes);
        let b = &boxes[b_index];
        let b_circuit = b.circuit(&boxes);

        if a_circuit != b_circuit {
            boxes[b_circuit].parent = Some(a_circuit);
            boxes[a_circuit].circuit_size += boxes[b_circuit].circuit_size;
        }
    }

    boxes.into_iter().filter(|b| b.parent.is_none()).map(|b| b.circuit_size).k_largest(3).product()
}

pub fn solve_second(input: &str) -> usize {
    let mut boxes = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (x, rest) = line.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();
            Box {
                index,
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
                z: z.parse::<usize>().unwrap(),
                parent: None,
                circuit_size: 1,
            }
        })
        .collect::<Vec<_>>();

    let mut distances = boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((a_index, a), (b_index, b))| (Reverse(a.dist(b)), a_index, b_index))
        .collect::<BinaryHeap<_>>();

    loop {
        let (_, a_index, b_index) = distances.pop().unwrap();
        let a = &boxes[a_index];
        let a_circuit = a.circuit(&boxes);
        let b = &boxes[b_index];
        let b_circuit = b.circuit(&boxes);

        if a_circuit != b_circuit {
            boxes[b_circuit].parent = Some(a_circuit);
            boxes[a_circuit].circuit_size += boxes[b_circuit].circuit_size;

            if boxes[a_circuit].circuit_size == boxes.len() {
                return boxes[a_index].x * boxes[b_index].x;
            }
        }
    }
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(40, solve_first(sample, 10));
    assert_eq!(25272, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(54180, solve_first(input, 1000));
    assert_eq!(25325968, solve_second(input));
}

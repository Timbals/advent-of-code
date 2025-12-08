use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct JunctionBox {
    index: usize,
    x: i64,
    y: i64,
    z: i64,

    parent: usize, // disjoint-set data structure
    circuit_size: usize,
}

impl JunctionBox {
    fn dist_squared(&self, other: &Self) -> i64 {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)
    }

    fn find_circuit(mut current: usize, boxes: &mut [JunctionBox]) -> usize {
        while boxes[current].parent != current {
            (current, boxes[current].parent) =
                (boxes[current].parent, boxes[boxes[current].parent].parent);
        }
        current
    }

    fn union_circuit(mut a: usize, mut b: usize, boxes: &mut [JunctionBox]) -> usize {
        if a == b {
            return a;
        }
        if boxes[a].circuit_size < boxes[b].circuit_size {
            std::mem::swap(&mut a, &mut b);
        }
        boxes[b].parent = a;
        boxes[a].circuit_size += boxes[b].circuit_size;
        a
    }
}

pub fn solve(input: &str, connection_count: Option<usize>) -> usize {
    let mut boxes = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let mut coordinates = line.split(',').map(|x| x.parse().unwrap());
            JunctionBox {
                index,
                x: coordinates.next().unwrap(),
                y: coordinates.next().unwrap(),
                z: coordinates.next().unwrap(),
                parent: index,
                circuit_size: 1,
            }
        })
        .collect::<Vec<_>>();

    let mut distances = boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((a_index, a), (b_index, b))| (Reverse(a.dist_squared(b)), a_index, b_index))
        .collect::<BinaryHeap<_>>();

    for _ in 0..connection_count.unwrap_or(usize::MAX) {
        let (_, a_index, b_index) = distances.pop().unwrap();
        let a_circuit = JunctionBox::find_circuit(a_index, &mut boxes);
        let b_circuit = JunctionBox::find_circuit(b_index, &mut boxes);
        let union = JunctionBox::union_circuit(a_circuit, b_circuit, &mut boxes);

        if boxes[union].circuit_size == boxes.len() {
            return (boxes[a_index].x * boxes[b_index].x) as usize;
        }
    }

    boxes.into_iter().filter(|b| b.index == b.parent).map(|b| b.circuit_size).k_largest(3).product()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(40, solve(sample, Some(10)));
    assert_eq!(25272, solve(sample, None));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(54180, solve(input, Some(1000)));
    assert_eq!(25325968, solve(input, None));
}

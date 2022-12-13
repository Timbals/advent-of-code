use itertools::Itertools;
use std::cmp::Ordering;
use std::slice::from_ref;
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.starts_with('[') {
            let mut children = Vec::new();
            let mut last_comma = 1;
            let mut bracket_count = -1;
            for (i, x) in input.bytes().enumerate() {
                match x {
                    b'[' => bracket_count += 1,
                    b']' => bracket_count -= 1,
                    b',' if bracket_count == 0 => {
                        children.push(input[last_comma..i].parse()?);
                        last_comma = i + 1;
                    }
                    _ => {}
                }
            }
            if last_comma + 1 != input.len() {
                children.push(input[last_comma..input.len() - 1].parse()?);
            }

            Ok(Self::List(children))
        } else {
            Ok(Self::Integer(input.parse().unwrap()))
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => a.partial_cmp(b),
            (a, Packet::List(b)) => from_ref(a).partial_cmp(b),
            (Packet::List(a), b) => a.as_slice().partial_cmp(from_ref(b)),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn solve_first(input: &str) -> usize {
    let packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Packet>().unwrap())
        .collect_vec();

    packets
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn solve_second(input: &str) -> usize {
    let divider_one: Packet = "[[2]]".parse().unwrap();
    let divider_two: Packet = "[[6]]".parse().unwrap();

    let mut packets = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Packet>().unwrap())
        .chain([divider_one.clone(), divider_two.clone()])
        .collect_vec();

    packets.sort();

    (packets.iter().position(|x| x == &divider_one).unwrap() + 1)
        * (packets.iter().position(|x| x == &divider_two).unwrap() + 1)
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(13, solve_first(sample));
    assert_eq!(140, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(6086, solve_first(input));
    assert_eq!(27930, solve_second(input));
}

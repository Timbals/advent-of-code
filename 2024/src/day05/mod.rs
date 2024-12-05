use std::collections::{HashMap, HashSet};

pub fn solve_first_old(input: &str) -> usize {
    //! input contains cycles, so a graph-based approach doesn't work

    let mut lines = input.lines();
    let orderings = lines
        .by_ref()
        .take_while(|&line| !line.is_empty())
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));

    // how to sort a partial ordering -> toposort, by treating it like a graph?
    let mut outgoing = HashMap::<usize, HashSet<usize>>::new();
    let mut incoming = HashMap::<usize, HashSet<usize>>::new();

    for (a, b) in orderings {
        outgoing.entry(a).or_default().insert(b);
        incoming.entry(b).or_default().insert(a);

        outgoing.entry(b).or_default();
        incoming.entry(a).or_default();
    }

    let mut sort_order = Vec::new();
    while !incoming.is_empty() {
        let new =
            incoming.iter().filter(|(_, x)| x.is_empty()).map(|(x, _)| *x).collect::<Vec<_>>();

        for x in &new {
            for y in &outgoing[x] {
                incoming.entry(*y).or_default().remove(x);
            }
            incoming.remove(x);
        }

        sort_order.extend(new);
    }

    let sort_order =
        sort_order.into_iter().enumerate().map(|(i, x)| (x, i)).collect::<HashMap<usize, usize>>();

    let mut result = 0;
    for update in lines {
        let update = update.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let middle = update[update.len() / 2];

        if update.iter().filter_map(|x| sort_order.get(x)).is_sorted() {
            result += middle;
        }
    }

    result
}

pub fn solve_first(input: &str) -> usize {
    let mut lines = input.lines();
    let orderings = lines
        .by_ref()
        .take_while(|&line| !line.is_empty())
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));

    // how to sort a partial ordering -> toposort, by treating it like a graph?
    let mut outgoing = HashMap::<usize, HashSet<usize>>::new();
    let mut incoming = HashMap::<usize, HashSet<usize>>::new();

    for (a, b) in orderings {
        outgoing.entry(a).or_default().insert(b);
        incoming.entry(b).or_default().insert(a);

        outgoing.entry(b).or_default();
        incoming.entry(a).or_default();
    }

    let mut result = 0;
    'outer: for update in lines {
        let update = update.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let middle = update[update.len() / 2];

        let mut disallowed = HashSet::new();
        for x in update {
            if disallowed.contains(&x) {
                continue 'outer;
            }

            for y in incoming.get(&x).unwrap_or(&HashSet::new()) {
                disallowed.insert(*y);
            }
        }

        result += middle;
    }

    result
}

pub fn solve_second(input: &str) -> usize {
    let mut lines = input.lines();
    let orderings = lines
        .by_ref()
        .take_while(|&line| !line.is_empty())
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));

    let mut outgoing = HashMap::<usize, HashSet<usize>>::new();
    let mut incoming = HashMap::<usize, HashSet<usize>>::new();

    for (a, b) in orderings {
        outgoing.entry(a).or_default().insert(b);
        incoming.entry(b).or_default().insert(a);

        outgoing.entry(b).or_default();
        incoming.entry(a).or_default();
    }

    let mut result = 0;
    for update in lines {
        let mut update = update.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

        let mut incorrect = false;
        'inner: loop {
            let mut disallowed = HashMap::new();
            for (i, x) in update.iter().enumerate() {
                if let Some(&j) = disallowed.get(x) {
                    incorrect = true;
                    update.swap(i, j);
                    continue 'inner;
                }

                for y in incoming.get(x).unwrap_or(&HashSet::new()) {
                    disallowed.insert(*y, i);
                }
            }

            break 'inner;
        }

        if incorrect {
            result += update[update.len() / 2];
        }
    }

    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(143, solve_first(sample));
    assert_eq!(123, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(6612, solve_first(input));
    assert_eq!(4944, solve_second(input));
}

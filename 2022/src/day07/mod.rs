use std::collections::HashMap;

pub fn parse(input: &str) -> HashMap<String, usize> {
    let mut directories = HashMap::new();
    let mut current = vec![""];

    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["$", "cd", ".."] => {
                current.pop();
            }
            ["$", "cd", "/"] => current = vec![""],
            ["$", "cd", dir] => current.push(*dir),
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, _] => {
                let size = size.parse::<usize>().unwrap();
                // we use scan to yield all intermediate paths from the root
                let parents = current.iter().scan(String::new(), |state, dir| {
                    state.push_str(dir);
                    state.push('/'); // the input also works without the separator (but it's technically necessary to differentiate between /a/a/ and /aa/)
                    Some(state.clone())
                });
                for dir in parents {
                    // this assumes every file is only ever seen once
                    *directories.entry(dir).or_default() += size;
                }
            }
            _ => {}
        }
    }

    directories
}

pub fn solve_first(input: &str) -> usize {
    parse(input)
        .into_values()
        .filter(|&size| size <= 100000)
        .sum()
}

pub fn solve_second(input: &str) -> usize {
    let directories = parse(input);
    let needed = directories["/"] - 40_000_000;
    directories
        .into_values()
        .filter(|&size| size >= needed)
        .min()
        .unwrap()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(95437, solve_first(sample));
    assert_eq!(24933642, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1501149, solve_first(input));
    assert_eq!(10096985, solve_second(input));
}

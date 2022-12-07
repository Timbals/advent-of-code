use std::collections::HashMap;

#[derive(Default)]
struct Directory<'a> {
    parent: usize,
    size: usize,
    subdirectories: HashMap<&'a str, usize>,
}

impl<'a> Directory<'a> {
    fn size(&self, system: &Vec<Directory>) -> usize {
        self.size
            + self
                .subdirectories
                .values()
                .map(|&i| system[i].size(system))
                .sum::<usize>()
    }
}

fn parse(input: &str) -> Vec<Directory> {
    let mut directories = vec![Directory::default()];

    let mut current = 0;

    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["$", "cd", ".."] => current = directories[current].parent,
            ["$", "cd", "/"] => current = 0,
            ["$", "cd", dir] => current = directories[current].subdirectories[dir],
            ["$", "ls"] => {}
            ["dir", name] => {
                if !directories[current].subdirectories.contains_key(name) {
                    let index = directories.len();
                    directories[current].subdirectories.insert(name, index);
                    directories.push(Directory {
                        parent: current,
                        size: 0,
                        subdirectories: HashMap::new(),
                    });
                }
            }
            [size, _] => {
                directories[current].size += size.parse::<usize>().unwrap();
            }
            _ => {}
        }
    }

    directories
}

fn solve_first(input: &str) -> usize {
    let directories = parse(input);
    directories
        .iter()
        .map(|x| x.size(&directories))
        .filter(|&size| size <= 100000)
        .sum()
}

fn solve_second(input: &str) -> usize {
    let directories = parse(input);
    let free = 70000000 - directories[0].size(&directories);
    let needed = 30000000_usize.saturating_sub(free);
    directories
        .iter()
        .map(|x| x.size(&directories))
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

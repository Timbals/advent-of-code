use std::collections::HashMap;

struct Directory<'a> {
    name: &'a str,
    index: usize,
    parent: usize,
    files: HashMap<&'a str, usize>,
    subdirectories: HashMap<&'a str, usize>,
}

impl<'a> Directory<'a> {
    fn size(&self, system: &Vec<Directory>) -> usize {
        self.files.values().sum::<usize>()
            + self
                .subdirectories
                .values()
                .map(|&i| system[i].size(system))
                .sum::<usize>()
    }
}

fn parse(input: &str) -> Vec<Directory> {
    let root = Directory {
        name: "",
        index: 0,
        parent: 0,
        files: HashMap::new(),
        subdirectories: HashMap::new(),
    };
    let mut directories = vec![root];

    let mut current = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        match parts.next().unwrap() {
            "$" => {
                if parts.next().unwrap() == "cd" {
                    match parts.next().unwrap() {
                        ".." => current = directories[current].parent,
                        "/" => current = 0,
                        dir => {
                            current = *directories[current].subdirectories.get(dir).unwrap();
                        }
                    }
                }
            }
            "dir" => {
                let name = parts.next().unwrap();
                if !directories[current].subdirectories.contains_key(name) {
                    let index = directories.len();
                    directories[current].subdirectories.insert(name, index);
                    let new = Directory {
                        name,
                        index,
                        parent: current,
                        files: HashMap::new(),
                        subdirectories: HashMap::new(),
                    };
                    directories.push(new);
                }
            }
            file_size => {
                let name = parts.next().unwrap();
                let file_size = file_size.parse::<usize>().unwrap();

                directories[current].files.insert(name, file_size);
            }
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

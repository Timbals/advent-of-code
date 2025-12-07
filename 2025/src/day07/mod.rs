pub fn solve_first(input: &str) -> usize {
    let mut grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<_>>();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find(|(_, byte)| **byte == b'S').map(|(x, _)| (x, y))
        })
        .unwrap();

    let mut stack = vec![start];
    let mut split_count = 0;

    while let Some((x, y)) = stack.pop() {
        match grid.get(y).and_then(|line| line.get(x)) {
            Some(b'S') => {
                stack.push((x, y + 1));
            }
            Some(b'.') => {
                grid[y][x] = b'|';
                stack.push((x, y + 1));
            }
            Some(b'^') => {
                split_count += 1;
                stack.push((x - 1, y));
                stack.push((x + 1, y));
            }
            Some(b'|') | None => {}
            _ => unreachable!(),
        }
    }

    split_count
}

pub fn solve_second(input: &str) -> usize {
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let mut state = vec![0; grid[0].len()];
    let mut next_state = vec![0; grid[0].len()];
    for line in grid.into_iter() {
        for (x, byte) in line.iter().copied().enumerate() {
            match byte {
                b'S' => next_state[x] += 1,
                b'.' => next_state[x] += state[x],
                b'^' => {
                    next_state[x - 1] += state[x];
                    next_state[x + 1] += state[x];
                }
                _ => unreachable!(),
            }
        }
        std::mem::swap(&mut state, &mut next_state);
        next_state.fill(0);
    }

    state.into_iter().sum()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(21, solve_first(sample));
    assert_eq!(40, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1703, solve_first(input));
    assert_eq!(171692855075500, solve_second(input));
}

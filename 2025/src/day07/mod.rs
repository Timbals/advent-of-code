pub fn solve_first(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let mut state = vec![false; width];
    let mut next_state = vec![false; width];
    let mut split_count = 0;
    for row in input.lines().map(str::as_bytes) {
        for (x, byte) in row.iter().copied().enumerate() {
            match byte {
                b'S' => next_state[x] = true,
                b'.' => next_state[x] |= state[x],
                b'^' => {
                    if state[x] {
                        split_count += 1;
                    }
                    next_state[x - 1] |= state[x];
                    next_state[x + 1] |= state[x];
                }
                _ => unreachable!(),
            }
        }
        std::mem::swap(&mut state, &mut next_state);
        next_state.fill(false);
    }
    split_count
}

pub fn solve_second(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let mut state = vec![0; width];
    let mut next_state = vec![0; width];
    for row in input.lines().map(str::as_bytes) {
        for (x, byte) in row.iter().copied().enumerate() {
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

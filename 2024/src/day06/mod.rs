use std::collections::HashSet;

fn parse(input: &str) -> ((usize, usize), Vec<Vec<bool>>) {
    let mut guard = (0, 0);
    let obstacles = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, symbol)| match symbol {
                    b'#' => true,
                    b'^' => {
                        guard.0 = x;
                        guard.1 = y;
                        false
                    }
                    _ => false,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (guard, obstacles)
}

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn patrol(
    mut position: (usize, usize),
    mut direction_index: usize,
    obstacles: &[Vec<bool>],
) -> Option<usize> {
    let width = obstacles[0].len();
    let height = obstacles.len();

    let mut direction = DIRECTIONS[direction_index];

    let mut positions = HashSet::new();
    while (0..width).contains(&position.0) && (0..height).contains(&position.1) {
        if !positions.insert((position, direction_index)) {
            return None;
        }
        while obstacles
            .get(position.1.wrapping_add_signed(direction.1))
            .and_then(|row| row.get(position.0.wrapping_add_signed(direction.0)))
            == Some(&true)
        {
            direction_index = (direction_index + 1) % DIRECTIONS.len();
            direction = DIRECTIONS[direction_index];
        }
        position.0 = position.0.wrapping_add_signed(direction.0);
        position.1 = position.1.wrapping_add_signed(direction.1);
    }

    Some(positions.into_iter().map(|(x, _)| x).collect::<HashSet<_>>().len())
}

pub fn solve_first(input: &str) -> usize {
    let (guard, obstacles) = parse(input);
    patrol(guard, 0, &obstacles).expect("initial configuration looped")
}

pub fn solve_second(input: &str) -> usize {
    let (mut guard, mut obstacles) = parse(input);
    let width = obstacles[0].len();
    let height = obstacles.len();

    let mut current_direction = 0;
    let mut direction = DIRECTIONS[current_direction];

    let mut positions = HashSet::new();
    let mut cycles = HashSet::new();
    while (0..width).contains(&guard.0) && (0..height).contains(&guard.1) {
        positions.insert(guard);

        while obstacles
            .get(guard.1.wrapping_add_signed(direction.1))
            .and_then(|row| row.get(guard.0.wrapping_add_signed(direction.0)))
            == Some(&true)
        {
            current_direction = (current_direction + 1) % DIRECTIONS.len();
            direction = DIRECTIONS[current_direction];
        }
        let old = guard;
        guard.0 = guard.0.wrapping_add_signed(direction.0);
        guard.1 = guard.1.wrapping_add_signed(direction.1);

        if (0..width).contains(&guard.0)
            && (0..height).contains(&guard.1)
            && !positions.contains(&guard)
        {
            obstacles[guard.1][guard.0] = true;
            if patrol(old, current_direction, &obstacles).is_none() {
                cycles.insert(guard);
            }
            obstacles[guard.1][guard.0] = false;
        }
    }

    cycles.len()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(41, solve_first(sample));
    assert_eq!(6, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(5129, solve_first(input));
    assert_eq!(1888, solve_second(input));
}

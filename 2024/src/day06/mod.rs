use std::collections::HashSet;

pub fn solve_first(input: &str) -> usize {
    let mut guard = (0, 0);
    let mut bounds = (0, 0);
    let obstacles = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            bounds.1 = bounds.1.max(y);
            line.bytes()
                .enumerate()
                .map(|(x, symbol)| {
                    bounds.0 = bounds.0.max(x);
                    match symbol {
                        b'#' => true,
                        b'^' => {
                            guard.0 = x;
                            guard.1 = y;
                            false
                        }
                        _ => false,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut current_direction = 0;
    let mut direction = directions[current_direction];

    let mut positions = HashSet::new();
    while (0..=bounds.0).contains(&guard.0) && (0..=bounds.1).contains(&guard.1) {
        positions.insert(guard);
        while obstacles
            .get(guard.1.wrapping_add_signed(direction.1))
            .and_then(|row| row.get(guard.0.wrapping_add_signed(direction.0)))
            == Some(&true)
        {
            current_direction = (current_direction + 1) % directions.len();
            direction = directions[current_direction];
        }
        guard.0 = guard.0.wrapping_add_signed(direction.0);
        guard.1 = guard.1.wrapping_add_signed(direction.1);
    }

    positions.len()
}

fn check_loop(
    mut pos: (usize, usize),
    mut current_direction: usize,
    bounds: (usize, usize),
    obstacles: &Vec<Vec<bool>>,
) -> bool {
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut direction = directions[current_direction];

    let mut positions = HashSet::new();
    while (0..=bounds.0).contains(&pos.0) && (0..=bounds.1).contains(&pos.1) {
        if !positions.insert((pos, current_direction)) {
            return true;
        }
        while obstacles
            .get(pos.1.wrapping_add_signed(direction.1))
            .and_then(|row| row.get(pos.0.wrapping_add_signed(direction.0)))
            == Some(&true)
        {
            current_direction = (current_direction + 1) % directions.len();
            direction = directions[current_direction];
            positions.insert((pos, current_direction));
        }
        pos.0 = pos.0.wrapping_add_signed(direction.0);
        pos.1 = pos.1.wrapping_add_signed(direction.1);
    }

    false
}

pub fn solve_second(input: &str) -> usize {
    let mut guard = (0, 0);
    let mut bounds = (0, 0);
    let mut obstacles = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            bounds.1 = bounds.1.max(y);
            line.bytes()
                .enumerate()
                .map(|(x, symbol)| {
                    bounds.0 = bounds.0.max(x);
                    match symbol {
                        b'#' => true,
                        b'^' => {
                            guard.0 = x;
                            guard.1 = y;
                            false
                        }
                        _ => false,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut current_direction = 0;
    let mut direction = directions[current_direction];

    let mut positions = HashSet::new();
    let mut cycles = HashSet::new();
    while (0..=bounds.0).contains(&guard.0) && (0..=bounds.1).contains(&guard.1) {
        positions.insert((guard, current_direction));

        while obstacles
            .get(guard.1.wrapping_add_signed(direction.1))
            .and_then(|row| row.get(guard.0.wrapping_add_signed(direction.0)))
            == Some(&true)
        {
            current_direction = (current_direction + 1) % directions.len();
            direction = directions[current_direction];
            positions.insert((guard, current_direction));
        }
        let old = guard;
        guard.0 = guard.0.wrapping_add_signed(direction.0);
        guard.1 = guard.1.wrapping_add_signed(direction.1);

        if (0..=bounds.0).contains(&guard.0)
            && (0..=bounds.1).contains(&guard.1)
            && !positions.contains(&(guard, 0))
            && !positions.contains(&(guard, 1))
            && !positions.contains(&(guard, 2))
            && !positions.contains(&(guard, 3))
        {
            obstacles[guard.1][guard.0] = true;
            if check_loop(old, current_direction, bounds, &obstacles) {
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

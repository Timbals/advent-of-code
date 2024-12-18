use std::collections::VecDeque;

pub fn solve_first(input: &str, size: usize, limit: usize) -> usize {
    let mut visited =
        (0..size).map(|_| (0..size).map(|_| false).collect::<Vec<_>>()).collect::<Vec<_>>();
    for (x, y) in input.lines().take(limit).map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }) {
        visited[y][x] = true;
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    while let Some((steps, x, y)) = queue.pop_front() {
        if x == size - 1 && y == size - 1 {
            return steps;
        }

        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            if (0..size).contains(&nx) && (0..size).contains(&ny) && !visited[ny][nx] {
                visited[ny][nx] = true;
                queue.push_back((steps + 1, nx, ny));
            }
        }
    }

    unreachable!("no valid path")
}

pub fn solve_second(input: &str, size: usize) -> String {
    let mut visited_template =
        (0..size).map(|_| (0..size).map(|_| false).collect::<Vec<_>>()).collect::<Vec<_>>();
    'outer: for (new_x, new_y) in input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }) {
        visited_template[new_y][new_x] = true;
        let mut visited = visited_template.clone();

        let mut queue = VecDeque::new();
        queue.push_back((0, 0, 0));
        while let Some((steps, x, y)) = queue.pop_front() {
            if x == size - 1 && y == size - 1 {
                continue 'outer;
            }

            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                if (0..size).contains(&nx) && (0..size).contains(&ny) && !visited[ny][nx] {
                    visited[ny][nx] = true;
                    queue.push_back((steps + 1, nx, ny));
                }
            }
        }

        return format!("{new_x},{new_y}");
    }

    unreachable!("no byte blocks the path")
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(22, solve_first(sample, 7, 12));
    assert_eq!("6,1", solve_second(sample, 7));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(354, solve_first(input, 71, 1024));
    assert_eq!("36,17", solve_second(input, 71));
}

pub fn solve_first(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| (c, [false; 4])).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // (x, y, direction), right = 0; down = 1; left = 2; up = 3
    let mut queue = vec![(0, 0, 0)];

    while let Some((x, y, dir)) = queue.pop() {
        if y >= grid.len() || x >= grid[0].len() {
            continue;
        }

        if grid[y][x].1[dir] {
            continue;
        }

        grid[y][x].1[dir] = true;

        let mut new_dirs = Vec::with_capacity(2);

        match grid[y][x].0 {
            '.' => new_dirs.push(dir),
            '/' if dir == 0 || dir == 3 => new_dirs.push((dir + 3) % 6),
            '/' if dir == 1 || dir == 2 => new_dirs.push((dir % 2) + 1),
            '\\' if dir == 0 || dir == 1 => new_dirs.push((dir + 1) % 2),
            '\\' if dir == 2 => new_dirs.push(3),
            '\\' if dir == 3 => new_dirs.push(2),
            '|' if dir == 1 || dir == 3 => new_dirs.push(dir),
            '|' if dir == 0 || dir == 2 => new_dirs.extend_from_slice(&[1, 3]),
            '-' if dir == 0 || dir == 2 => new_dirs.push(dir),
            '-' if dir == 1 || dir == 3 => new_dirs.extend_from_slice(&[0, 2]),
            _ => unreachable!(),
        }

        for new_dir in new_dirs {
            match new_dir {
                0 => queue.push((x + 1, y, new_dir)),
                1 => queue.push((x, y + 1, new_dir)),
                2 => queue.push((x.wrapping_sub(1), y, new_dir)),
                3 => queue.push((x, y.wrapping_sub(1), new_dir)),
                _ => unreachable!(),
            }
        }
    }

    grid.iter()
        .flat_map(|row| row.iter().filter(|(_, x)| x.iter().any(|&y| y)))
        .count()
}

pub fn solve_second(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut starts = Vec::new();

    for y in 0..grid.len() {
        starts.push((0, y, 0));
        starts.push((grid[0].len() - 1, y, 2));
    }

    for x in 0..grid[0].len() {
        starts.push((x, 0, 1));
        starts.push((x, grid.len() - 1, 3));
    }

    starts
        .into_iter()
        .map(|start| {
            let mut energy = grid
                .iter()
                .map(|row| row.iter().map(|_| [false; 4]).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            // (x, y, direction), right = 0; down = 1; left = 2; up = 3
            let mut queue = vec![start];

            while let Some((x, y, dir)) = queue.pop() {
                if y >= grid.len() || x >= grid[0].len() {
                    continue;
                }

                if energy[y][x][dir] {
                    continue;
                }

                energy[y][x][dir] = true;

                let mut new_dirs = Vec::with_capacity(2);

                match grid[y][x] {
                    '.' => new_dirs.push(dir),
                    '/' if dir == 0 || dir == 3 => new_dirs.push((dir + 3) % 6),
                    '/' if dir == 1 || dir == 2 => new_dirs.push((dir % 2) + 1),
                    '\\' if dir == 0 || dir == 1 => new_dirs.push((dir + 1) % 2),
                    '\\' if dir == 2 => new_dirs.push(3),
                    '\\' if dir == 3 => new_dirs.push(2),
                    '|' if dir == 1 || dir == 3 => new_dirs.push(dir),
                    '|' if dir == 0 || dir == 2 => new_dirs.extend_from_slice(&[1, 3]),
                    '-' if dir == 0 || dir == 2 => new_dirs.push(dir),
                    '-' if dir == 1 || dir == 3 => new_dirs.extend_from_slice(&[0, 2]),
                    _ => unreachable!(),
                }

                for new_dir in new_dirs {
                    match new_dir {
                        0 => queue.push((x + 1, y, new_dir)),
                        1 => queue.push((x, y + 1, new_dir)),
                        2 => queue.push((x.wrapping_sub(1), y, new_dir)),
                        3 => queue.push((x, y.wrapping_sub(1), new_dir)),
                        _ => unreachable!(),
                    }
                }
            }

            dbg!(energy
                .iter()
                .flat_map(|row| row.iter().filter(|x| x.iter().any(|&y| y)))
                .count())
        })
        .max()
        .unwrap()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(46, solve_first(sample));
    assert_eq!(51, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(8116, solve_first(input));
    assert_eq!(8383, solve_second(input));
}

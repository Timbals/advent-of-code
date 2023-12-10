use std::collections::HashSet;

pub fn solve_first(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut start = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (x, y);
            }
        }
    }

    'outer: for (start_tile, start_from) in [
        ('|', 'S'),
        ('-', 'W'),
        ('L', 'E'),
        ('J', 'W'),
        ('7', 'W'),
        ('F', 'E'),
    ] {
        println!("testing {start_tile} {start_from}");

        let mut count = 0;

        let mut current = start;
        let mut tile = start_tile;
        let mut from = start_from;
        while current != start || count == 0 {
            println!("{current:?}, {tile:?} {from:?}");
            if tile == '.' {
                continue 'outer;
            }

            if count >= grid.len() * grid[0].len() {
                continue 'outer;
            }

            let (dx, dy) = match (tile, from) {
                ('|', 'S') => (0, -1),
                ('|', 'N') => (0, 1),
                ('-', 'W') => (1, 0),
                ('-', 'E') => (-1, 0),
                ('L', 'E') => (0, -1),
                ('L', 'N') => (1, 0),
                ('J', 'W') => (0, -1),
                ('J', 'N') => (-1, 0),
                ('7', 'W') => (0, 1),
                ('7', 'S') => (-1, 0),
                ('F', 'E') => (0, 1),
                ('F', 'S') => (1, 0),
                _ => continue 'outer,
            };
            current.0 = current.0.wrapping_add_signed(dx);
            current.1 = current.1.wrapping_add_signed(dy);
            tile = if let Some(&tile) = grid.get(current.1).and_then(|x| x.get(current.0)) {
                tile
            } else {
                continue 'outer;
            };
            from = match (dx, dy) {
                (0, 1) => 'N',
                (0, -1) => 'S',
                (1, 0) => 'W',
                (-1, 0) => 'E',
                _ => unreachable!("{dx} {dy}"),
            };
            count += 1;
        }

        return count / 2;
    }

    unreachable!()
}

pub fn solve_second(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut start = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (x, y);
            }
        }
    }

    let mut loop_tiles = HashSet::new();
    let mut final_start_tile = ' ';
    'outer: for (start_tile, start_from) in [
        ('|', 'S'),
        ('-', 'W'),
        ('L', 'E'),
        ('J', 'W'),
        ('7', 'W'),
        ('F', 'E'),
    ] {
        println!("testing {start_tile} {start_from}");

        let mut count = 0;

        loop_tiles.clear();
        let mut current = start;
        let mut tile = start_tile;
        let mut from = start_from;
        while current != start || count == 0 {
            loop_tiles.insert(current);

            if tile == '.' {
                continue 'outer;
            }

            if count >= grid.len() * grid[0].len() {
                continue 'outer;
            }

            let (dx, dy) = match (tile, from) {
                ('|', 'S') => (0, -1),
                ('|', 'N') => (0, 1),
                ('-', 'W') => (1, 0),
                ('-', 'E') => (-1, 0),
                ('L', 'E') => (0, -1),
                ('L', 'N') => (1, 0),
                ('J', 'W') => (0, -1),
                ('J', 'N') => (-1, 0),
                ('7', 'W') => (0, 1),
                ('7', 'S') => (-1, 0),
                ('F', 'E') => (0, 1),
                ('F', 'S') => (1, 0),
                _ => continue 'outer,
            };
            current.0 = current.0.wrapping_add_signed(dx);
            current.1 = current.1.wrapping_add_signed(dy);
            tile = if let Some(&tile) = grid.get(current.1).and_then(|x| x.get(current.0)) {
                tile
            } else {
                continue 'outer;
            };
            from = match (dx, dy) {
                (0, 1) => 'N',
                (0, -1) => 'S',
                (1, 0) => 'W',
                (-1, 0) => 'E',
                _ => unreachable!("{dx} {dy}"),
            };
            count += 1;
        }

        final_start_tile = match (from, start_from) {
            ('N', 'N') => '|',
            ('W', 'W') => '-',
            ('N', 'W') => 'L',
            ('N', 'E') => 'J',
            ('S', 'E') => '7',
            ('S', 'W') => 'F',
            _ => unreachable!("{from}, {start_from}"),
        };
        break;
    }

    grid[start.1][start.0] = final_start_tile;

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        let mut inside = false;
        let mut entering = ' ';
        for (x, c) in row.iter().enumerate() {
            if loop_tiles.contains(&(x, y)) {
                if *c == '|' {
                    inside = !inside;
                }

                if entering == 'L' && *c == '7' {
                    inside = !inside;
                }

                if entering == 'F' && *c == 'J' {
                    inside = !inside;
                }

                if *c != '-' {
                    entering = *c;
                }
            }

            println!("{x} {y} {inside}");

            if inside && !loop_tiles.contains(&(x, y)) {
                println!("counting");
                count += 1;
            }
        }
    }

    count
}

#[test]
pub fn sample() {
    assert_eq!(4, solve_first(include_str!("sample1.txt")));
    assert_eq!(10, solve_second(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(6860, solve_first(input));
    assert_eq!(343, solve_second(input));
}

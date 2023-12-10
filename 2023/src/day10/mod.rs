use std::collections::HashSet;

fn find_loop(input: &str) -> (HashSet<(usize, usize)>, Vec<Vec<char>>) {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), c)))
        .find(|&(_, &c)| c == 'S')
        .map(|((x, y), _)| (x, y))
        .expect("no start tile found");

    let loop_tiles = [
        ('|', (0, -1)),
        ('-', (1, 0)),
        ('L', (-1, 0)),
        ('J', (1, 0)),
        ('7', (1, 0)),
        ('F', (-1, 0)),
    ]
    .into_iter()
    .find_map(|(tile, (mut dx, mut dy))| {
        grid[start.1][start.0] = tile;

        let mut current = start;
        let mut loop_tiles = HashSet::new();
        loop {
            (dx, dy) = match grid.get(current.1).and_then(|x| x.get(current.0)) {
                Some('|') if dx == 0 => (0, dy),
                Some('-') if dy == 0 => (dx, 0),
                Some('L') if dx < 0 || dy > 0 => (dy, dx),
                Some('J') if dx > 0 || dy > 0 => (-dy, -dx),
                Some('7') if dx > 0 || dy < 0 => (dy, dx),
                Some('F') if dx < 0 || dy < 0 => (-dy, -dx),
                _ => break None,
            };

            current.0 = current.0.checked_add_signed(dx)?;
            current.1 = current.1.checked_add_signed(dy)?;

            loop_tiles.insert(current);

            if current == start {
                break Some(loop_tiles);
            }
        }
    })
    .expect("no valid loop found");

    (loop_tiles, grid)
}

pub fn solve_first(input: &str) -> usize {
    find_loop(input).0.len() / 2
}

pub fn solve_second(input: &str) -> usize {
    let (loop_tiles, grid) = find_loop(input);

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        let mut inside = false;
        let mut last = ' ';
        for (x, c) in row.iter().enumerate() {
            if loop_tiles.contains(&(x, y)) {
                if *c == '|' || (last == 'L' && *c == '7') || (last == 'F' && *c == 'J') {
                    inside = !inside; // "|", "L---7" and "F---J" change parity
                } else if *c != '-' {
                    last = *c;
                }
            } else if inside {
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

use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Elf,
}

fn print_board(board: &Vec<Vec<Tile>>) {
    for row in board {
        println!(
            "{}",
            row.iter()
                .map(|x| match x {
                    Tile::Empty => '.',
                    Tile::Elf => '#',
                })
                .join("")
        );
    }
}

pub fn solve(input: &str, round_limit: Option<usize>) -> usize {
    let mut board = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|x| match x {
                    b'.' => Tile::Empty,
                    b'#' => Tile::Elf,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut directions = vec![
        [(0, -1), (-1, -1), (1, -1)],
        [(0, 1), (-1, 1), (1, 1)],
        [(-1, 0), (-1, -1), (-1, 1)],
        [(1, 0), (1, -1), (1, 1)],
    ];

    let mut proposed = Vec::new();
    for i in 0.. {
        if let Some(round_limit) = round_limit {
            if i == round_limit {
                break;
            }
        }
        // print_board(&board);
        // println!("----------------------");

        proposed.clear();

        for (x, y) in board
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, v)| (y, x, v)))
            .filter(|(_, _, v)| **v == Tile::Elf)
            .map(|(y, x, _)| (x, y))
        {
            let (x, y) = (x as isize, y as isize);

            let mut elf = false;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if !(dx == 0 && dy == 0)
                        && *board
                            .get((y + dy) as usize)
                            .and_then(|row| row.get((x + dx) as usize))
                            .unwrap_or(&Tile::Empty)
                            == Tile::Elf
                    {
                        elf = true;
                    }
                }
            }
            if !elf {
                continue;
            }

            'outer: for direction in &directions {
                for (nx, ny) in direction.map(|(dx, dy)| (x + dx, y + dy)) {
                    if *board
                        .get(ny as usize)
                        .and_then(|row| row.get(nx as usize))
                        .unwrap_or(&Tile::Empty)
                        != Tile::Empty
                    {
                        continue 'outer;
                    }
                }

                proposed.push(((x, y), (x + direction[0].0, y + direction[0].1)));
                break;
            }
        }

        for (k, _) in proposed
            .iter()
            .map(|(_, to)| *to)
            .counts()
            .iter()
            .filter(|(_, v)| **v > 1)
        {
            for i in (0..proposed.len()).rev() {
                if proposed[i].1 == *k {
                    proposed.swap_remove(i);
                }
            }
        }

        if round_limit.is_none() && proposed.is_empty() {
            return i + 1;
        }

        let mut extend_left = false;
        let mut extend_right = false;
        let mut extend_down = false;
        let mut extend_up = false;
        for (_, (x, y)) in &proposed {
            if *x < 0 {
                extend_left = true;
            }
            if *x >= board[0].len() as isize {
                extend_right = true;
            }
            if *y < 0 {
                extend_up = true;
            }
            if *y >= board.len() as isize {
                extend_down = true;
            }
        }

        for row in board.iter_mut() {
            if extend_left {
                row.insert(0, Tile::Empty);
            }
            if extend_right {
                row.push(Tile::Empty);
            }
        }
        if extend_up {
            board.insert(0, vec![Tile::Empty; board[0].len()]);
        }
        if extend_down {
            board.push(vec![Tile::Empty; board[0].len()]);
        }

        for ((mut from_x, mut from_y), (mut to_x, mut to_y)) in &proposed {
            if extend_left {
                from_x += 1;
                to_x += 1;
            }
            if extend_up {
                from_y += 1;
                to_y += 1;
            }
            board[from_y as usize][from_x as usize] = Tile::Empty;
            board[to_y as usize][to_x as usize] = Tile::Elf;
        }

        directions.rotate_left(1);
    }

    board
        .into_iter()
        .flat_map(|x| x.into_iter())
        .filter(|&x| x == Tile::Empty)
        .count()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(110, solve(sample, Some(10)));
    assert_eq!(20, solve(sample, None));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(4052, solve(input, Some(10)));
    assert_eq!(978, solve(input, None));
}

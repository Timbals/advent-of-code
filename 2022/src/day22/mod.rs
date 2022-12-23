use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open,
    Wall,
    Wrap,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn movement(self) -> (isize, isize) {
        match self {
            Facing::Right => (1, 0),
            Facing::Down => (0, 1),
            Facing::Left => (-1, 0),
            Facing::Up => (0, -1),
        }
    }

    fn left(self) -> Self {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }

    fn right(self) -> Self {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
}

pub fn solve_first(input: &str) -> isize {
    let board = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|x| match x {
                    b'.' => Tile::Open,
                    b'#' => Tile::Wall,
                    b' ' => Tile::Wrap,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let mut numbers = input
        .lines()
        .last()
        .unwrap()
        .split(['R', 'L'])
        .map(|x| x.parse::<usize>().unwrap());
    let mut letters = input
        .lines()
        .last()
        .unwrap()
        .as_bytes()
        .iter()
        .filter(|&&x| x == b'L' || x == b'R');

    let start_x = board[0].iter().position(|&x| x == Tile::Open).unwrap() as isize;
    let (mut x, mut y, mut facing) = (start_x, 0_isize, Facing::Right);

    loop {
        for _ in 0..numbers.next().unwrap() {
            let mut next = (
                (x + facing.movement().0).rem_euclid(board[y as usize].len() as isize),
                (y + facing.movement().1).rem_euclid(board.len() as isize),
            );
            // TODO needs padded input
            while board[next.1 as usize][next.0 as usize] == Tile::Wrap {
                next.0 = (next.0 + facing.movement().0)
                    .rem_euclid(board[next.1 as usize].len() as isize);
                next.1 = (next.1 + facing.movement().1).rem_euclid(board.len() as isize);
            }

            match board[next.1 as usize][next.0 as usize] {
                Tile::Open => {
                    (x, y) = next;
                }
                Tile::Wall => {
                    break;
                }
                Tile::Wrap => unreachable!(),
            }
        }

        match letters.next() {
            Some(b'L') => facing = facing.left(),
            Some(b'R') => facing = facing.right(),
            None => break,
            _ => unreachable!(),
        }
    }

    1000 * (y + 1) + 4 * (x + 1) + facing as isize
}

struct Face {
    offset: (isize, isize),
    tiles: Vec<Vec<Tile>>,
    seams: [(usize, Facing); 4], // (index of other face, flip_x, flip_y) indexed by `Facing`
}

pub fn solve_second(input: &str) -> isize {
    let n = 50;
    let lines = input.lines().map(|line| line.as_bytes()).collect_vec();

    let faces = [
        lines[0..n]
            .iter()
            .map(|x| x[n..2 * n].to_vec())
            .collect_vec(),
        lines[0..n]
            .iter()
            .map(|x| x[2 * n..3 * n].to_vec())
            .collect_vec(),
        lines[n..2 * n]
            .iter()
            .map(|x| x[n..2 * n].to_vec())
            .collect_vec(),
        lines[2 * n..3 * n]
            .iter()
            .map(|x| x[0..n].to_vec())
            .collect_vec(),
        lines[2 * n..3 * n]
            .iter()
            .map(|x| x[n..2 * n].to_vec())
            .collect_vec(),
        lines[3 * n..4 * n]
            .iter()
            .map(|x| x[0..n].to_vec())
            .collect_vec(),
    ]
    .into_iter()
    .map(|x| {
        x.into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|x| match x {
                        b'.' => Tile::Open,
                        b'#' => Tile::Wall,
                        _ => unreachable!(),
                    })
                    .collect_vec()
            })
            .collect_vec()
    })
    .collect_vec();

    let n = n as isize;

    let faces = [
        Face {
            offset: (n, 0),
            tiles: faces[0].clone(),
            seams: [
                (1, Facing::Right),
                (2, Facing::Down),
                (3, Facing::Right),
                (5, Facing::Right),
            ],
        },
        Face {
            offset: (2 * n, 0),
            tiles: faces[1].clone(),
            seams: [
                (4, Facing::Left),
                (2, Facing::Left),
                (0, Facing::Left),
                (5, Facing::Up),
            ],
        },
        Face {
            offset: (n, n),
            tiles: faces[2].clone(),
            seams: [
                (1, Facing::Up),
                (4, Facing::Down),
                (3, Facing::Down),
                (0, Facing::Up),
            ],
        },
        Face {
            offset: (0, 2 * n),
            tiles: faces[3].clone(),
            seams: [
                (4, Facing::Right),
                (5, Facing::Down),
                (0, Facing::Right),
                (2, Facing::Right),
            ],
        },
        Face {
            offset: (n, 2 * n),
            tiles: faces[4].clone(),
            seams: [
                (1, Facing::Left),
                (5, Facing::Left),
                (3, Facing::Left),
                (2, Facing::Up),
            ],
        },
        Face {
            offset: (0, 3 * n),
            tiles: faces[5].clone(),
            seams: [
                (4, Facing::Up),
                (1, Facing::Down),
                (0, Facing::Down),
                (3, Facing::Up),
            ],
        },
    ];

    let mut numbers = input
        .lines()
        .last()
        .unwrap()
        .split(['R', 'L'])
        .map(|x| x.parse::<usize>().unwrap());
    let mut letters = input
        .lines()
        .last()
        .unwrap()
        .as_bytes()
        .iter()
        .filter(|&&x| x == b'L' || x == b'R');

    let start_x = faces[0].tiles[0]
        .iter()
        .position(|&x| x == Tile::Open)
        .unwrap() as isize;
    let (mut x, mut y, mut face, mut facing) = (start_x, 0_isize, 0_usize, Facing::Right);

    loop {
        for _ in 0..numbers.next().unwrap() {
            let mut next = (
                (x + facing.movement().0),
                (y + facing.movement().1),
                face,
                facing,
            );
            if next.0 < 0 || n <= next.0 || next.1 < 0 || n <= next.1 {
                let seam = faces[face].seams[facing as usize];

                match facing {
                    Facing::Right => match seam.1 {
                        Facing::Right => next.0 = 0,
                        Facing::Down => unreachable!(),
                        Facing::Left => {
                            next.0 = n - 1;
                            next.1 = n - 1 - y;
                        }
                        Facing::Up => {
                            next.0 = y;
                            next.1 = n - 1;
                        }
                    },
                    Facing::Down => match seam.1 {
                        Facing::Right => unreachable!(),
                        Facing::Down => next.1 = 0,
                        Facing::Left => {
                            next.0 = n - 1;
                            next.1 = x;
                        }
                        Facing::Up => unreachable!(),
                    },
                    Facing::Left => match seam.1 {
                        Facing::Right => {
                            next.0 = 0;
                            next.1 = n - 1 - y;
                        }
                        Facing::Down => {
                            next.0 = y;
                            next.1 = 0;
                        }
                        Facing::Left => next.0 = n - 1,
                        Facing::Up => unreachable!(),
                    },
                    Facing::Up => match seam.1 {
                        Facing::Right => {
                            next.0 = 0;
                            next.1 = x;
                        }
                        Facing::Down => unreachable!(),
                        Facing::Left => unreachable!(),
                        Facing::Up => next.1 = n - 1,
                    },
                }

                next.2 = seam.0;
                next.3 = seam.1;
            }

            match faces[next.2].tiles[next.1 as usize][next.0 as usize] {
                Tile::Open => {
                    (x, y, face, facing) = next;
                }
                Tile::Wall => {
                    break;
                }
                Tile::Wrap => unreachable!(),
            }
        }

        match letters.next() {
            Some(b'L') => facing = facing.left(),
            Some(b'R') => facing = facing.right(),
            None => break,
            _ => unreachable!(),
        }
    }

    1000 * (y + faces[face].offset.1 + 1) + 4 * (x + faces[face].offset.0 + 1) + facing as isize
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(6032, solve_first(sample));
    assert_eq!(5031, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(186128, solve_first(input));
    assert_eq!(34426, solve_second(input));
}

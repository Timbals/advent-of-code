use std::collections::VecDeque;

pub fn solve_first(input: &str) -> usize {
    let mut files = VecDeque::new();
    let mut free = VecDeque::new();
    let mut file = true;
    for size in input.chars().map(|x| x.to_digit(10).unwrap() as usize) {
        if file {
            files.push_back((size, files.len()));
        } else {
            free.push_back(size);
        }
        file = !file;
    }

    let mut result = 0;
    let mut position = 0;
    loop {
        let Some((file_size, file_id)) = files.pop_front() else {
            break;
        };
        result += file_id * (position..position + file_size).sum::<usize>();
        position += file_size;

        if let Some(space) = free.pop_front() {
            for _ in 0..space {
                let Some((file_size, file_id)) = files.back_mut() else {
                    break;
                };
                result += position * *file_id;
                *file_size -= 1;
                position += 1;
                if *file_size == 0 {
                    files.pop_back();
                }
            }
        }
    }

    result
}

pub fn solve_second(input: &str) -> usize {
    enum Block {
        File(usize, usize), // (size, id)
        Free(usize),        // (size)
    }

    let mut blocks = Vec::new();
    let mut file_sizes = Vec::new();
    let mut file = true;
    for size in input.chars().map(|x| x.to_digit(10).unwrap() as usize) {
        if file {
            blocks.push(Block::File(size, file_sizes.len()));
            file_sizes.push(size);
        } else {
            blocks.push(Block::Free(size));
        }
        file = !file;
    }

    for (file_id, file_size) in file_sizes.into_iter().enumerate().rev() {
        let mut free_space_id = None;
        let mut file_block_id = None;
        for (block_id, block) in blocks.iter().enumerate() {
            match block {
                Block::Free(size) if *size >= file_size && free_space_id.is_none() => {
                    free_space_id = Some(block_id)
                }
                Block::File(_, x) if *x == file_id => {
                    file_block_id = Some(block_id);
                    break;
                }
                _ => {}
            }
        }
        let file_block_id = file_block_id.unwrap();
        if let Some(free_space_id) = free_space_id {
            // works without merging adjacent free blocks
            // because every file block to the right was already moved,
            // but the empty space matters for the final position calculation
            let file_block = std::mem::replace(&mut blocks[file_block_id], Block::Free(file_size));
            let Block::Free(space) = &mut blocks[free_space_id] else { unreachable!() };
            *space -= file_size;
            blocks.insert(free_space_id, file_block);
        }
    }

    let mut result = 0;
    let mut position = 0;
    for block in blocks {
        match block {
            Block::File(space, id) => {
                result += id * (position..position + space).sum::<usize>();
                position += space;
            }
            Block::Free(space) => position += space,
        }
    }

    result
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(1928, solve_first(sample));
    assert_eq!(2858, solve_second(sample));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(6349606724455, solve_first(input));
    assert_eq!(6376648986651, solve_second(input));
}

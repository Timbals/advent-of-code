use itertools::Itertools;

fn combo(x: u64, a: u64, b: u64, c: u64) -> u64 {
    match x {
        0..4 => x,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    }
}

pub fn solve_first(input: &str) -> String {
    let mut lines = input.lines();
    let mut a = lines.next().unwrap().strip_prefix("Register A: ").unwrap().parse::<u64>().unwrap();
    let mut b = lines.next().unwrap().strip_prefix("Register B: ").unwrap().parse::<u64>().unwrap();
    let mut c = lines.next().unwrap().strip_prefix("Register C: ").unwrap().parse::<u64>().unwrap();
    lines.next().unwrap();
    let program = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .tuples()
        .collect::<Box<[(_, _)]>>();

    let mut output = Vec::new();

    let mut pc = 0;
    while pc < program.len() {
        let (operator, operand) = program[pc];
        let mut next_pc = pc + 1;
        match operator {
            0 => a = a.checked_shr(combo(operand, a, b, c) as u32).unwrap_or(0), // adv
            1 => b ^= operand,                                                   // bxl
            2 => b = combo(operand, a, b, c) % 8,                                // bst
            3 if a == 0 => {}                                                    // jnz
            3 if a != 0 => next_pc = operand as usize,                           // jnz
            4 => b ^= c,                                                         // bxc
            5 => output.push(combo(operand, a, b, c) % 8),                       // out
            6 => b = a.checked_shr(combo(operand, a, b, c) as u32).unwrap_or(0), // bdv
            7 => c = a.checked_shr(combo(operand, a, b, c) as u32).unwrap_or(0), // cdv
            _ => unreachable!(),
        }
        pc = next_pc;
    }

    output.into_iter().join(",")
}

pub fn solve_second(input: &str) -> u64 {
    let mut lines = input.lines();
    let program = lines
        .nth(4)
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Box<[_]>>();

    // the program loops until a == 0
    assert_eq!(program[program.len() - 2], 3);
    assert_eq!(program[program.len() - 1], 0);
    // one output just before the loop condition
    assert_eq!(program.iter().tuples().position(|(op, _)| *op == 5), Some((program.len() - 4) / 2));
    // also assumes that the `b` and `c` registers don't need their value carried over between loops
    // also assumes that 3 bits of the `a` register get consumed (shifted out) every loop

    let mut possible_a = vec![0_u64];

    for &target in program.iter().rev() {
        let old_a = std::mem::take(&mut possible_a);
        for old_a in old_a {
            'outer: for i in 0..8 {
                let mut a = (old_a << 3) | i;
                let mut b = 0;
                let mut c = 0;
                for (&operator, &operand) in program.iter().tuples() {
                    match operator {
                        0 => a = a.checked_shr(combo(operand, a, b, c) as u32).unwrap_or(0), // adv
                        1 => b ^= operand,                                                   // bxl
                        2 => b = combo(operand, a, b, c) % 8,                                // bst
                        4 => b ^= c,                                                         // bxc
                        5 => {
                            if combo(operand, a, b, c) % 8 != target {
                                continue 'outer;
                            }
                        } // out
                        6 => b = a.checked_shr(combo(operand, a, b, c) as u32).unwrap_or(0), // bdv
                        7 => c = a.checked_shr(combo(operand, a, b, c) as u32).unwrap_or(0), // cdv
                        _ => {}
                    }
                }

                possible_a.push((old_a << 3) | i);
            }
        }
    }

    possible_a.into_iter().min().unwrap()
}

#[test]
pub fn sample() {
    assert_eq!("4,6,3,5,6,3,5,2,1,0", solve_first(include_str!("sample.txt")));
    assert_eq!(117440, solve_second(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!("4,0,4,7,1,2,7,1,6", solve_first(input));
    assert_eq!(202322348616234, solve_second(input));
}

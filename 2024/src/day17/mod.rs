use itertools::Itertools;

pub fn solve_first(input: &str) -> String {
    let mut lines = input.lines();
    let mut a =
        lines.next().unwrap().strip_prefix("Register A: ").unwrap().parse::<usize>().unwrap();
    let mut b =
        lines.next().unwrap().strip_prefix("Register B: ").unwrap().parse::<usize>().unwrap();
    let mut c =
        lines.next().unwrap().strip_prefix("Register C: ").unwrap().parse::<usize>().unwrap();
    lines.next().unwrap();
    let program = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .tuples()
        .collect::<Box<[(_, _)]>>();

    let combo = |x, a, b, c| match x {
        0..4 => x,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    };

    let mut output = Vec::new();

    let mut pc = 0;
    while pc < program.len() {
        let (operator, operand) = program[pc];
        let mut next_pc = pc + 1;
        match operator {
            0 => a /= 2_usize.pow(combo(operand, a, b, c) as u32), // adv
            1 => b ^= operand,                                     // bxl
            2 => b = combo(operand, a, b, c) % 8,                  // bst
            3 if a == 0 => {}                                      // jnz
            3 if a != 0 => next_pc = operand,                      // jnz
            4 => b ^= c,                                           // bxc
            5 => output.push(combo(operand, a, b, c) % 8),         // out
            6 => b = a / 2_usize.pow(combo(operand, a, b, c) as u32), // bdv
            7 => c = a.checked_shr(combo(operand, a, b, c) as u32).unwrap_or(0), // cdv
            _ => unreachable!(),
        }
        pc = next_pc;
    }

    output.into_iter().join(",")
}

pub fn solve_second(input: &str) -> usize {
    let mut lines = input.lines();
    lines.next().unwrap();
    let initial_b =
        lines.next().unwrap().strip_prefix("Register B: ").unwrap().parse::<usize>().unwrap();
    let initial_c =
        lines.next().unwrap().strip_prefix("Register C: ").unwrap().parse::<usize>().unwrap();
    lines.next().unwrap();
    let program_flat = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Box<[_]>>();
    let program = program_flat.iter().copied().tuples().collect::<Box<[(_, _)]>>();

    let combo = |x, a, b, c| match x {
        0..4 => x,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    };

    let mut initial_a = 0;
    'outer: loop {
        let mut expected_output = program_flat.iter().copied().peekable();
        let mut a = initial_a;
        initial_a += 1;
        let mut b = initial_b;
        let mut c = initial_c;

        let mut counter = 0;
        let mut pc = 0;
        while pc < program.len() {
            counter += 1;
            if counter > 100_000 {
                continue 'outer;
            }

            let (operator, operand) = program[pc];
            let mut next_pc = pc + 1;
            match operator {
                0 => a /= 2_usize.pow(combo(operand, a, b, c) as u32), // adv
                1 => b ^= operand,                                     // bxl
                2 => b = combo(operand, a, b, c) % 8,                  // bst
                3 if a == 0 => {}                                      // jnz
                3 if a != 0 => next_pc = operand,                      // jnz
                4 => b ^= c,                                           // bxc
                5 => {
                    // out
                    if Some(combo(operand, a, b, c) % 8) == expected_output.next() {
                        if expected_output.peek().is_none() {
                            return initial_a - 1;
                        }
                    } else {
                        continue 'outer;
                    }
                }
                6 => b = a / 2_usize.pow(combo(operand, a, b, c) as u32), // bdv
                7 => c = a.checked_shr(combo(operand, a, b, c) as u32).unwrap_or(0), // cdv
                _ => unreachable!(),
            }
            pc = next_pc;
        }
    }
}

// pub fn solve_second_rev(input: &str) -> u64 {
//     let mut lines = input.lines();
//     lines.next().unwrap();
//     let mut initial_b =
//         lines.next().unwrap().strip_prefix("Register B: ").unwrap().parse::<usize>().unwrap();
//     let mut initial_c =
//         lines.next().unwrap().strip_prefix("Register C: ").unwrap().parse::<usize>().unwrap();
//     lines.next().unwrap();
//     let program_flat = lines
//         .next()
//         .unwrap()
//         .strip_prefix("Program: ")
//         .unwrap()
//         .split(',')
//         .map(|x| x.parse::<usize>().unwrap())
//         .collect::<Box<[_]>>();
//     let program = program_flat.iter().copied().tuples().collect::<Box<[(_, _)]>>();
//     let mut output = program_flat.iter().rev();
//
//     #[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq)]
//     enum Bit {
//         Unknown,
//         Zero,
//         One,
//         /// (var_index (0=a, 1=b, 2=c), bit_index, flipped)
//         Dependent(u8, usize, bool),
//     }
//
//     // impl PartialEq for Bit {
//     //     fn eq(&self, other: &Self) -> bool {
//     //         if let Bit::Unknown = self {
//     //             true
//     //         } else if let Bit::Unknown = other {
//     //             true
//     //         } else {
//     //             match (self, other) {
//     //                 (Bit::Zero, Bit::Zero) => true,
//     //                 (Bit::One, Bit::One) => true,
//     //                 (
//     //                     Bit::Dependent(a_var, a_index, a_flipped),
//     //                     Bit::Dependent(b_var, b_index, b_flipped),
//     //                 ) if *a_var == *b_var && *a_index == *b_index && *a_flipped == *b_flipped => {
//     //                     true
//     //                 }
//     //                 _ => false,
//     //             }
//     //         }
//     //     }
//     // }
//
//     impl Debug for Bit {
//         fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//             match self {
//                 Bit::Unknown => write!(f, "?"),
//                 Bit::Zero => write!(f, "0"),
//                 Bit::One => write!(f, "1"),
//                 Bit::Dependent(_, index, flip) => {
//                     if *flip {
//                         write!(f, "({index})")
//                     } else {
//                         write!(f, "[{index}]")
//                     }
//                 }
//             }
//         }
//     }
//
//     fn fmt_number(number: &[Bit; 64]) -> String {
//         let mut s = String::new();
//         for bit in number {
//             s.push_str(&format!("{bit:?}"));
//         }
//         s.push('\n');
//         s
//     }
//
//     impl Not for Bit {
//         type Output = Bit;
//
//         fn not(self) -> Self::Output {
//             match self {
//                 Bit::Unknown => Bit::Unknown,
//                 Bit::Zero => Bit::One,
//                 Bit::One => Bit::Zero,
//                 Bit::Dependent(var_index, bit_index, flipped) => {
//                     Bit::Dependent(var_index, bit_index, !flipped)
//                 }
//             }
//         }
//     }
//
//     let mut a = vec![[Bit::Zero; 64]];
//     let mut b = [Bit::Unknown; 64];
//     let mut c = [Bit::Unknown; 64];
//
//     loop {
//         a.sort_unstable();
//         a.dedup();
//
//         println!(
//             "{}{}{}",
//             a.iter().map(|a| fmt_number(a)).collect::<String>(),
//             fmt_number(&b),
//             fmt_number(&c)
//         );
//
//         // out(b % 8)
//         let out = *output.next().unwrap();
//
//         let val = if out & 0b001 != 0 { Bit::One } else { Bit::Zero };
//         if let Bit::Dependent(var_index, bit_index, flipped) = b[63] {
//             assert_eq!(var_index, 2);
//             c[bit_index] = if flipped { !val } else { val };
//             for a in &mut a {
//                 for a in a {
//                     if let Bit::Dependent(var_index_a, bit_index_a, flipped_a) = a {
//                         assert_eq!(*var_index_a, 2);
//                         if bit_index == *bit_index_a {
//                             *a = if *flipped_a { !c[bit_index] } else { c[bit_index] };
//                         }
//                     }
//                 }
//             }
//         }
//         b[63] = if out & 0b001 != 0 { Bit::One } else { Bit::Zero };
//
//         let val = if out & 0b010 != 0 { Bit::One } else { Bit::Zero };
//         if let Bit::Dependent(var_index, bit_index, flipped) = b[62] {
//             assert_eq!(var_index, 2);
//             c[bit_index] = if flipped { !val } else { val };
//             for a in &mut a {
//                 for a in a {
//                     if let Bit::Dependent(var_index_a, bit_index_a, flipped_a) = a {
//                         assert_eq!(*var_index_a, 2);
//                         if bit_index == *bit_index_a {
//                             *a = if *flipped_a { !c[bit_index] } else { c[bit_index] };
//                         }
//                     }
//                 }
//             }
//         }
//         b[62] = if out & 0b010 != 0 { Bit::One } else { Bit::Zero };
//
//         let val = if out & 0b100 != 0 { Bit::One } else { Bit::Zero };
//         if let Bit::Dependent(var_index, bit_index, flipped) = b[61] {
//             assert_eq!(var_index, 2);
//             c[bit_index] = if flipped { !val } else { val };
//             for a in &mut a {
//                 for a in a {
//                     if let Bit::Dependent(var_index_a, bit_index_a, flipped_a) = a {
//                         assert_eq!(*var_index_a, 2);
//                         if bit_index == *bit_index_a {
//                             *a = if *flipped_a { !c[bit_index] } else { c[bit_index] };
//                         }
//                     }
//                 }
//             }
//         }
//         b[61] = if out & 0b100 != 0 { Bit::One } else { Bit::Zero };
//
//         // b = b ^ c
//         b = std::array::from_fn(|i| match b[i] {
//             Bit::Zero => {
//                 if let Bit::Zero = c[i] {
//                     Bit::Zero
//                 } else if let Bit::One = c[i] {
//                     Bit::One
//                 } else {
//                     Bit::Dependent(2, i, false)
//                 }
//             }
//             Bit::One => {
//                 if let Bit::Zero = c[i] {
//                     Bit::One
//                 } else if let Bit::One = c[i] {
//                     Bit::Zero
//                 } else {
//                     Bit::Dependent(2, i, true)
//                 }
//             }
//             _ => Bit::Unknown,
//         });
//
//         // b = b ^ 0b100
//         b[61] = !b[61];
//
//         // a = a >> 3
//         for a in &mut a {
//             // TODO assert that the bits that get shifted out are zero(?) or unknown
//             *a = std::array::from_fn(|i| match i {
//                 61..64 => Bit::Unknown,
//                 _ => a[i + 3],
//             });
//         }
//
//         // c = a >> b
//
//         let old_a = std::mem::take(&mut a);
//         'shift_loop: for shift in 0..64 {
//             // TODO check if any bit with a value higher than 32 is set in b
//             // TODO check if `shift` is compatible with `b`
//             for (i, b) in b.iter().rev().enumerate() {
//                 if let Bit::One = b {
//                     if shift & (1 << i) == 0 {
//                         continue 'shift_loop;
//                     }
//                 } else if let Bit::Zero = b {
//                     if shift & (1 << i) != 0 {
//                         continue 'shift_loop;
//                     }
//                 }
//             }
//
//             for old_a in &old_a {
//                 a.push(std::array::from_fn(|i| {
//                     if 63 - i < shift {
//                         Bit::Unknown
//                     } else if let Bit::Zero = c[i + shift] {
//                         Bit::Zero
//                     } else if let Bit::One = c[i + shift] {
//                         Bit::One
//                     } else {
//                         Bit::Dependent(2, i + shift, false)
//                     }
//                 }));
//             }
//         }
//
//         // b = b ^ 1
//         b[63] = !b[63];
//
//         // b = a % 8
//         a.retain_mut(|a| {
//             let retain = (a[63] == b[63] || a[63] == Bit::Unknown || b[63] == Bit::Unknown)
//                 && (a[62] == b[62] || a[62] == Bit::Unknown || b[62] == Bit::Unknown)
//                 && (a[61] == b[61] || a[61] == Bit::Unknown || b[61] == Bit::Unknown);
//
//             a[63] = b[63];
//             a[62] = b[62];
//             a[61] = b[61];
//
//             retain
//         });
//     }
//
//     todo!()
// }

pub fn solve_second_hardcoded(input: &str) -> u64 {
    let mut lines = input.lines();
    let program = lines
        .nth(4)
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .rev();

    let mut a = vec![0];

    for target in program {
        let old_a = std::mem::take(&mut a);
        for old_a in old_a {
            for i in 0..8 {
                let a1 = (old_a << 3) | i;
                let b1 = a1 % 8;
                let b2 = b1 ^ 1;
                let c1 = a1 >> b2;
                let b3 = b2 ^ 4;
                let b4 = b3 ^ c1;

                if (b4 % 8) != target {
                    continue;
                }

                a.push(a1);
            }
        }
    }

    a.into_iter().min().unwrap()
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!("4,6,3,5,6,3,5,2,1,0", solve_first(sample));
    assert_eq!(117440, solve_second(include_str!("sample2.txt")));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!("4,0,4,7,1,2,7,1,6", solve_first(input));
    assert_eq!(202322348616234, solve_second_hardcoded(input));
}

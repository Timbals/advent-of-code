use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::iter::once;
use std::usize;

pub fn solve_first(input: &str) -> usize {
    let blueprints = input.lines().map(|line| {
        let mut line = line.split(' ');
        [
            line.nth(6).unwrap().parse::<usize>().unwrap(),
            line.nth(5).unwrap().parse::<usize>().unwrap(),
            line.nth(5).unwrap().parse::<usize>().unwrap(),
            line.nth(2).unwrap().parse::<usize>().unwrap(),
            line.nth(5).unwrap().parse::<usize>().unwrap(),
            line.nth(2).unwrap().parse::<usize>().unwrap(),
        ]
    });

    let mut score = 0;
    for (i, bp) in blueprints.enumerate() {
        let max_ore = bp[0] + bp[1] + bp[2] + bp[4];
        let max_clay = bp[3];
        let max_obsidian = bp[5];

        let mut bp_score = 0;

        let mut seen = HashSet::new();
        let mut queue = VecDeque::from_iter(once([24, 0, 0, 0, 0, 1, 0, 0, 0]));
        while !queue.is_empty() {
            let current = queue.pop_back().unwrap();

            if seen.contains(&current) {
                continue;
            } else {
                seen.insert(current);
            }

            let [remaining, mut ore, mut clay, mut obsidian, geodes, ore_robots, clay_robots, obsidian_robots, geodes_robots] =
                current;
            if remaining == 0 {
                if geodes > bp_score {
                    dbg!(current);
                    dbg!(geodes);
                }
                bp_score = max(bp_score, geodes);
                continue;
            }

            if ore > max_ore * remaining {
                ore = max_ore * remaining;
            }
            if clay > max_clay * remaining {
                clay = max_clay * remaining;
            }
            if obsidian > max_obsidian * remaining {
                obsidian = max_obsidian * remaining;
            }

            queue.push_back([
                remaining - 1,
                ore + ore_robots,
                clay + clay_robots,
                obsidian + obsidian_robots,
                geodes + geodes_robots,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geodes_robots,
            ]);
            if ore >= bp[0] && ore_robots < max_ore {
                queue.push_back([
                    remaining - 1,
                    ore + ore_robots - bp[0],
                    clay + clay_robots,
                    obsidian + obsidian_robots,
                    geodes + geodes_robots,
                    ore_robots + 1,
                    clay_robots,
                    obsidian_robots,
                    geodes_robots,
                ]);
            }
            if ore >= bp[1] && clay_robots < max_clay {
                queue.push_back([
                    remaining - 1,
                    ore + ore_robots - bp[1],
                    clay + clay_robots,
                    obsidian + obsidian_robots,
                    geodes + geodes_robots,
                    ore_robots,
                    clay_robots + 1,
                    obsidian_robots,
                    geodes_robots,
                ]);
            }
            if ore >= bp[2] && clay >= bp[3] && obsidian_robots < max_obsidian {
                queue.push_back([
                    remaining - 1,
                    ore + ore_robots - bp[2],
                    clay + clay_robots - bp[3],
                    obsidian + obsidian_robots,
                    geodes + geodes_robots,
                    ore_robots,
                    clay_robots,
                    obsidian_robots + 1,
                    geodes_robots,
                ]);
            }
            if ore >= bp[4] && obsidian >= bp[5] {
                queue.push_back([
                    remaining - 1,
                    ore + ore_robots - bp[4],
                    clay + clay_robots,
                    obsidian + obsidian_robots - bp[5],
                    geodes + geodes_robots,
                    ore_robots,
                    clay_robots,
                    obsidian_robots,
                    geodes_robots + 1,
                ]);
            }

            // for pay_ore in 0..=usize::from(ore >= bp[0] && ore_robots < max_ore) {
            //     let ore = ore - pay_ore * bp[0];
            //     for pay_clay in 0..=usize::from(ore >= bp[1] && clay_robots < max_clay) {
            //         let ore = ore - pay_clay * bp[1];
            //         for pay_obsidian in 0..=usize::from(
            //             ore >= bp[2] && clay >= bp[3] && obsidian_robots < max_obsidian,
            //         ) {
            //             let ore = ore - pay_obsidian * bp[2];
            //             let clay = clay - pay_obsidian * bp[3];
            //             for pay_geodes in 0..=usize::from(ore >= bp[4] && obsidian >= bp[5]) {
            //                 let ore = ore - pay_geodes * bp[4];
            //                 let obsidian = obsidian - pay_geodes * bp[5];
            //                 queue.push_back([
            //                     remaining - 1,
            //                     ore + ore_robots,
            //                     clay + clay_robots,
            //                     obsidian + obsidian_robots,
            //                     geodes + geodes_robots,
            //                     ore_robots + pay_ore,
            //                     clay_robots + pay_clay,
            //                     obsidian_robots + pay_obsidian,
            //                     geodes_robots + pay_geodes,
            //                 ]);
            //             }
            //         }
            //     }
            // }
        }

        dbg!("done");
        dbg!(bp_score);
        score += (i + 1) * bp_score;
    }

    score
}

pub fn solve_second(input: &str, part: bool) -> usize {
    let blueprints = input
        .lines()
        .take(if part { usize::MAX } else { 3 })
        .map(|line| line.split(' '))
        .map(|mut line| [6, 5, 5, 2, 5, 2].map(|i| line.nth(i).unwrap().parse::<usize>().unwrap()));

    let mut score = usize::from(!part);
    for (i, bp) in blueprints.enumerate() {
        fn search(
            bp: &[usize; 6],
            best: &mut usize,
            remaining: usize,
            [mut ores, robots]: [[usize; 4]; 2],
        ) {
            if ores[3] + remaining * robots[3] > *best {
                dbg!(ores[3] + remaining * robots[3]);
            }
            *best = max(*best, ores[3] + remaining * robots[3]);

            if remaining == 0 {
                return;
            }

            if ores[3] + (0..(robots[3] + remaining)).sum::<usize>()
                - (0..(robots[3].saturating_sub(1))).sum::<usize>()
                <= *best
            {
                return;
            }

            let max_ore = bp[0] + bp[1] + bp[2] + bp[4];
            let max_clay = bp[3];
            let max_obsidian = bp[5];

            if ores[0] > max_ore * remaining {
                ores[0] = max_ore * remaining;
            }
            if ores[1] > max_clay * remaining {
                ores[1] = max_clay * remaining;
            }
            if ores[2] > max_obsidian * remaining {
                ores[2] = max_obsidian * remaining;
            }

            for next_robot in 0..4 {
                let wait;
                let mut costs = [0; 4];

                match next_robot {
                    0 => {
                        // ore
                        if ores[0] >= bp[0] {
                            wait = 0;
                        } else {
                            wait = (bp[0] - ores[0] - 1) / robots[0] + 1;
                        }
                        costs[0] = bp[0];
                    }
                    1 => {
                        // clay
                        if ores[0] >= bp[1] {
                            wait = 0;
                        } else {
                            wait = (bp[1] - ores[0] - 1) / robots[0] + 1;
                        }
                        costs[0] = bp[1];
                    }
                    2 => {
                        // obsidian
                        if robots[1] == 0 {
                            continue;
                        }
                        if ores[0] >= bp[2] && ores[1] >= bp[3] {
                            wait = 0;
                        } else if ores[0] >= bp[2] {
                            wait = (bp[3] - ores[1] - 1) / robots[1] + 1;
                        } else if ores[1] >= bp[3] {
                            wait = (bp[2] - ores[0] - 1) / robots[0] + 1;
                        } else {
                            wait = max(
                                (bp[2] - ores[0] - 1) / robots[0] + 1,
                                (bp[3] - ores[1] - 1) / robots[1] + 1,
                            );
                        }
                        costs[0] = bp[2];
                        costs[1] = bp[3];
                    }
                    3 => {
                        // geode
                        if robots[2] == 0 {
                            continue;
                        }
                        if ores[0] >= bp[4] && ores[2] >= bp[5] {
                            wait = 0;
                        } else if ores[0] >= bp[4] {
                            wait = (bp[5] - ores[2] - 1) / robots[2] + 1;
                        } else if ores[2] >= bp[5] {
                            wait = (bp[4] - ores[0] - 1) / robots[0] + 1;
                        } else {
                            wait = max(
                                (bp[4] - ores[0] - 1) / robots[0] + 1,
                                (bp[5] - ores[2] - 1) / robots[2] + 1,
                            );
                        }
                        costs[0] = bp[4];
                        costs[2] = bp[5];
                    }
                    _ => unreachable!(),
                }

                let i = wait + 1;

                if i > remaining {
                    // building next_robot takes too long
                    continue;
                }

                let mut next_robots = robots;
                next_robots[next_robot] += 1;

                search(
                    bp,
                    best,
                    remaining - i,
                    [
                        [
                            ores[0] + i * robots[0] - costs[0],
                            ores[1] + i * robots[1] - costs[1],
                            ores[2] + i * robots[2] - costs[2],
                            ores[3] + i * robots[3] - costs[3],
                        ],
                        next_robots,
                    ],
                );
            }
        }

        let time = if part { 24 } else { 32 };
        let mut best = 0;
        search(&bp, &mut best, time, [[0, 0, 0, 0], [1, 0, 0, 0]]);
        if part {
            score += (i + 1) * dbg!(best);
        } else {
            score *= dbg!(best);
        }
    }

    score
}

#[test]
pub fn sample() {
    let sample = include_str!("sample.txt");
    assert_eq!(33, solve_second(sample, true));
    assert_eq!(56 * 62, solve_second(sample, false));
}

#[test]
pub fn input() {
    let input = include_str!("input.txt");
    assert_eq!(1382, solve_second(input, true));
    assert_eq!(31740, solve_second(input, false));
}

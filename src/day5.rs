use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input/5.txt");

fn get_input() -> Vec<((u64, u64), (u64, u64))> {
    let mut res = vec![];

    let process = |v: Option<&str>| v.unwrap().parse().unwrap();
    let next = |vs: &mut std::str::Split<[char; 2]>| process(vs.next());

    for line in INPUT.lines() {
        let mut vals = line.split([',', ' ']);

        res.push((
            (next(&mut vals), next(&mut vals)),
            (
                {
                    vals.next();
                    next(&mut vals)
                },
                next(&mut vals),
            ),
        ));
        debug_assert_eq!(vals.next(), None);
    }
    #[cfg(debug_assertions)]
    {
        let mut res_2 = vec![];
        for line in INPUT.lines() {
            let mut vals = line.split([',', ' ']);

            res_2.push((
                (process(vals.next()), process(vals.next())),
                (
                    {
                        vals.next();
                        process(vals.next())
                    },
                    process(vals.next()),
                ),
            ));
        }

        debug_assert_eq!(res, res_2);
    }

    res
}

// Tries: 2
fn part1() -> u64 {
    let input = get_input();

    let mut vent_map = HashMap::new();

    for line in input {
        let start = line.0;
        let end = line.1;

        let x1 = start.0.min(end.0);
        let y1 = start.1.min(end.1);

        let x2 = start.0.max(end.0);
        let y2 = start.1.max(end.1);

        let mut update_map = |x, y| {
            if let Some(count) = vent_map.get_mut(&(x, y)) {
                *count += 1;
            } else {
                vent_map.insert((x, y), 1);
            }
        };

        match (x1 == x2, y1 == y2) {
            (false, false) => continue,
            (true, false) => {
                for y in y1..=y2 {
                    update_map(x1, y);
                }
            }
            (false, true) => {
                for x in x1..=x2 {
                    update_map(x, y1);
                }
            }
            (true, true) => update_map(x1, y1),
        }
    }

    vent_map
        .values()
        .fold(0, |acc, v| if *v >= 2 { acc + 1 } else { acc })
}
// wrong: 534

pub fn part1_pretty() {
    println!("day 5 part 1: {}", part1());
}

fn part2() -> usize {
    let input = get_input();

    let mut vent_map = HashMap::new();

    for line in input {
        let start = line.0;
        let end = line.1;

        let x1 = start.0.min(end.0);
        let y1 = start.1.min(end.1);

        let x2 = start.0.max(end.0);
        let y2 = start.1.max(end.1);

        let mut update_map = |x, y| {
            if let Some(count) = vent_map.get_mut(&(x, y)) {
                *count += 1;
            } else {
                vent_map.insert((x, y), 1);
            }
        };

        match (x1 == x2, y1 == y2) {
            (false, false) => {
                // Check for diagonal lines
                if x2.abs_diff(x1) == y2.abs_diff(y1) {
                    let rev_y = (start.0 < end.0) != (start.1 < end.1);
                    let ys = if rev_y {
                        (y1..=y2).rev().collect_vec()
                    } else {
                        (y1..=y2).collect_vec()
                    };
                    for (x, y) in (x1..=x2).zip(ys) {
                        update_map(x, y);
                    }
                }
            }
            (true, false) => {
                for y in y1..=y2 {
                    update_map(x1, y);
                }
            }
            (false, true) => {
                for x in x1..=x2 {
                    update_map(x, y1);
                }
            }
            (true, true) => update_map(x1, y1),
        }
    }

    vent_map.values().filter(|v| **v >= 2).count()
}
// wrong: 21243

pub fn part2_pretty() {
    println!("day 5 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), 7318);
    }

    #[test]
    fn t_part2() {
        assert_eq!(part2(), 19939);
    }

    extern crate test;
    use test::{black_box, Bencher};

    #[bench]
    fn b_part1(b: &mut Bencher) {
        b.iter(|| part1());
    }

    // #[bench]
    // fn b_part2(b: &mut Bencher) {
    //     b.iter(|| part2());
    // }
}

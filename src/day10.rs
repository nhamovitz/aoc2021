use std::collections::HashMap;

use lazy_static::lazy_static;

const INPUT: &str = include_str!("input/10.txt");

lazy_static! {
    static ref ILLEGAL_POINT_VALS: HashMap<char, u64> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    static ref MATCHING_DELIMS: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    // use opening and not closing delimiters because that's what we have in the stack.
    // this avoids an unnecessary lookup in `MATCHING_DELIMS`
    static ref COMPLETING_POINT_VALS: HashMap<char, u64> =
        HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
}

// wrong: 802104
fn part1() -> u64 {
    let mut error_score = 0;

    for line in INPUT.lines() {
        if let Some(closer) = first_incorrect_closer(line) {
            error_score += ILLEGAL_POINT_VALS[&closer];
        }
    }

    error_score
}

fn first_incorrect_closer(s: &str) -> Option<char> {
    let mut stack = Vec::with_capacity(s.len() / 2);
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => match stack.pop() {
                Some(opener) => {
                    if c != MATCHING_DELIMS[&opener] {
                        return Some(c);
                    }
                }
                None => return Some(c),
            },
            _ => unreachable!(),
        }
    }

    None
}

pub fn part1_pretty() {
    println!("day 10 part 1: {}", part1());
}

// wrong: 2797840607
// Tries: 4
fn part2() -> u64 {
    let mut error_scores = vec![];

    for line in INPUT.lines() {
        if let Some(score) = balance_error(line) {
            error_scores.push(score);
        }
    }

    debug_assert_eq!(error_scores.len() % 2, 1);

    error_scores.sort_unstable();

    // NOTE: must figure out how tf integer division works. is this rounding,, down?
    let middle_idx = error_scores.len() / 2;
    error_scores[middle_idx]
}

fn balance_error(line: &str) -> Option<u64> {
    let mut stack = Vec::with_capacity(line.len() / 2);
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => match stack.pop() {
                Some(opener) => {
                    if c != MATCHING_DELIMS[&opener] {
                        return None;
                    }
                }
                None => return None,
            },
            _ => unreachable!(),
        }
    }

    Some(
        stack
            .iter()
            .rev()
            .map(|c| COMPLETING_POINT_VALS[c])
            .fold(0, |acc, points| acc * 5 + points),
    )
}

pub fn part2_pretty() {
    println!("day 10 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_ANS: u64 = 394647;
    const P2_ANS: u64 = 2380061249;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), P1_ANS);
    }

    #[test]
    fn t_part2() {
        assert_eq!(part2(), P2_ANS);
    }

    // extern crate test;
    // use test::{black_box, Bencher};

    // #[bench]
    // fn b_part1(b: &mut Bencher) {
    //     b.iter(|| part1());
    // }

    // #[bench]
    // fn b_part2(b: &mut Bencher) {
    //     b.iter(|| part2());
    // }
}

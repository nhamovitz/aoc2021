use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult, Position};

const INPUT: &str = include_str!("input/14.txt");

type Element = char;

fn parse_input() -> (Vec<Element>, HashMap<(char, char), char>) {
    let (polymer, insertions) = INPUT.split_once("\n\n").unwrap();
    let polymer = polymer.chars().collect();

    let insertions = insertions
        .lines()
        .map(|l| {
            let (input, output) = l.split_once(" -> ").unwrap();
            let input = input.chars().next_tuple().unwrap();
            let output = output.chars().next().unwrap();
            (input, output)
        })
        .collect();

    (polymer, insertions)
}

// Tries: 1 🎉
fn part1() -> usize {
    construct_polymer_simulate(10)
}

fn construct_polymer_simulate(days: u64) -> usize {
    let (mut polymer, insertion_mapping) = parse_input();

    for _d in 0..days {
        // dbg!(d);
        // dbg!(polymer.len());

        let mut new_polymer: Vec<char> = 
        // Vec::with_capacity(polymer.len() * 2 - 1);
        vec![];
        for pair in polymer.array_windows().with_position() {
            let (last, pair) = match pair {
                Position::First(&[p1, p2]) | Position::Middle(&[p1, p2]) => (false, (p1, p2)),
                Position::Last(&[p1, p2]) => (true, (p1, p2)),
                Position::Only(_) => unreachable!("that is very strange"),
            };

            new_polymer.push(pair.0);
            if let Some(to_insert) = insertion_mapping.get(&pair) {
                new_polymer.push(*to_insert);
            }

            if last {
                new_polymer.push(pair.1);
            }
        }

        polymer = new_polymer;
    }

    let counts = polymer.into_iter().counts();
    if let MinMaxResult::MinMax((c1, min), (c2, max)) = counts.iter().minmax_by_key(|(_element, count)| **count) {
        dbg!((c1, min), (c2, max));
        dbg!(*max - *min)
    } else {
        unreachable!("again, very odd")
    }
}

pub fn part1_pretty() {
    println!("day 14 part 1: {}", part1());
}

fn part2() -> usize {
    // return construct_polymer_simulate(40);
    
    // let mut pair_counts = polymer.iter().tuple_windows().counts();
    //     let mut new_pair_counts = HashMap::new();
  
    let (mut polymer, insertion_mapping) = parse_input();

    let mut len = polymer.len();
    for d in 0..40 {
        construct_polymer_simulate(dbg!(d));}
        0
}

pub fn part2_pretty() {
    println!("day 14 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_ANS: usize = 2745;
    // const P2_ANS: u64 = 2____;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), P1_ANS);
    }

    #[test]
    fn t_part2() {
        // assert_eq!(part2(), P2_ANS);
    }

    // extern crate test;
    // use test::{black_box, Bencher};

    // #[bench]
    // fn b_part1(b: &mut Bencher) {
    //     b.iter(part1);
    // }

    // #[bench]
    // fn b_part2(b: &mut Bencher) {
    //     b.iter(part2);
    // }
}

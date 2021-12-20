use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &'static str = include_str!("input/7.txt");

fn get_input() -> Vec<u16> {
    INPUT
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

// Tries: 2
fn part1() -> u64 {
    let mut input = get_input();
    input.sort_unstable();
    let input = input;

    let min = input[0];
    let max = *input.last().unwrap();

    let mut total_fuel_by_destination = HashMap::with_capacity((max - min + 1).into());

    for destination in min..=max {
        let fuel = input
            .iter()
            .map(|start| (start.abs_diff(destination) as u64))
            .sum();
        total_fuel_by_destination.insert(destination, fuel);
    }

    let mut fuels = total_fuel_by_destination.values().collect_vec();
    fuels.sort();
    *fuels[0]
}

pub fn part1_pretty() {
    println!("day 7 part 1: {}", part1());
}

fn part2() -> u64 {
    todo!()
}

pub fn part2_pretty() {
    println!("day 7 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_part1() {
        // assert_eq!(part1(), XXX);
    }

    #[test]
    fn t_part2() {
        // assert_eq!( , );
    }

    extern crate test;
    use test::{black_box, Bencher};

    #[bench]
    fn b_part1(b: &mut Bencher) {
        b.iter(|| part1());
    }

    #[bench]
    fn b_part2(b: &mut Bencher) {
        b.iter(|| part2());
    }
}

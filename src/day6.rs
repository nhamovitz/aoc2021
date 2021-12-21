use std::{fmt::Debug, str::FromStr};

use itertools::{repeat_n, Itertools};

const INPUT: &'static str = include_str!("input/6.txt");

#[derive(Clone)]
struct Lanternfish {
    timer: u8,
}

impl Default for Lanternfish {
    fn default() -> Self {
        Self { timer: 8 }
    }
}

impl Lanternfish {
    fn new_adult(timer: u8) -> Self {
        Self { timer }
    }

    fn update(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            true
        } else {
            self.timer -= 1;
            false
        }
    }
}

trait TimerVal: FromStr + Debug {}
impl TimerVal for u8 {}
impl TimerVal for u64 {}

fn get_input<Num: TimerVal>() -> Vec<Num>
where
    <Num as FromStr>::Err: Debug,
{
    INPUT
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

// Tries: 1
fn part1() -> u64 {
    run_simulation(80)
}

fn run_simulation(days: u64) -> u64 {
    let mut fishies = get_input()
        .iter()
        .map(|t| Lanternfish::new_adult(*t))
        .collect_vec();

    for d in 0..days {
        eprintln!("day {}", d);
        let mut to_add = 0;
        for fishie in &mut fishies {
            if fishie.update() {
                to_add += 1;
            }
        }
        fishies.extend(repeat_n(Lanternfish::default(), to_add));
        eprintln!("len {}", fishies.len());
    }

    fishies.len().try_into().unwrap()
}

pub fn part1_pretty() {
    println!("day 6 part 1: {}", part1());
}

fn elegantly(days: u64) -> u64 {
    let (weeks, rem) = num_integer::div_rem(days, 7);

    let mut fishies = get_input()
        .iter()
        .map(|t| Lanternfish::new_adult(*t))
        .collect_vec();

    let _ = produced_in_a_week_by(fishies);

    todo!()
}

fn part2() -> u64 {
    // u64: Day 188, len  4792662916; fails day 189
    //  u8: Day 212, len 38951363935; fails day 213
    run_simulation(256)

    // elegantly(256)
}

fn produced_in_a_week_by(mut fishies: Vec<Lanternfish>) -> Vec<Lanternfish> {
    let mut ret = Vec::<Lanternfish>::with_capacity(5);
    for _ in 0..7 {
        for fish in &mut ret {
            fish.update();
        }

        let mut to_add = 0;
        for fishie in &mut fishies {
            if fishie.update() {
                to_add += 1;
            }
        }

        ret.extend(repeat_n(Lanternfish::default(), to_add));
    }
    ret
}

pub fn part2_pretty() {
    println!("day 6 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), 365131);
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

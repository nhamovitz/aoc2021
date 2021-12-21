use itertools::{repeat_n, Itertools};
use std::collections::HashMap;

const INPUT: &str = include_str!("input/6.txt");
// const INPUT: &str = include_str!("input/6ex.txt");

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

fn get_input() -> Vec<u8> {
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
        if days > 100 {
            print!("Day {:03}: ", d);
        }

        let mut to_add = 0;
        for fishie in &mut fishies {
            if fishie.update() {
                to_add += 1;
            }
        }
        fishies.extend(repeat_n(Lanternfish::default(), to_add));

        if days > 100 {
            println!("len {:010}", fishies.len());
        }
    }

    fishies.len().try_into().unwrap()
}

pub fn part1_pretty() {
    println!("day 6 part 1: {}", part1());
}

#[allow(unused_variables)]
#[allow(unused_mut)]
// half-formed, buggy, in-progress
mod elegantly_first_try {
    use super::*;

    fn elegantly_(days: u64) -> u64 {
        let (weeks, rem) = num_integer::div_rem(days, 7);

        let mut fishies = get_input()
            .iter()
            .map(|t| Lanternfish::new_adult(*t))
            .collect_vec();

        let _ = produced_in_a_week_by(fishies);

        todo!()
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
}

type SchoolState = HashMap<u8, usize>;

fn next_state(state: SchoolState) -> SchoolState {
    let mut result = SchoolState::with_capacity(8);

    // otherwise the 0->6 and the 7->6 `insert`s overwrite each other
    let mut six_count = 0;

    for (timer, count) in state {
        match timer {
            0 => {
                result.insert(8, count);
                six_count += count;
            }
            7 => {
                six_count += count;
            }
            _ => {
                result.insert(timer - 1, count);
            }
        }
    }

    result.insert(6, six_count);

    result
}

fn elegantly(days: u64) -> usize {
    let initial_state = get_input().into_iter().counts();

    let mut state = initial_state;

    for _ in 0..days {
        state = next_state(state);
    }
    state.into_values().sum()
}

fn part2() -> u64 {
    // u64: Day 188, len  4792662916; fails day 189
    //  u8: Day 212, len 38951363935; fails day 213
    // run_simulation(256)

    elegantly(256).try_into().unwrap()
}

pub fn part2_pretty() {
    println!("day 6 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_ANS: u64 = 365131;
    const P2_ANS: u64 = 1650309278600;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), P1_ANS);
    }

    #[test]
    fn t_part2() {
        assert_eq!(part2(), P2_ANS);
    }

    #[test]
    fn t_elegantly_p1() {
        assert_eq!(elegantly(80) as u64, P1_ANS);
    }

    extern crate test;
    use test::{black_box, Bencher};

    #[bench]
    fn b_simulation(b: &mut Bencher) {
        let days = black_box(80);
        b.iter(|| run_simulation(days));
    }

    #[bench]
    fn b_schoolstate(b: &mut Bencher) {
        let days = black_box(80);
        b.iter(|| elegantly(days));
    }
}

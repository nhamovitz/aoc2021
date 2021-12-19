use itertools::{repeat_n, Itertools};

const INPUT: &'static str = include_str!("input/6.txt");

#[derive(Clone)]
struct Lanternfish {
    timer: u64,
}

impl Default for Lanternfish {
    fn default() -> Self {
        Self { timer: 8 }
    }
}

impl Lanternfish {
    fn new_adult(timer: u64) -> Self {
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

fn get_input() -> Vec<u64> {
    INPUT
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

// Tries: 1
fn part1() -> u64 {
    let mut fishies = get_input()
        .iter()
        .map(|t| Lanternfish::new_adult(*t))
        .collect_vec();

    for _ in 0..80 {
        let mut to_add = 0;
        for fishie in &mut fishies {
            if fishie.update() {
                to_add += 1;
            }
        }
        fishies.extend(repeat_n(Lanternfish::default(), to_add));
    }

    fishies.len().try_into().unwrap()
}

pub fn part1_pretty() {
    println!("day 6 part 1: {}", part1());
}

fn part2() -> u64 {
    todo!()
}

pub fn part2_pretty() {
    println!("day XXX part 2: {}", part2());
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

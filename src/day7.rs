use tap::Pipe;

const INPUT: &'static str = include_str!("input/7.txt");

fn get_input() -> Vec<u64> {
    INPUT
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

// Tries: 2
fn calculate(f: impl Fn(u64) -> u64, mut input: Vec<u64>) -> u64 {
    input.sort_unstable();
    let input = input;

    let min = input[0];
    let max = *input.last().unwrap();

    let mut fuels = Vec::with_capacity((max - min + 1).try_into().unwrap());

    for destination in min..=max {
        let fuel = input
            .iter()
            .map(|start| start.abs_diff(destination).pipe(&f))
            .sum();
        fuels.push(fuel);
    }

    fuels.sort_unstable();
    fuels[0]
}

const fn triangle(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

#[inline(always)]
const fn identity(n: u64) -> u64 {
    n
}

pub fn part1_pretty() {
    println!("day 7 part 1: {}", part1());
}

fn part1() -> u64 {
    calculate(identity, get_input())
}

// Tries: 1 😁
fn part2() -> u64 {
    calculate(triangle, get_input())
}

pub fn part2_pretty() {
    println!("day 7 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), 328318);
    }

    #[test]
    fn t_part2() {
        assert_eq!(part2(), 89791146);
    }

    extern crate test;
    use test::{black_box, Bencher};

    #[bench]
    fn b_part1(b: &mut Bencher) {
        let input = black_box(get_input());
        b.iter(|| calculate(identity, input.clone()));
    }

    #[bench]
    fn b_part2(b: &mut Bencher) {
        let input = black_box(get_input());
        b.iter(|| calculate(triangle, input.clone()));
    }
}

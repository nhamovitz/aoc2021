use itertools::Itertools;

const INPUT: &str = include_str!("input/8.txt");

fn get_input() -> Vec<&'static str> {
    INPUT
        .lines()
        .map(|line| line.split(" | ").last().unwrap())
        .collect()
}

// Tries: 1 😁
fn part1() -> u64 {
    let mut count = 0;
    get_input()
        .into_iter()
        .flat_map(|f| f.split_ascii_whitespace())
        .for_each(|digit| {
            if matches!(digit.len(), 2 | 4 | 3 | 7) {
                count += 1;
            }
        });
    count
}

pub fn part1_pretty() {
    println!("day 8 part 1: {}", part1())
}

fn part2() -> u64 {
    todo!()
}

pub fn part2_pretty() {
    println!("day 8 part 2: {}", part2());
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

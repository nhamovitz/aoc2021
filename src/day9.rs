use std::vec;

use ndarray::Array2;
use tap::Pipe;

const INPUT: &str = include_str!("input/9.txt");

fn get_input() -> Array2<u8> {
    let shape = INPUT
        .lines()
        .enumerate()
        .last()
        .unwrap()
        .pipe(|(c, l)| (c + 1, l.len()));

    Array2::from_shape_vec(
        shape,
        INPUT
            .lines()
            .flat_map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
            })
            .collect(),
    )
    .unwrap()
}

fn part1() -> u64 {
    let input = get_input();
    let (max_x, max_y) = input.dim();

    let mut risk_levels = vec![];

    'outer: for ((x, y), height) in input.indexed_iter() {
        macro_rules! get_adjacent {
            ($dim:ident, $max:ident) => {{
                let mut to_check = vec![];
                if $dim != 0 {
                    to_check.push($dim - 1);
                }
                if $dim != $max - 1 {
                    to_check.push($dim + 1);
                }
                to_check
            }};
        }
        let check_xs = get_adjacent!(x, max_x);
        let check_ys = get_adjacent!(y, max_y);

        for check_x in check_xs {
            for check_y in &check_ys {
                if *input.get((check_x, *check_y)).unwrap() < *height {
                    continue 'outer;
                }
            }
        }

        risk_levels.push((*height + 1) as u64);
    }

    risk_levels.iter().sum()
}
// wrong: 10045

pub fn part1_pretty() {
    println!("day 9 part 1: {}", part1());
}

fn part2() -> u64 {
    todo!()
}

pub fn part2_pretty() {
    println!("day 9 part 2: {}", part2());
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

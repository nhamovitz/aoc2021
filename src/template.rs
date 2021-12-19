const INPUT: &'static str = include_str!("input/xxx.txt");

fn part1() -> XXX {}

pub fn part1_pretty() {
    println!("day XXX part 1: {}", part1());
}

fn part2() -> XXX {}

pub fn part2_pretty() {
    println!("day XXX part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), XXX);
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

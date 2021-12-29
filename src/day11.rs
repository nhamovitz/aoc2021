use std::{collections::HashSet, vec};

use itertools::{iproduct, Itertools};
use ndarray::{Array2, ShapeBuilder};

type Coordinate = (usize, usize);

const INPUT: &str = include_str!("input/11.txt");
const INPUT_SIZE: Coordinate = (10, 10);

#[derive(Default, Debug)]
struct Octupus {
    energy: u8,
    index: Coordinate,
    adjacent: HashSet<Coordinate>,
}

mod cartesian {
    use super::*;
    fn cartesian_product_iproduct(a: Vec<usize>, b: Vec<usize>) -> HashSet<(usize, usize)> {
        iproduct!(a, b).collect()
    }

    fn cartesian_product_w_method(a: Vec<usize>, b: Vec<usize>) -> HashSet<(usize, usize)> {
        a.into_iter().cartesian_product(b.into_iter()).collect()
    }

    fn cartesian_product_for_loop(a: Vec<usize>, b: Vec<usize>) -> HashSet<(usize, usize)> {
        let mut res = HashSet::new();
        for x in a {
            for y in &b {
                res.insert((x, *y));
            }
        }
        res
    }

    pub(crate) fn cart_prod(a: Vec<usize>, b: Vec<usize>) -> HashSet<(usize, usize)> {
        let iprod = cartesian_product_iproduct(a.clone(), b.clone());

        #[cfg(debug_assertions)]
        {
            let metho = cartesian_product_w_method(a.clone(), b.clone());
            let loopf = cartesian_product_for_loop(a.clone(), b.clone());

            assert_eq!(iprod, metho);
            assert_eq!(iprod, loopf);
            assert_eq!(metho, loopf);
        }
        iprod
    }
}

impl Octupus {
    //.pipe(|(x, y)| (x - 1, y - 1)); // this doesn't work rip
    const MAX_INDEX: Coordinate = (INPUT_SIZE.0 - 1, INPUT_SIZE.1 - 1);
    const ENERGY_LIMIT: u8 = 9;

    fn new(energy: u32, index: Coordinate) -> Self {
        let adjacent = {
            macro_rules! get_adjacent {
                ($dim:expr, $max:expr) => {{
                    let mut to_check = vec![];
                    if $dim != 0 {
                        to_check.push($dim - 1);
                    }
                    to_check.push($dim);
                    if $dim != $max {
                        to_check.push($dim + 1);
                    }
                    to_check
                }};
            }

            let check_xs = get_adjacent!(index.0, Self::MAX_INDEX.0);
            let check_ys = get_adjacent!(index.1, Self::MAX_INDEX.1);

            let mut res = cartesian::cart_prod(check_xs, check_ys);
            let remove = res.remove(&index);
            debug_assert!(remove);
            res
        };

        Self {
            energy: energy.try_into().unwrap(),
            index,
            adjacent,
        }
    }

    fn increment(&mut self) -> bool {
        self.energy = self.energy.saturating_add(1);
        self.energy > Self::ENERGY_LIMIT
    }

    fn reset(&mut self) {
        if self.energy > Self::ENERGY_LIMIT {
            // debug_assert_eq!(self.energy, Self::ENERGY_LIMIT + 1, "{:?}", self);
            self.energy = 0;
        }
    }
}

#[derive(Default)]
struct Grid {
    grid: Array2<Octupus>,
}

impl Grid {
    fn new(grid: Array2<Octupus>) -> Self {
        Self { grid }
    }

    fn adjacent_to(&self, idx: Coordinate) -> HashSet<Coordinate> {
        self.grid.get(idx).unwrap().adjacent.clone()
    }

    fn step(&mut self) -> usize {
        let mut have_flashed = HashSet::new();
        let mut to_flash = HashSet::new();

        self.grid.map_inplace(|oct| {
            if oct.increment() {
                to_flash.insert(oct.index);
            }
        });

        while !to_flash.is_empty() {
            let mut to_flash_new = HashSet::new();
            let mut have_flashed_new = HashSet::new();

            for coord_to_flash in to_flash.difference(&have_flashed) {
                have_flashed_new.insert(*coord_to_flash);
                for adj_coord in self.adjacent_to(*coord_to_flash) {
                    let oct = self.grid.get_mut(adj_coord).unwrap();
                    if oct.increment() {
                        to_flash_new.insert(oct.index);
                    }
                }
            }

            to_flash = to_flash_new;
            have_flashed.extend(have_flashed_new.iter());

            // dbg!(&to_flash, &have_flashed);
        }

        // println!("no more to flash");

        self.grid.map_inplace(Octupus::reset);

        have_flashed.len()
    }
}

fn get_input() -> Grid {
    let mut entries = Vec::with_capacity(INPUT_SIZE.0 * INPUT_SIZE.1);

    for (y, line) in INPUT.lines().enumerate() {
        for (x, energy) in line.chars().enumerate() {
            entries.push(Octupus::new(energy.to_digit(10).unwrap(), (x, y)));
        }
    }
    Grid::new(Array2::from_shape_vec(INPUT_SIZE.f(), entries).unwrap())
}

// Tries: 1,~5
fn part1() -> u64 {
    let mut grid = get_input();

    // dbg!(&grid.grid.get((1, 2)).unwrap().energy);

    // let mut log = String::with_capacity(70 * 100);

    // for oct in grid.grid {
    //     writeln!(&mut log, "{:?}\n", oct).unwrap();
    // }
    // std::fs::write("src/debugging/11/coords_4ish.txt", log).unwrap();

    // return 0;

    //

    let mut flash_count = 0;
    for d in 0..100 {
        dbg!(d);
        flash_count += grid.step();
    }

    flash_count.try_into().unwrap()
}

pub fn part1_pretty() {
    println!("day 11 part 1: {}", part1());
}

fn part2() -> u64 {
    todo!()
}

pub fn part2_pretty() {
    println!("day 11 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::inconsistent_digit_grouping)]
    const P1_ANS: u64 = 1____;
    #[allow(clippy::inconsistent_digit_grouping)]
    const P2_ANS: u64 = 2____;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), P1_ANS);
    }

    #[test]
    fn t_part2() {
        // assert_eq!( , P2_ANS);
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

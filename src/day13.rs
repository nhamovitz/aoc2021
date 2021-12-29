use std::{array, collections::HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("input/13.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    fn folded(&self, fold: &Fold) -> Self {
        match fold {
            Fold::X(along) => {
                if self.x < *along {
                    *self
                } else if self.x > *along {
                    Self::new(self.x - (2 * (self.x - along)), self.y)
                } else {
                    unreachable!("there should be no points along the fold line");
                }
            }
            Fold::Y(along) => {
                if self.y < *along {
                    *self
                } else if self.y > *along {
                    Self::new(self.x, self.y - (2 * (self.y - along)))
                } else {
                    unreachable!("there should be no points along the fold line");
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(u64),
    Y(u64),
}

fn parse_input() -> (HashSet<Point>, Vec<Fold>) {
    let (points, folds) = INPUT.split_once("\n\n").unwrap();

    let points = points
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            Point { x, y }
        })
        .collect();

    let folds = folds
        .lines()
        .map(|l| {
            let fold = l.splitn(3, ' ').nth(2).unwrap();
            let letter = fold.chars().next().unwrap();
            let number = (&fold[2..]).parse().unwrap();
            if letter == 'x' {
                Fold::X(number)
            } else if letter == 'y' {
                Fold::Y(number)
            } else {
                unreachable!()
            }
        })
        .collect();

    (points, folds)
}

fn do_fold(points: impl IntoIterator<Item = Point>, fold: &Fold) -> HashSet<Point> {
    points
        .into_iter()
        .map(|p| p.folded(fold))
        .unique()
        .collect()
}

// Tries: 1
fn part1() -> usize {
    let (points, folds) = parse_input();

    let fold = &folds[0];

    do_fold(points, fold).len()
}

pub fn part1_pretty() {
    println!("day 13 part 1: {}", part1());
}

fn disp_points(points: impl IntoIterator<Item = Point>) {
    let mut buf = [['.'; 40]; 7];
    for point in points {
        let (x, y) = (point.x as usize, point.y as usize);
        buf[y][x] = '#';
    }
    for line in buf {
        for char in line {
            print!("{}", &char);
        }
        println!();
    }
}

fn part2() -> HashSet<Point> {
    let (mut points, folds) = parse_input();

    for fold in &folds {
        points = do_fold(points, fold);
    }
    points
}

pub fn part2_pretty() {
    println!("day 13 part 2:\n");
    disp_points(part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_ANS: usize = 631;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), P1_ANS);
    }

    #[test]
    fn t_part2() {
        assert_eq!(
            part2(),
            HashSet::from([
                Point { x: 33, y: 2 },
                Point { x: 6, y: 2 },
                Point { x: 13, y: 5 },
                Point { x: 1, y: 2 },
                Point { x: 28, y: 1 },
                Point { x: 36, y: 2 },
                Point { x: 25, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 30, y: 5 },
                Point { x: 2, y: 0 },
                Point { x: 7, y: 2 },
                Point { x: 16, y: 0 },
                Point { x: 2, y: 5 },
                Point { x: 0, y: 4 },
                Point { x: 37, y: 0 },
                Point { x: 8, y: 0 },
                Point { x: 28, y: 3 },
                Point { x: 18, y: 0 },
                Point { x: 30, y: 1 },
                Point { x: 10, y: 1 },
                Point { x: 37, y: 2 },
                Point { x: 27, y: 0 },
                Point { x: 22, y: 0 },
                Point { x: 10, y: 0 },
                Point { x: 28, y: 4 },
                Point { x: 23, y: 1 },
                Point { x: 30, y: 2 },
                Point { x: 5, y: 1 },
                Point { x: 10, y: 3 },
                Point { x: 23, y: 2 },
                Point { x: 0, y: 1 },
                Point { x: 15, y: 4 },
                Point { x: 10, y: 5 },
                Point { x: 5, y: 4 },
                Point { x: 26, y: 0 },
                Point { x: 35, y: 2 },
                Point { x: 5, y: 2 },
                Point { x: 15, y: 5 },
                Point { x: 25, y: 4 },
                Point { x: 26, y: 5 },
                Point { x: 10, y: 4 },
                Point { x: 0, y: 3 },
                Point { x: 3, y: 5 },
                Point { x: 1, y: 5 },
                Point { x: 35, y: 4 },
                Point { x: 23, y: 4 },
                Point { x: 22, y: 5 },
                Point { x: 7, y: 0 },
                Point { x: 25, y: 3 },
                Point { x: 25, y: 2 },
                Point { x: 15, y: 0 },
                Point { x: 17, y: 2 },
                Point { x: 35, y: 5 },
                Point { x: 30, y: 4 },
                Point { x: 31, y: 3 },
                Point { x: 5, y: 3 },
                Point { x: 11, y: 5 },
                Point { x: 2, y: 2 },
                Point { x: 23, y: 0 },
                Point { x: 32, y: 3 },
                Point { x: 3, y: 0 },
                Point { x: 15, y: 1 },
                Point { x: 33, y: 5 },
                Point { x: 6, y: 0 },
                Point { x: 17, y: 0 },
                Point { x: 21, y: 5 },
                Point { x: 38, y: 0 },
                Point { x: 12, y: 5 },
                Point { x: 27, y: 5 },
                Point { x: 5, y: 0 },
                Point { x: 30, y: 0 },
                Point { x: 36, y: 0 },
                Point { x: 5, y: 5 },
                Point { x: 32, y: 0 },
                Point { x: 0, y: 2 },
                Point { x: 33, y: 1 },
                Point { x: 20, y: 4 },
                Point { x: 23, y: 3 },
                Point { x: 27, y: 3 },
                Point { x: 35, y: 0 },
                Point { x: 35, y: 3 },
                Point { x: 0, y: 5 },
                Point { x: 31, y: 0 },
                Point { x: 15, y: 2 },
                Point { x: 15, y: 3 },
                Point { x: 30, y: 3 },
                Point { x: 32, y: 4 },
                Point { x: 0, y: 0 },
                Point { x: 10, y: 2 },
                Point { x: 35, y: 1 },
                Point { x: 16, y: 2 },
                Point { x: 28, y: 5 },
            ])
        );
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

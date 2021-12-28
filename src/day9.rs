use ndarray::Array2;
use tap::Pipe;

const INPUT: &str = include_str!("input/9.txt");

fn get_input() -> Array2<u8> {
    let shape = INPUT
        .lines()
        .enumerate()
        .last()
        .unwrap()
        .pipe(|(c, l)| (c + 1, l.len()))
        // .pipe(|(x, y)| (y, x))
        ;

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
    .reversed_axes()
}

// Tries: 4
fn part1() -> u64 {
    let input = get_input();

    let (max_x, max_y) = input
        .dim()
        // subtract 1 from each because dimensions are 1-indexed, and we want the highest possible index
        .pipe(|(x, y)| (x - 1, y - 1));

    let mut risk_levels = vec![];

    for ((x, y), height) in input.indexed_iter() {
        let adjacent = get_adjacent_indices(x, y, max_y, max_x);

        if adjacent
            .into_iter()
            .map(|coord| *height < *input.get(coord).unwrap())
            .all(|b| b)
        {
            risk_levels.push((*height + 1) as u64);
        }
    }

    risk_levels.iter().sum()
}

fn get_adjacent_indices(x: usize, y: usize, max_y: usize, max_x: usize) -> Vec<(usize, usize)> {
    let mut adjacent = vec![];
    if x == 0 {
        if y == 0 {
            adjacent.push((x, y + 1));
            adjacent.push((x + 1, y));
        } else if y == max_y {
            adjacent.push((x, y - 1));
            adjacent.push((x + 1, y));
        } else {
            adjacent.push((x, y - 1));
            adjacent.push((x, y + 1));
            adjacent.push((x + 1, y));
        }
    } else if x == max_x {
        if y == 0 {
            adjacent.push((x, y + 1));
            adjacent.push((x - 1, y));
        } else if y == max_y {
            adjacent.push((x, y - 1));
            adjacent.push((x - 1, y));
        } else {
            adjacent.push((x, y - 1));
            adjacent.push((x, y + 1));
            adjacent.push((x - 1, y));
        }
    } else {
        if y == 0 {
            adjacent.push((x - 1, y));
            adjacent.push((x + 1, y));
            adjacent.push((x, y + 1));
        } else if y == max_y {
            adjacent.push((x - 1, y));
            adjacent.push((x + 1, y));
            adjacent.push((x, y - 1));
        } else {
            adjacent.push((x, y - 1));
            adjacent.push((x, y + 1));
            adjacent.push((x + 1, y));
            adjacent.push((x - 1, y));
        }
    }
    adjacent
}
// wrong: 10045
// wrong:  1738

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
        assert_eq!(part1(), 588);
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

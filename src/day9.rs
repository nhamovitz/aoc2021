use ndarray::{Array2, ArrayView2, ArrayViewMut2};
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
    .reversed_axes()
}

// wrong: 10045
// wrong:  1738
// Tries: 4
fn part1() -> u64 {
    let input = get_input();

    let low_point_coords = low_points_of(input.view());

    low_point_coords
        .into_iter()
        .map(|c| (input[c] + 1) as u64)
        .sum()
}

fn max_indices(dim: (usize, usize)) -> (usize, usize) {
    // subtract 1 from each because dimensions are 1-indexed, and we want the highest possible index
    (dim.0 - 1, dim.1 - 1)
}

fn low_points_of(input: ArrayView2<u8>) -> Vec<(usize, usize)> {
    let (max_x, max_y) = max_indices(input.dim());

    let mut low_point_coords = vec![];

    for (coord, height) in input.indexed_iter() {
        let adjacent = get_adjacent_indices(coord.0, coord.1, max_y, max_x);

        if adjacent
            .into_iter()
            .map(|coord| *height < *input.get(coord).unwrap())
            .all(|b| b)
        {
            low_point_coords.push(coord);
        }
    }
    low_point_coords
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

pub fn part1_pretty() {
    println!("day 9 part 1: {}", part1());
}

struct Cell {
    height: u8,
    visited: bool,
}

impl Cell {
    fn new(height: u8) -> Self {
        Self {
            height,
            visited: false,
        }
    }
    fn visit(&mut self) {
        debug_assert!(!self.visited);
        self.visited = true;
    }
}

// Tries: 2
fn part2() -> u64 {
    let data = get_input();
    let low_points = low_points_of(data.view());

    let mut data = data.mapv(|height| Cell::new(height));

    let mut basin_sizes = vec![];

    for coord in low_points {
        basin_sizes.push(basin_size_around(coord, data.view_mut()));
    }

    basin_sizes.sort_unstable();
    basin_sizes.into_iter().rev().take(3).product()
}

fn basin_size_around(coord: (usize, usize), mut data: ArrayViewMut2<Cell>) -> u64 {
    let mut basin_size = 0;
    let cell = data.get_mut(coord).unwrap();

    if cell.visited || cell.height == 9 {
        return 0;
    }

    cell.visit();
    basin_size += 1;

    let (max_x, max_y) = max_indices(data.dim());
    for c in get_adjacent_indices(coord.0, coord.1, max_y, max_x) {
        basin_size += basin_size_around(c, data.view_mut());
    }

    basin_size
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
        assert_eq!(part2(), 964712);
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

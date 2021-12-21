use ndarray::{arr2, Array2, ArrayView1};
use std::{convert::Infallible, str::FromStr};

const INPUT: &str = include_str!("input/4.txt");

type BingoSquare = (u64, bool);

#[derive(PartialEq, Eq, Debug)]
struct Board {
    board: Array2<BingoSquare>,
}

impl Board {
    fn from_vec(board_str: &str) -> Self {
        let mut board = Vec::with_capacity(25);
        for row in board_str.lines() {
            for val in row.split_ascii_whitespace() {
                board.push((val.parse().unwrap(), false));
            }
        }

        Self {
            board: Array2::from_shape_vec((5, 5), board).unwrap(),
        }
    }

    fn from_arr(board_str: &str) -> Self {
        let mut board: [[BingoSquare; 5]; 5] = Default::default();
        for (r, row) in board_str.lines().enumerate() {
            for (c, val) in row.split_ascii_whitespace().enumerate() {
                board[r][c] = (val.parse().unwrap(), false);
            }
        }

        Self {
            board: arr2(&board),
        }
    }

    fn new(board_str: &str) -> Self {
        let r1 = Self::from_vec(board_str);
        let r2 = Self::from_arr(board_str);
        debug_assert_eq!(r1, r2);
        r2
    }

    fn has_bingo(&mut self) -> bool {
        for r in self.board.rows() {
            if Self::is_bingo(r) {
                return true;
            }
        }
        for c in self.board.columns() {
            if Self::is_bingo(c) {
                return true;
            }
        }
        false
    }

    fn is_bingo(lane: ArrayView1<BingoSquare>) -> bool {
        lane.iter().all(|(_, marked)| *marked)
    }

    fn mark_number(&mut self, n: u64) -> bool {
        if let Some((_, marked)) = self.board.iter_mut().find(|(num, _)| *num == n) {
            assert!(!*marked);
            *marked = true;
            true
        } else {
            false
        }
    }

    fn sum_of_unmarked(&self) -> u64 {
        self.board
            .iter()
            .filter(|(_, marked)| !*marked)
            .map(|(n, _)| n)
            .sum()
    }

    fn score(&self, draw: u64) -> u64 {
        self.sum_of_unmarked() * draw
    }
}

impl FromStr for Board {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_arr(s))
    }
}

fn p1_naive() -> u64 {
    let (draws, mut boards) = get_input();

    for (i, draw) in draws.iter().enumerate() {
        for board in &mut boards {
            board.mark_number(*draw);
            // Don't need to check for a completed board for the first 5
            if i > 5 {
                if board.has_bingo() {
                    return board.sum_of_unmarked() * draw;
                }
            }
        }
    }
    unreachable!()
}

// Tries: 2
fn part2() -> u64 {
    let (draws, mut boards) = get_input();

    let mut next_is_last_winner = false;
    let mut won_boards = 0;
    for draw in draws {
        for board in &mut boards {
            if !board.has_bingo() {
                board.mark_number(draw);
                if board.has_bingo() {
                    won_boards += 1;
                    if next_is_last_winner {
                        return board.score(draw);
                    }
                }
            }
        }

        if won_boards == boards.len() - 1 {
            next_is_last_winner = true;
        }
    }

    unreachable!()
}
// wrong: 12441

fn get_input() -> (Vec<u64>, Vec<Board>) {
    let mut input = INPUT.split_terminator("\n\n");
    let draws = input
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();
    let boards = input.map(Board::new).collect::<Vec<_>>();
    (draws, boards)
}

pub fn part1_pretty() {
    println!("day 4 part 1: {}", p1_naive());
}

pub fn part2_pretty() {
    println!("day 4 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::{black_box, Bencher};

    #[test]
    fn t_part1() {
        assert_eq!(p1_naive(), 44088);
    }

    #[test]
    fn t_part2() {
        assert_eq!(part2(), 23670);
    }

    fn get_board_str() -> &'static str {
        let mut input = INPUT.split_terminator("\n\n");
        input.next();
        input.next().unwrap()
    }

    #[bench]
    fn b_board_arr(b: &mut Bencher) {
        let s = get_board_str();
        b.iter(|| {
            let s = black_box(s);
            Board::from_arr(s)
        })
    }

    #[bench]
    fn b_board_vec(b: &mut Bencher) {
        let s = get_board_str();
        b.iter(|| {
            let s = black_box(s);
            Board::from_vec(s)
        })
    }

    #[bench]
    fn b_part2(b: &mut Bencher) {
        b.iter(|| part2());
    }

    #[bench]
    fn b_part1_naive(b: &mut Bencher) {
        b.iter(|| p1_naive());
    }
}

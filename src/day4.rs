use ndarray::{arr2, Array2};

const INPUT: &'static str = include_str!("input/4.txt");

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
                let val = (val.parse().unwrap(), false);
                board[r][c] = val;
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

    fn has_bingo(&self) -> bool {
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

    fn is_bingo(lane: ndarray::ArrayView1<BingoSquare>) -> bool {
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

fn get_input() -> (Vec<u64>, Vec<Board>) {
    let mut input = INPUT.split_terminator("\n\n");
    let draws = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();
    let boards = input.map(Board::new).collect::<Vec<_>>();
    (draws, boards)
}

pub fn part1_pretty() {
    println!("day 4 part 1: {}", p1_naive());
}

// fn part2() -> XXX {}

// pub fn part2_pretty() {
//     println!("day XXX part 2: {}", part2());
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_part1() {
        assert_eq!(p1_naive(), 44088);
    }

    #[test]
    fn t_part2() {
        // assert_eq!( , );
    }
}

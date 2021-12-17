use itertools::Itertools;

const INPUT: &'static str = include_str!("input/4.txt");

struct Board {
    board: [[u32; 5]; 5],
    marked: Vec<(u8, u8)>,
}

impl Board {
    fn from(board_str: &str) -> Self {
        let mut board: [[u32; 5]; 5] = Default::default();
        for (r, row) in board_str.lines().enumerate() {
            for (c, val) in row.split_ascii_whitespace().enumerate() {
                board[r][c] = val.parse().unwrap();
            }
        }

        Self {
            board,
            marked: Vec::with_capacity(12),
        }
    }

    fn has_bingo(&self) -> bool {
        let rs = [(0u32, 0u32), (0, 1), (0, 2), (0, 3), (0, 4)];
    }
}

fn p1_naive() -> u64 {
    let mut input = INPUT.split_terminator("\n\n");

    let draws = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let boards = input.map(Board::from).collect::<Vec<_>>();

    todo!()
}

pub fn part1_pretty() {
    println!("day 4 part 1: {}", p1_naive());
}

// fn part2() -> XXX {}

// pub fn part2_pretty() {
//     println!("day XXX part 2: {}", part2());
// }

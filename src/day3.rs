const INPUT: &'static str = include_str!("input/3.txt");

fn part1() -> u64 {
    let mut input = INPUT.lines().peekable();

    let width = input.peek().map_or(12, |s| s.len());
    let mut counts = vec![0u64; width];

    let mut len_input = 0u64;
    for num in input {
        for (idx, digit) in num.chars().rev().enumerate() {
            match digit {
                '1' => {
                    *counts
                        .get_mut(idx)
                        .expect("one of the numbers was longer than the first") += 1;
                }
                '0' => {}
                _ => unreachable!("non-binary digit 😠"),
            }
        }
        len_input += 1;
    }

    let threshold = len_input / 2;
    let rates = counts
        .iter()
        .map(|n| *n > threshold)
        .enumerate()
        .fold((0, 0), |acc, (idx, b)| {
            let pow = 2u64.pow(idx.try_into().unwrap());
            if b {
                (acc.0 + pow, acc.1)
            } else {
                (acc.0, acc.1 + pow)
            }
        });

    rates.0 * rates.1
}

pub fn part1_pretty() {
    println!("part 1: {:?}", part1());
}

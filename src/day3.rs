use std::ops::Shl;

fn get_input() -> Vec<&'static str> {
    include_str!("input/3.txt").split_terminator("\n").collect()
}

fn get_gamma(list: Vec<&str>) -> u32 {
    let width = list[0].len();

    let mut one_counts = vec![0u32; width];

    for num in &list {
        for (i, digit) in num.chars().enumerate() {
            if digit == '1' {
                one_counts[i] = one_counts[i] + 1;
            }
        }
    }

    // dbg!(&counts);

    let counts = one_counts;
    let threshold = dbg!((list.len() as f64 / 2.0) as u32);
    counts
        .into_iter()
        .map(|c| if c > threshold { 1 } else { 0 })
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, bit)| {
            acc + if bit == 1 {
                2_u32.pow(idx.try_into().unwrap())
            } else if bit == 0 {
                0
            } else {
                unreachable!()
            }
        })
}

fn part1() -> u64 {
    let gamma = get_gamma(get_input());
    let epsilon = !gamma;

    let res = gamma.widening_mul(epsilon);
    ((res.1 as u64) << 32) + (res.0 as u64)
}

pub fn part1_pretty() {
    println!("gamma rate * episilon rate: {}", part1());
}

// wrong: 2808908183214

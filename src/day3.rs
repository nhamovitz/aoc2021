use std::collections::HashSet;

const INPUT: &'static str = include_str!("input/3.txt");

fn part1() -> u64 {
    let mut input = INPUT.lines().peekable();

    let width = input.peek().map_or(12, |s| s.len());
    let input = input;

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
            let pow = 2_u64.pow(idx.try_into().unwrap());
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
pub fn part2_pretty() {
    println!("part 2: {:?}", part2());
}

#[derive(Debug)]
enum MoreCommon {
    One,
    Zero,
    Equal,
}
impl MoreCommon {
    fn from(n: f64, threshold: f64) -> Self {
        if n < threshold {
            MoreCommon::Zero
        } else if n == threshold {
            MoreCommon::Equal
        } else if n > threshold {
            MoreCommon::One
        } else {
            unreachable!()
        }
    }
    fn oxygen(&self) -> char {
        match self {
            Self::One | Self::Equal => '1',
            Self::Zero => '0',
        }
    }
    fn co2(&self) -> char {
        match self {
            Self::One | Self::Equal => '0',
            Self::Zero => '1',
        }
    }
}

fn part2() -> u64 {
    let input: Vec<_> = INPUT.lines().collect();

    let (width, len_input) = info(input.iter());
    let len_input: usize = len_input.try_into().unwrap();

    let mut oxygen_candidates: HashSet<&str> = input.into_iter().collect();
    let mut co2_candidates = oxygen_candidates.clone();

    // Oxygen
    for i in 0..width {
        dbg!(i);
        dbg!(&oxygen_candidates);

        let mut to_remove_oxygen = Vec::with_capacity(len_input);

        let (counts, len_input) = counts_in(width, &oxygen_candidates);
        let common_digits = get_common_digits(&counts, len_input as u64);

        dbg!(&common_digits);

        for num_str in &oxygen_candidates {
            if num_str.chars().nth(i).unwrap() != common_digits[i].oxygen() {
                to_remove_oxygen.push(num_str.clone());
            }
        }
        for num_str in to_remove_oxygen {
            assert!(oxygen_candidates.remove(num_str));
        }
        if oxygen_candidates.len() == 1 {
            dbg!(&oxygen_candidates);
            break;
        }
    }

    // CO2
    for i in 0..width {
        let mut to_remove_co2 = Vec::with_capacity(len_input);

        let (counts, len_input) = counts_in(width, &co2_candidates);

        let common_digits = get_common_digits(&counts, len_input as u64);
        for num_str in &co2_candidates {
            if num_str.chars().nth(i).unwrap() != common_digits[i].co2() {
                to_remove_co2.push(num_str.clone());
            }
        }
        for num_str in to_remove_co2 {
            assert!(co2_candidates.remove(num_str));
        }
        if co2_candidates.len() == 1 {
            break;
        }
    }

    assert_eq!(oxygen_candidates.len(), 1);
    assert_eq!(co2_candidates.len(), 1);

    let lhs = single_elem_hs_to_num(oxygen_candidates);
    let rhs = single_elem_hs_to_num(co2_candidates);
    let prod = lhs * rhs;
    dbg!(lhs, rhs, prod);
    prod
}

fn single_elem_hs_to_num(hs: HashSet<&str>) -> u64 {
    assert_eq!(hs.len(), 1);
    u64::from_str_radix(*hs.iter().next().unwrap(), 2).unwrap()
}

fn get_common_digits(counts: &[u64], len: u64) -> Vec<MoreCommon> {
    counts
        .iter()
        .map(|n| MoreCommon::from(*n as f64, len as f64 / 2.))
        .collect()
}

fn w_hashsets_incorrect(input: Vec<&str>, width: usize, more_common_digit: Vec<bool>) -> u64 {
    let mut oxygen_candidates: HashSet<&&str> = HashSet::from_iter(input.iter());
    let mut co2_candidates = oxygen_candidates.clone();
    for i in 0..width {
        for num in &input {
            let bit_matches_common = (num.chars().nth(i).unwrap() == '1') == more_common_digit[i];
            if oxygen_candidates.len() != 1 && !bit_matches_common {
                oxygen_candidates.remove(num);
            } else if co2_candidates.len() != 1 && bit_matches_common {
                co2_candidates.remove(num);
            }
        }
    }

    assert_eq!(oxygen_candidates.len(), 1);
    assert_eq!(co2_candidates.len(), 1);
    u64::from_str_radix(oxygen_candidates.iter().next().unwrap(), 2).unwrap()
        * u64::from_str_radix(co2_candidates.iter().next().unwrap(), 2).unwrap()
}

fn info<'a, S>(input: impl Iterator<Item = S>) -> (usize, u64)
where
    S: AsRef<str>,
{
    let mut input = input.peekable();
    let width = input.peek().map_or(12, |s| s.as_ref().len());

    let (counts, len_input) = counts_in(width, input);
    (width, len_input)
}

fn counts_in<'a, S>(width: usize, input: impl IntoIterator<Item = S>) -> (Vec<u64>, u64)
where
    S: AsRef<str>,
{
    let mut counts = vec![0u64; width];
    let mut len_input = 0u64;
    for num in input {
        for (idx, digit) in num.as_ref().chars().enumerate() {
            match digit {
                '1' => {
                    counts[idx] += 1;
                }
                '0' => {}
                _ => unreachable!("non-binary digit 😠 (tho actually 😁)"),
            }
        }
        len_input += 1;
    }
    (counts, len_input)
}
// wrong:  2249760
// wrong:   427716
// wrong: 11833600
// wrong:  2253200 (admittedly almost-total guess)

// correct!: 6085575

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), 2250414);
    }

    #[test]
    fn t_part2() {
        assert_eq!(part2(), 6085575);
    }
}

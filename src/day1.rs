use tap::Pipe;

fn get_increasing_count(list: &[u32]) -> u32 {
    if list.is_empty() {
        return 0;
    }

    let mut prev = u32::MAX;
    let mut count = 0;
    for n in list {
        if n > &prev {
            count += 1;
        }
        prev = *n;
    }

    count
}

fn get_input() -> Vec<u32> {
    let input: Result<Vec<_>, _> = include_str!("input/1.txt")
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>())
        .collect();

    match input {
        Ok(v) => v,
        Err(e) => panic!("couldn't parse some line as `u32`: {}", e),
    }
}

fn sum(window: (u32, u32, u32)) -> u32 {
    window.0 + window.1 + window.2
}

fn get_windows(list: &[u32]) -> Result<Vec<(u32, u32, u32)>, &str> {
    let len = list.len();
    if len < 3 {
        return Err("not enough elements to make even one 3-tuple");
    }

    let mut res = Vec::with_capacity(len - 2);
    for i in 0..len - 2 {
        res.push((list[i], list[i + 1], list[i + 2]));
    }
    Ok(res)
}

fn part1() -> u32 {
    get_input().pipe_as_ref(get_increasing_count)
}

fn part2() -> u32 {
    get_input()
        .pipe_as_ref(get_windows)
        .expect("would be a pretty stupid challenge")
        .into_iter()
        .map(sum)
        .collect::<Vec<_>>()
        .pipe_as_ref(get_increasing_count)
}

pub fn part1_pretty() {
    println!("increased {} times", part1());
}

pub fn part2_pretty() {
    println!("sliding window increased {} times", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), 1184);
    }

    #[test]
    fn t_part2() {
        assert_eq!(part2(), 1158);
    }
}

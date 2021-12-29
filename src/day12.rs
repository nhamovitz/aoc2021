use std::{
    collections::{HashMap, HashSet},
    vec,
};

const INPUT: &str = include_str!("input/12.txt");

type Cave = &'static str;

fn is_lowercase(s: &str) -> bool {
    s.chars().next().unwrap().is_ascii_lowercase()
}

fn get_input() -> HashMap<Cave, HashSet<Cave>> {
    let mut map: HashMap<Cave, HashSet<Cave>> = HashMap::with_capacity(INPUT.lines().count() * 2);

    for path in INPUT.lines() {
        let (a, b) = path.split_once('-').unwrap();

        // no idea which of these to use lol

        // macro_rules! insert_path {
        //     ($start:ident, $end:ident, $map:ident) => {{
        //         if let Some(connections) = ($map).get_mut($start) {
        //             connections.push($end)
        //         } else {
        //             $map.insert($start, vec![$end]);
        //         }
        //     }};
        // }
        // insert_path!(a, b, map);
        // insert_path!(b, a, map);

        let mut insert_path = |start, end| {
            if let Some(connections) = map.get_mut(start) {
                debug_assert!(connections.insert(end));
            } else {
                map.insert(start, HashSet::from([end]));
            }
        };
        insert_path(a, b);
        insert_path(b, a);
    }

    map
}

fn part1() -> u64 {
    let graph = get_input();
    traverse(&graph, "start", "end", vec![])
}

fn traverse(
    graph: &HashMap<Cave, HashSet<Cave>>,
    start: Cave,
    destination: Cave,
    mut path: Vec<Cave>,
) -> u64 {
    if path.last() == Some(&destination) {
        return 0;
    }

    path.push(start);

    let mut paths = 0;

    let can_get_to = graph.get(start).unwrap();
    if can_get_to.contains(destination) {
        paths += 1;
    }

    for cave in can_get_to {
        // short-circuits: we want to recurse if either 1) it's uppercase
        // (probably faster to check) to 2) we haven't already visited this cave
        if !is_lowercase(cave) || !path.contains(cave) {
            paths += traverse(graph, cave, destination, path.clone());
        }
    }
    paths
}

pub fn part1_pretty() {
    println!("day 12 part 1: {}", part1());
}

fn part2() -> u64 {
    todo!()
}

pub fn part2_pretty() {
    println!("day 12 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_ANS: u64 = 4912;
    // const P2_ANS: u64 = 2____;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), P1_ANS);
    }

    #[test]
    fn t_part2() {
        // assert_eq!(part2(), P2_ANS);
    }

    extern crate test;
    use test::{black_box, Bencher};

    #[bench]
    fn b_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    // #[bench]
    // fn b_part2(b: &mut Bencher) {
    //     b.iter(part2);
    // }
}

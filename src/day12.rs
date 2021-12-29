use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input/12.txt");

type Cave = &'static str;

fn is_uppercase(s: &str) -> bool {
    s.chars().next().unwrap().is_ascii_uppercase()
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
    if start == destination {
        return 1;
    }

    path.push(start);

    let mut paths = 0;
    for cave in graph.get(start).unwrap() {
        // short-circuits: we want to recurse if either 1) it's uppercase
        // (probably faster to check) to 2) we haven't already visited this cave
        if is_uppercase(cave) || !path.contains(cave) {
            paths += traverse(graph, cave, destination, path.clone());
        }
    }
    paths
}

pub fn part1_pretty() {
    println!("day 12 part 1: {}", part1());
}

#[derive(Clone, Default, Debug)]
struct Path {
    visited: Vec<Cave>,
    visit_twice_cave: (Cave, usize),
}

impl Path {
    fn can_visit(&self, cave: &str) -> bool {
        // dbg!(cave);
        if is_uppercase(cave) {
            return true;
        }

        if matches!(cave, "start") {
            return false;
        }

        // this is the revist-allowed cave
        if cave == self.visit_twice_cave.0 {
            return self.visit_twice_cave.1 < 2;
        }

        !self.visited.contains(&cave)
    }

    fn visit(&mut self, cave: Cave) {
        if cave == self.visit_twice_cave.0 {
            debug_assert!(self.visit_twice_cave.1 < 2);
            self.visit_twice_cave.1 += 1;
            self.visited.push("visit_cave");
        } else {
            // comment out to add the uppercase caves to the path, for debugging
            // update: nope, uncommenting this,, breaks the program,, 😭😭
            // if !is_uppercase(cave) {
            self.visited.push(cave);
            // }
        }
    }

    fn with_visit_cave(cave: Cave) -> Self {
        Self {
            visited: Default::default(),
            visit_twice_cave: (cave, 0),
        }
    }

    pub fn uses_visit_cave(&self) -> bool {
        self.visited.contains(&"visit_cave")
    }
}

// wrong: 32717
fn traverse_p2(
    graph: &HashMap<Cave, HashSet<Cave>>,
    start: Cave,
    destination: Cave,
    mut path: Path,
    paths: &mut HashSet<Vec<Cave>>,
) -> u64 {
    if start == destination {
        // println!("hit recursion limit dw");
        if path.uses_visit_cave() {
            // dbg!(&path);
            let mut replace = vec![];
            for (i, cave) in path.visited.iter().enumerate() {
                if cave == &"visit_cave" {
                    replace.push(i);
                }
            }
            let mut res = path.visited;
            for i in replace {
                res[i] = path.visit_twice_cave.0;
            }

            paths.insert(res);
            return 1;
        } else {
            return 0;
        }
    }
    path.visit(start); // this could be after the conditional for tiny bit more efficiency, but this way makes it easier to read the path

    let mut path_total = 0;
    // dbg!(start);
    for cave in graph.get(start).unwrap() {
        if path.can_visit(cave) {
            // moving `path.visit(start);` to here makes the output non-deterministic 😭. i hate recursion
            path_total += traverse_p2(graph, cave, destination, path.clone(), paths);
        }
    }
    path_total
}

// Tries: like 6? and several hours
fn part2() -> u64 {
    let graph = get_input();

    let mut paths = HashSet::new();

    for cave in graph
        .keys()
        .filter(|cave| !is_uppercase(cave) && !matches!(**cave, "start" | "end"))
    {
        traverse_p2(
            &graph,
            "start",
            "end",
            Path::with_visit_cave(cave),
            &mut paths,
        );
    }
    paths.len().try_into().unwrap()
}

pub fn part2_pretty() {
    println!("day 12 part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_ANS: u64 = 4912;
    const P2_ANS: u64 = 150004;

    #[test]
    fn t_part1() {
        assert_eq!(part1(), P1_ANS);
    }

    #[test]
    fn t_part2() {
        assert_eq!(part2(), P2_ANS);
    }

    extern crate test;
    use test::{black_box, Bencher};

    #[bench]
    fn b_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn b_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}

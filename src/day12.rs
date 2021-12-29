use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input/12ex1.txt");

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
    path: HashSet<(Cave, Option<usize>)>,
    // (which cave we're allowed to visit twice, how many times we've visited it)
}

enum CanVisitStatus {
    CanVisitAndMakeVisitCave,
    CanVisitButNotMakeVisitCave,
    CannotVisit,
    CanVisitOnceMore,
}

impl Path {
    fn can_visit(&self, cave: &str) -> CanVisitStatus {
        use CanVisitStatus::*;
        // dbg!(cave);
        if is_uppercase(cave) {
            return CanVisitButNotMakeVisitCave;
        }

        if matches!(cave, "start" | "end") {
            return CannotVisit;
        }

        if self.no_visit_cave_set() {
            return CanVisitAndMakeVisitCave;
        }

        // this is the revist-allowed cave
        let matching_count = self
            .path
            .iter()
            .filter(|(cv, visit_metadata)| &cave == cv && visit_metadata == &Some(1))
            .count();
        if matching_count > 0 {
            debug_assert_eq!(matching_count, 1);
            return CanVisitOnceMore;
        }

        // otherwise, "is the cave in the already-visited set?"
        let matching_count = self.path.iter().filter(|(cv, _)| &cave == cv).count();
        if matching_count > 0 {
            debug_assert_eq!(matching_count, 1);
            return CannotVisit;
        }

        unreachable!()
    }

    fn no_visit_cave_set(&self) -> bool {
        self.path
            .iter()
            .all(|(_cave, visit_metadata)| visit_metadata.is_none())
    }

    fn update_cave_if_previously_visited(&mut self, cave: Cave) {
        let mut updated = 0;
        for (cv, mut visit_metadata) in &self.path {
            if let Some(ref mut visits) = visit_metadata {
                if cv == &cave {
                    debug_assert_eq!(*visits, 1);
                    *visits += 1;
                    updated += 1;
                }
            }
        }
        debug_assert_eq!(updated, 1);
    }

    fn visit(&mut self, cave: Cave, set_visit_cave: bool) {
        if set_visit_cave {
            debug_assert!(self.no_visit_cave_set());
            debug_assert!(!is_uppercase(cave));

            self.path.insert((cave, Some(1)));
        } else {
            self.path.insert((cave, None));
            if !is_uppercase(cave) {
                self.update_cave_if_previously_visited(cave);
            }
        }
    }

    fn empty() -> Self {
        Self::default()
    }
}

// wrong: 32717
fn traverse_p2(
    graph: &HashMap<Cave, HashSet<Cave>>,
    start: Cave,
    destination: Cave,
    mut path: Path,
    iter: u64,
) -> u64 {
    if start == destination {
        // println!("hit recursion limit dw");
        println!("{:?}", path.path);
        return 1;
    }

    if iter > 50  {
        return 0;
    }
    let mut paths = 0;
    for cave in graph.get(start).unwrap() {
        // short-circuits: we want to recurse if either 1) it's uppercase
        // (probably faster to check) to 2) we haven't already visited this cave
        match path.can_visit(cave) {
            CanVisitStatus::CanVisitAndMakeVisitCave => {
                path.visit(cave, true);
                paths += traverse_p2(graph, cave, destination, path.clone(), iter + 1);
            }
            CanVisitStatus::CanVisitButNotMakeVisitCave | CanVisitStatus::CanVisitOnceMore => {
                path.visit(cave, false);
                paths += traverse_p2(graph, cave, destination, path.clone(), iter + 1);
            }
            CanVisitStatus::CannotVisit => {}
        }
    }
    paths

    // let mut paths = 0;

    // let to_visit = graph.get(start).unwrap();

    // if path.visit(start) {
    //     let mut path_visiting_cave = path.clone();
    //     path_visiting_cave.set_visit_cave(start);

    //     for cave in to_visit {
    //         paths += traverse_p2(graph, cave, destination, path_visiting_cave.clone());
    //     }
    // }

    // let mut paths = 0;
    // for cave in to_visit {
    //     paths += traverse_p2(graph, cave, destination, path.clone());
    // }

    // paths
}

fn part2() -> u64 {
    let graph = get_input();
    traverse_p2(&graph, "start", "end", Path::empty(), 0)
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

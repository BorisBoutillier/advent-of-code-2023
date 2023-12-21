use std::collections::{HashMap, HashSet};
pub struct Map {
    start: (isize, isize),
    map: Vec<Vec<bool>>,
}
impl Map {
    pub fn parse(input: &str) -> Map {
        let mut start = None;
        let map: Vec<Vec<bool>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => true,
                        'S' => {
                            start = Some((x as isize, y as isize));
                            true
                        }
                        '#' => false,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        let start = start.unwrap();
        let len_y = map.len() as isize;
        let len_x = map[0].len() as isize;
        println!("Map {len_x} x {len_y}");
        println!("Start {start:?}");
        Map { start, map }
    }
    pub fn len_x(&self) -> isize {
        self.map[0].len() as isize
    }
    pub fn len_y(&self) -> isize {
        self.map.len() as isize
    }
    pub fn is_free(&self, x: isize, y: isize) -> bool {
        self.map[y as usize][x as usize]
    }
    pub fn start(&self) -> (isize, isize) {
        self.start
    }
}

use aoc_commons::*;
pub fn get_next(x: isize, y: isize, map: &Map) -> HashSet<(isize, isize)> {
    [x - 1, x + 1]
        .into_iter()
        .filter(|&new_x| new_x >= 0 && new_x < map.len_x())
        .filter(|&new_x| map.is_free(new_x, y))
        .map(|new_x| (new_x, y))
        .chain(
            [y - 1, y + 1]
                .into_iter()
                .filter(|&new_y| new_y >= 0 && new_y < map.len_y())
                .filter(|&new_y| map.is_free(x, new_y))
                .map(|new_y| (x, new_y)),
        )
        .collect()
}
type Reachable = HashMap<(isize, isize), isize>;
use pathfinding::prelude::dijkstra_all;
pub fn count_from(reachables1: &Reachable, reachables2: &Reachable, steps: isize) -> usize {
    reachables1
        .iter()
        .filter(|&((x, y), d)| {
            (x + y) % 2 == steps % 2 && ((*d).min(*reachables2.get(&(*x, *y)).unwrap()) <= steps)
        })
        .count()
}

pub fn solver(part: Part, input: &str) -> String {
    let map = Map::parse(input);
    let successors = |n: &(isize, isize)| -> Vec<((isize, isize), usize)> {
        get_next(n.0, n.1, &map)
            .into_iter()
            .map(|(x, y)| ((x, y), 1))
            .collect()
    };

    let start = map.start();
    let mut reachables_from_start = dijkstra_all(&start, successors)
        .into_iter()
        .map(|(pos, (_, d))| (pos, d as isize))
        .collect::<HashMap<_, _>>();
    reachables_from_start.insert(start, 0);
    if part == Part::Part1 {
        let steps = 64;
        reachables_from_start
            .iter()
            .filter(|&((x, y), d)| (x + y) % 2 == steps % 2 && *d <= steps)
            .count()
            .to_string()
    } else {
        panic!("Not working");
        let steps = 26501365;
        //let steps = 64;
        assert_eq!(map.len_x(), map.len_y());
        assert_eq!(start.0, (map.len_x() - 1) / 2);
        assert_eq!(start.0, start.1);
        let start_s = (start.0, map.len_y() - 1);
        let mut reachables_from_s = dijkstra_all(&start_s, successors)
            .into_iter()
            .map(|(pos, (_, d))| (pos, d as isize))
            .collect::<HashMap<_, _>>();
        reachables_from_s.insert(start_s, 0);
        let start_n = (start.0, 0);
        let mut reachables_from_n = dijkstra_all(&start_n, successors)
            .into_iter()
            .map(|(pos, (_, d))| (pos, d as isize))
            .collect::<HashMap<_, _>>();
        reachables_from_n.insert(start_n, 0);
        let start_e = (map.len_x() - 1, start.1);
        let mut reachables_from_e = dijkstra_all(&start_e, successors)
            .into_iter()
            .map(|(pos, (_, d))| (pos, d as isize))
            .collect::<HashMap<_, _>>();
        reachables_from_e.insert(start_e, 0);
        let start_w = (map.len_x() - 1, start.1);
        let mut reachables_from_w = dijkstra_all(&start_w, successors)
            .into_iter()
            .map(|(pos, (_, d))| (pos, d as isize))
            .collect::<HashMap<_, _>>();
        reachables_from_w.insert(start_w, 0);
        let max_reach = [
            &reachables_from_n,
            &reachables_from_e,
            &reachables_from_s,
            &reachables_from_w,
        ]
        .iter()
        .map(|r| r.values().copied().max().unwrap())
        .max()
        .unwrap();
        let cached_count = count_from(
            &reachables_from_e,
            &reachables_from_e,
            steps - map.len_x() / 2,
        );
        println!("Max: {max_reach}");
        let n_grid = 1 + steps / map.len_x();
        let mut count = 0;
        for gx in -n_grid..=n_grid {
            if gx % 1000 == 0 {
                println!("{gx}");
            }
            for gy in -n_grid..=n_grid {
                let remaining_steps = steps
                    - ((gx.abs() - 1).max(0) * map.len_x() + gx.abs().min(1) * (map.len_x() / 2))
                    - ((gy.abs() - 1).max(0) * map.len_y() + gy.abs().min(1) * (map.len_y() / 2));
                if remaining_steps < 0 {
                    continue;
                }
                if (gx, gy) != (0, 0) && remaining_steps > max_reach {
                    count += cached_count;
                } else {
                    use std::cmp::Ordering::*;
                    count += match (gx.cmp(&0), gy.cmp(&0)) {
                        (Equal, Equal) => count_from(
                            &reachables_from_start,
                            &reachables_from_start,
                            remaining_steps,
                        ),
                        (Less, Equal) => {
                            count_from(&reachables_from_e, &reachables_from_e, remaining_steps)
                        }
                        (Greater, Equal) => {
                            count_from(&reachables_from_w, &reachables_from_w, remaining_steps)
                        }
                        (Equal, Less) => {
                            count_from(&reachables_from_n, &reachables_from_n, remaining_steps)
                        }
                        (Equal, Greater) => {
                            count_from(&reachables_from_s, &reachables_from_s, remaining_steps)
                        }
                        (Less, Less) => {
                            count_from(&reachables_from_e, &reachables_from_n, remaining_steps)
                        }
                        (Less, Greater) => {
                            count_from(&reachables_from_e, &reachables_from_s, remaining_steps)
                        }
                        (Greater, Less) => {
                            count_from(&reachables_from_w, &reachables_from_n, remaining_steps)
                        }
                        (Greater, Greater) => {
                            count_from(&reachables_from_w, &reachables_from_s, remaining_steps)
                        }
                    };
                }
            }
        }
        count.to_string()
    }
}
#[cfg(test)]
mod tests {
    //use super::*;
    //#[test]
    //fn example_part1() {
    //    assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "16");
    //}
    //#[test]
    //fn example_part2() {
    //    assert_eq!(
    //        solver(Part::Part2, include_str!("../example.txt")),
    //        "167409079868000"
    //    );
    //}
}

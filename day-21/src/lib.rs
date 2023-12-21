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
pub fn part1(map: &Map) -> String {
    let successors = |n: &(isize, isize)| -> Vec<((isize, isize), usize)> {
        get_next(n.0, n.1, &map)
            .into_iter()
            .map(|(x, y)| ((x, y), 1))
            .collect()
    };

    let start = map.start();
    let reachables = dijkstra_all(&start, successors);
    let steps = 64;
    let start_is_reached = (map.start.0 + map.start.1) % 2 == steps % 2;
    (reachables
        .iter()
        .filter(|((x, y), (_, d))| (x + y) % 2 == steps % 2 && *d as isize <= steps)
        .count()
        + start_is_reached as usize)
        .to_string()
}
use pathfinding::prelude::dijkstra_all;
pub fn solver(part: Part, input: &str) -> String {
    let map = Map::parse(input);
    if part == Part::Part1 {
        part1(&map)
    } else {
        "0".to_string()
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

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
    let cache_depth = 22;
    let mut cache = HashMap::new();
    for s in 1..=cache_depth {
        for x in (s - 1)..=(map.len_x() - s) {
            for y in (s - 1)..=(map.len_y() - s) {
                if !map.is_free(x, y) {
                    continue;
                }
                if s == 1 {
                    cache.insert((x, y, s), get_next(x, y, map));
                } else {
                    cache.insert(
                        (x, y, s),
                        cache
                            .get(&(x, y, s - 1))
                            .unwrap()
                            .iter()
                            .flat_map(|(x2, y2)| cache.get(&(*x2, *y2, 1)).unwrap().clone())
                            .collect(),
                    );
                }
            }
        }
    }
    let mut stack = vec![(map.start(), 64)];
    let mut dests = HashSet::new();
    let mut it = 0;
    while let Some(((x, y), depth)) = stack.pop() {
        it += 1;
        if it % 100_000 == 0 {
            println!("Stack: {}, cur ({x},{y}:{depth})", stack.len());
        }
        if depth <= cache_depth {
            dests.extend(cache.get(&(x, y, depth)).unwrap().iter().copied());
        } else {
            stack.extend(
                cache
                    .get(&(x, y, cache_depth))
                    .unwrap()
                    .iter()
                    .map(|&(new_x, new_y)| ((new_x, new_y), depth - cache_depth)),
            );
        }
    }
    println!("Cache size: {}", cache.len());
    println!("Iterations: {}", it);
    dests.len().to_string()
}
pub fn solver(part: Part, input: &str) -> String {
    let map = Map::parse(input);
    if part == Part::Part1 {
        return part1(&map);
    }
    "0".to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "16");
    }
    //#[test]
    //fn example_part2() {
    //    assert_eq!(
    //        solver(Part::Part2, include_str!("../example.txt")),
    //        "167409079868000"
    //    );
    //}
}

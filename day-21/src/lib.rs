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
struct DepthCount {
    counts: HashMap<isize, usize>,
    count_max_even: usize,
    count_max_odd: usize,
    max: isize,
}
impl DepthCount {
    pub fn new(reachables: &Reachable) -> DepthCount {
        let mut depth_count = HashMap::new();
        for d in reachables.values() {
            *depth_count.entry(*d).or_insert(0usize) += 1;
        }
        let max = *depth_count.keys().max().unwrap();
        let mut counts = HashMap::new();
        for steps in 0..=max {
            counts.insert(
                steps,
                depth_count
                    .iter()
                    .map(|(&d, &c)| {
                        if d % 2 == steps % 2 && d <= steps {
                            c
                        } else {
                            0
                        }
                    })
                    .sum::<usize>(),
            );
        }
        let count_max_even = depth_count
            .iter()
            .map(|(&d, &c)| {
                if d % 2 == (max * 2) % 2 && d <= max * 2 {
                    c
                } else {
                    0
                }
            })
            .sum::<usize>();
        let count_max_odd = depth_count
            .iter()
            .map(|(&d, &c)| {
                if d % 2 == (max * 2 + 1) % 2 && d <= max * 2 + 1 {
                    c
                } else {
                    0
                }
            })
            .sum::<usize>();
        DepthCount {
            counts,
            count_max_even,
            count_max_odd,
            max,
        }
    }
    fn count_for(&self, steps: isize) -> usize {
        if steps > self.max {
            if steps % 2 == 0 {
                self.count_max_even
            } else {
                self.count_max_odd
            }
        } else {
            *self.counts.get(&steps).unwrap()
        }
    }
}

use pathfinding::prelude::dijkstra_all;
pub fn count_from(reachables: &Reachable, steps: isize) -> usize {
    reachables
        .iter()
        .filter(|&(_, d)| d % 2 == steps % 2 && *d <= steps)
        .count()
}

pub fn solver(part: Part, input: &str) -> String {
    reach(
        input,
        match part {
            Part::Part1 => 64,
            Part::Part2 => 26501365,
        },
    )
    .to_string()
}
pub fn reach(input: &str, steps: isize) -> usize {
    let map = Map::parse(input);
    let successors = |n: &(isize, isize)| -> Vec<((isize, isize), usize)> {
        get_next(n.0, n.1, &map)
            .into_iter()
            .map(|(x, y)| ((x, y), 1))
            .collect()
    };

    let start = map.start();
    assert_eq!(map.len_x(), map.len_y());
    assert_eq!(start.0, (map.len_x() - 1) / 2);
    assert_eq!(start.0, start.1);
    let len = map.len_x();
    let half_len = len / 2 + 1;
    let max = map.len_x() - 1;
    let mut reachables = HashMap::new();
    let mut depth_counts = HashMap::new();
    for gx in -1..=1 {
        for gy in -1..=1 {
            let l_start = match (gx, gy) {
                (-1, -1) => (max, max),
                (-1, 0) => (max, start.1),
                (-1, 1) => (max, 0),
                (0, -1) => (start.0, max),
                (0, 0) => (start.0, start.1),
                (0, 1) => (start.0, 0),
                (1, -1) => (0, max),
                (1, 0) => (0, start.1),
                (1, 1) => (0, 0),
                _ => panic!(),
            };
            let mut r = dijkstra_all(&l_start, successors)
                .into_iter()
                .map(|(pos, (_, d))| (pos, d as isize))
                .collect::<HashMap<_, _>>();
            r.insert(l_start, 0);
            //println!("ADDING FOR {gx},{gy} {l_start:?}");
            depth_counts.insert((gx, gy), DepthCount::new(&r));
            reachables.insert((gx, gy), r);
        }
    }
    //println!("Max: {max_reach}, Cached even: {cached_even}, Cached odd: {cached_odd}");
    let n_grid = 1 + steps / len;
    //println!("N-GRID: {n_grid}");
    let mut count = 0;
    for gx in -n_grid..=n_grid {
        if gx % 1_000 == 0 {
            println!("GX: {gx}");
        }
        let gx_a = gx.abs();
        let max_gy = n_grid + 1 - gx_a;
        for gy in -max_gy..=max_gy {
            let gy_a = gy.abs();
            let used_steps = (gx_a - 1).max(0) * len
                + gx_a.min(1) * half_len
                + (gy_a - 1).max(0) * len
                + gy_a.min(1) * half_len;
            let remaining_steps = steps - used_steps;
            //println!(
            //    "g ({gx},{gy}) -> rem: {remaining_steps}, used: {used_steps} , count: {count}"
            //);
            if remaining_steps < 0 {
                continue;
            }
            {
                use std::cmp::Ordering::*;
                count += match (gx.cmp(&0), gy.cmp(&0)) {
                    (Equal, Equal) => depth_counts[&(0, 0)].count_for(remaining_steps),
                    (Less, Equal) => depth_counts[&(-1, 0)].count_for(remaining_steps),
                    (Greater, Equal) => depth_counts[&(1, 0)].count_for(remaining_steps),
                    (Equal, Less) => depth_counts[&(0, -1)].count_for(remaining_steps),
                    (Equal, Greater) => depth_counts[&(0, 1)].count_for(remaining_steps),
                    (Less, Less) => depth_counts[&(-1, -1)].count_for(remaining_steps),
                    (Less, Greater) => depth_counts[&(-1, 1)].count_for(remaining_steps),
                    (Greater, Less) => depth_counts[&(1, -1)].count_for(remaining_steps),
                    (Greater, Greater) => depth_counts[&(1, 1)].count_for(remaining_steps),
                };
            }
        }
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;
    fn make_3x3(input: &str) -> String {
        let input_3x1: String = input.lines().fold(String::new(), |mut s, line| {
            writeln!(
                s,
                "{}{}{}",
                line.replace('S', "."),
                line,
                line.replace('S', ".")
            )
            .expect("Oops");
            s
        });
        format!(
            "{}{}{}",
            input_3x1.replace('S', "."),
            input_3x1,
            input_3x1.replace('S', ".")
        )
    }
    fn make_5x5(input: &str) -> String {
        let input_5x1: String = input.lines().fold(String::new(), |mut s, line| {
            writeln!(
                s,
                "{}{}{}{}{}",
                line.replace('S', "."),
                line.replace('S', "."),
                line,
                line.replace('S', "."),
                line.replace('S', "."),
            )
            .expect("Oops");
            s
        });
        format!(
            "{}{}{}{}{}",
            input_5x1.replace('S', "."),
            input_5x1.replace('S', "."),
            input_5x1,
            input_5x1.replace('S', "."),
            input_5x1.replace('S', "."),
        )
    }
    #[test]
    fn mini_example_low_range() {
        let mini_example = include_str!("../mini_example.txt");
        let mini_example_3x3 = make_3x3(mini_example);
        let mini_example_5x5 = make_5x5(mini_example);
        for steps in 1..=200 {
            println!("### TESTING for {steps} ---- ");
            println!("### 1x1");
            let r1 = reach(mini_example, steps);
            println!("### 3x3");
            let r2 = reach(&mini_example_3x3, steps);
            println!("### 5x5");
            let r3 = reach(&mini_example_5x5, steps);
            println!("### -> {r1} vs {r2} vs {r3}");
            assert_eq!(r1, r2);
            assert_eq!(r2, r3);
        }
    }
    #[test]
    fn mini_example_10000() {
        let mini_example = include_str!("../mini_example.txt");
        let mini_example_3x3 = make_3x3(mini_example);
        let mini_example_5x5 = make_5x5(mini_example);
        let steps = 10000;
        println!("### TESTING for {steps} ---- ");
        println!("### 1x1");
        let r1 = reach(mini_example, steps);
        println!("### 3x3");
        let r2 = reach(&mini_example_3x3, steps);
        println!("### 5x5");
        let r3 = reach(&mini_example_5x5, steps);
        println!("### -> {r1} vs {r2} vs {r3}");
        assert_eq!(r1, r2);
        assert_eq!(r2, r3);
    }
}

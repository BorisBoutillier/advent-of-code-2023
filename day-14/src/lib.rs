use std::collections::HashMap;

use aoc_commons::Part;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}
const BOULDER: char = 'O';
const ROCK: char = '#';
const EMPTY: char = '.';

pub fn part1(platform: &[Vec<char>]) -> usize {
    let n_row = platform.len();
    let n_col = platform[0].len();
    let mut load = 0;

    for c in 0..n_col {
        let mut cur_stop = 0;
        for r in 0..n_row {
            match platform[r][c] {
                ROCK => {
                    cur_stop = r + 1;
                }
                BOULDER => {
                    load += n_row - cur_stop;
                    cur_stop += 1;
                }
                _ => (),
            }
        }
    }
    load
}

pub fn load(platform: &[Vec<char>]) -> usize {
    let n_row = platform.len();
    platform
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .map(|item| if item == &BOULDER { n_row - r } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}
fn print(platform: &[Vec<char>]) {
    for row in platform.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}
fn tilt(platform: &mut Vec<Vec<char>>, dir: Direction) {
    let n_row = platform.len();
    let n_col = platform[0].len();
    match dir {
        Direction::N => {
            for c in 0..n_col {
                let mut cur_stop = 0;
                for r in 0..n_row {
                    match platform[r][c] {
                        ROCK => {
                            cur_stop = r + 1;
                        }
                        BOULDER => {
                            platform[r][c] = EMPTY;
                            platform[cur_stop][c] = BOULDER;
                            cur_stop += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        Direction::S => {
            for c in 0..n_col {
                let mut cur_stop = n_row - 1;
                for r in (0..n_row).rev() {
                    match platform[r][c] {
                        ROCK => {
                            if r > 0 {
                                cur_stop = r - 1;
                            }
                        }
                        BOULDER => {
                            platform[r][c] = EMPTY;
                            platform[cur_stop][c] = BOULDER;
                            cur_stop -= 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        Direction::W => {
            for row in platform.iter_mut() {
                let mut cur_stop = 0;
                for c in 0..n_col {
                    match row[c] {
                        ROCK => {
                            cur_stop = c + 1;
                        }
                        BOULDER => {
                            row[c] = EMPTY;
                            row[cur_stop] = BOULDER;
                            cur_stop += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        Direction::E => {
            for row in platform.iter_mut() {
                let mut cur_stop = n_col - 1;
                for c in (0..n_col).rev() {
                    match row[c] {
                        ROCK => {
                            if c > 0 {
                                cur_stop = c - 1;
                            }
                        }
                        BOULDER => {
                            row[c] = EMPTY;
                            row[cur_stop] = BOULDER;
                            cur_stop -= 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
const N_CYCLE: usize = 1_000_000_000;

pub fn solver(part: Part, input: &str) -> String {
    use Direction::*;
    let mut platform: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    match part {
        Part::Part1 => {
            tilt(&mut platform, N);
            load(&platform)
        }
        Part::Part2 => {
            let mut cache = HashMap::new();
            let mut cycle = 0;
            while cycle < N_CYCLE {
                for dir in [N, W, S, E].into_iter() {
                    tilt(&mut platform, dir);
                }
                if let Some(cycle_prev) = cache.get(&platform.clone()) {
                    let repeat = (N_CYCLE - cycle) / (cycle - cycle_prev);
                    cycle += repeat * (cycle - cycle_prev);
                } else {
                    cache.insert(platform.clone(), cycle);
                }
                cycle += 1;
            }
            load(&platform)
        }
    }
    .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "136");
    }
    #[test]
    fn example_part2() {
        assert_eq!(solver(Part::Part2, include_str!("../example.txt")), "64");
    }
}

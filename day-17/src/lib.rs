use std::{collections::HashMap, iter};

use aoc_commons::Part;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    NS,
    WE,
}

type Entry = ((isize, isize), Direction);
fn update_cache(cache: &mut HashMap<Entry, u32>, entry: Entry, heat_loss: u32) -> bool {
    if let Some(loss) = cache.get_mut(&entry) {
        if heat_loss >= *loss {
            return false;
        } else {
            *loss = (*loss).min(heat_loss);
        }
    } else {
        cache.insert(entry, heat_loss);
    }
    true
}
fn compute(map: Vec<Vec<u32>>, v1: isize, v2: isize) -> u32 {
    use Direction::*;
    let len_y = map.len() as isize;
    let len_x = map[0].len() as isize;
    println!("Len {len_x}x{len_y}");
    let mut stack = vec![(((0, 0), WE), 0), (((0, 0), NS), 0)];
    let mut cache = HashMap::new();
    let mut count = 0usize;
    let mut exit_heat_loss = u32::MAX;
    while let Some((((x, y), dir), heat_loss)) = stack.pop() {
        //println!("({x},{y}) {dir:?} {heat_loss}");
        count += 1;
        if count % 10_000_000 == 0 {
            println!("{count:10} :  Stack {}", stack.len());
        }
        if x == len_x - 1 && y == len_y - 1 {
            exit_heat_loss = exit_heat_loss.min(heat_loss);
            continue;
        }
        let iters: [Box<dyn Iterator<Item = (isize, isize)>>; 2] = match dir {
            NS => [
                Box::new((1..=v2.min(x)).map(|v| -v).zip(iter::repeat(0))),
                Box::new((1..=v2.min(len_x - x - 1)).zip(iter::repeat(0))),
            ],
            WE => [
                Box::new(iter::repeat(0).zip((1..=v2.min(y)).map(|v| -v))),
                Box::new(iter::repeat(0).zip(1..=v2.min(len_y - 1 - y))),
            ],
        };
        for iter in iters {
            let mut hl = heat_loss;
            for (dx, dy) in iter {
                let nx = dx + x;
                let ny = dy + y;
                hl += map[ny as usize][nx as usize];
                if hl >= exit_heat_loss {
                    break;
                }
                if (dy == 0 && dx.abs() < v1) || (dx == 0 && dy.abs() < v1) {
                    continue;
                }
                let entry = (
                    (nx, ny),
                    match dir {
                        WE => NS,
                        NS => WE,
                    },
                );
                if update_cache(&mut cache, entry, hl) {
                    stack.push((entry, hl));
                }
            }
        }
    }
    println!("COUNT {count}");
    exit_heat_loss
}

pub fn solver(part: Part, input: &str) -> String {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    match part {
        Part::Part1 => compute(map, 1, 3),
        Part::Part2 => compute(map, 4, 10),
    }
    .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "102");
    }
    #[test]
    fn example_part2_1() {
        assert_eq!(solver(Part::Part2, include_str!("../example.txt")), "94");
    }
    #[test]
    fn example_part2_2() {
        assert_eq!(solver(Part::Part2, include_str!("../example.2.txt")), "71");
    }
}

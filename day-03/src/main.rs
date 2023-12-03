use std::collections::{HashMap, HashSet};

use aoc_commons::{solve_aoc, Part, Part::*};

fn main() {
    solve_aoc(include_str!("../input.txt"), solver);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct EnginePart {
    id: u32,
    number: u32,
}

fn get_parts_position(input: &str) -> HashMap<(usize, usize), EnginePart> {
    let mut part_id = 0;
    let mut result = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        let mut x = 0;
        for s in line.split(|c: char| !c.is_ascii_digit()) {
            if !s.is_empty() {
                let number = s.parse::<u32>().unwrap();
                let engine_part = EnginePart {
                    id: part_id,
                    number,
                };
                part_id += 1;
                for dx in 0..s.len() {
                    result.insert((x + dx, y), engine_part);
                }
            }
            x += s.len() + 1;
        }
    }
    result
}
fn solver(part: Part, input: &str) -> String {
    let parts_position = get_parts_position(input);
    let mut engine_parts = HashSet::new();
    let mut gears = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut checked_y = vec![y];
        if y != 0 {
            checked_y.push(y - 1)
        }
        if y != input.len() - 1 {
            checked_y.push(y + 1)
        }
        for (x, c) in line.chars().enumerate() {
            let mut checked_x = vec![x];
            if x != 0 {
                checked_x.push(x - 1)
            }
            if x != line.len() - 1 {
                checked_x.push(x + 1)
            }
            match part {
                Part1 => {
                    if !c.is_ascii_digit() && c != '.' {
                        for &check_y in checked_y.iter() {
                            for &check_x in checked_x.iter() {
                                if let Some(part) = parts_position.get(&(check_x, check_y)) {
                                    engine_parts.insert(part);
                                }
                            }
                        }
                    }
                }
                Part2 => {
                    if c == '*' {
                        let mut gear_engine_parts = HashSet::new();
                        for &check_y in checked_y.iter() {
                            for &check_x in checked_x.iter() {
                                if let Some(part) = parts_position.get(&(check_x, check_y)) {
                                    gear_engine_parts.insert(part);
                                }
                            }
                        }
                        if gear_engine_parts.len() == 2 {
                            gears.push((x, y, gear_engine_parts));
                        }
                    }
                }
            }
        }
    }
    match part {
        Part1 => engine_parts.iter().map(|part| part.number).sum::<u32>(),
        Part2 => gears
            .iter()
            .map(|(_x, _y, parts)| parts.iter().map(|part| part.number).product::<u32>())
            .sum::<u32>(),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.txt")),
            "4361"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "467835"
        );
    }
}

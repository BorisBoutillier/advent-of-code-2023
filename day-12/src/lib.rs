use std::collections::HashMap;

use aoc_commons::Part;
const DAMAGED: char = '#';
const OK: char = '.';
const UNK: char = '?';

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Row {
    springs: String,
    groups: Vec<usize>,
}
impl Row {
    fn parse(part: Part, line: &str) -> Row {
        let (springs, groups) = line.split_once(' ').unwrap();
        let mut springs = springs.to_string();
        let mut groups = groups.to_string();
        if part == Part::Part2 {
            springs = vec![springs; 5].join("?");
            groups = vec![groups; 5].join(",");
        }
        let groups = groups
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Row::new(springs, groups).expect("Illegal")
    }
    fn new(springs: String, groups: Vec<usize>) -> Option<Row> {
        let row = Row { springs, groups };
        row.simplify()
    }
    fn simplify(mut self) -> Option<Row> {
        let mut start_groups = vec![];
        let mut cur_count = 0;
        let mut last_ok = None;
        let mut end = true;
        for (i, spring) in self.springs.chars().enumerate() {
            match spring {
                UNK => {
                    end = false;
                    break;
                }
                OK => {
                    if cur_count != 0 {
                        start_groups.push(cur_count);
                    }
                    cur_count = 0;
                    last_ok = Some(i);
                }
                DAMAGED => {
                    cur_count += 1;
                }
                _ => panic!(),
            }
        }
        if end {
            // Means there has been no unknown
            if cur_count > 0 {
                start_groups.push(cur_count);
            }
            if start_groups != self.groups {
                return None;
            } else {
                return Some(Row {
                    springs: "".to_string(),
                    groups: vec![],
                });
            }
        }
        if start_groups.len() > self.groups.len()
            || (!start_groups
                .iter()
                .zip(self.groups.iter())
                .all(|(g1, g2)| g1 == g2))
            || (cur_count != 0
                && (self.groups.len() == start_groups.len()
                    || self.groups[start_groups.len()] < cur_count))
        {
            None
        } else {
            self.groups = self.groups[start_groups.len()..].to_vec();
            if let Some(idx) = last_ok {
                self.springs = self.springs[idx..].to_string();
            }
            Some(self)
        }
    }
    fn compute_arrangements(&self, cache: &mut HashMap<Row, u64>) -> u64 {
        if let Some(c) = cache.get(self) {
            *c
        } else {
            let c = self.lcl_compute_arrangements(cache);
            cache.insert(self.clone(), c);
            c
        }
    }
    fn lcl_compute_arrangements(&self, cache: &mut HashMap<Row, u64>) -> u64 {
        if self.groups.is_empty() {
            return !self.springs.contains(DAMAGED) as u64;
        }
        // Our simplification mecanism, makes solving left to right the fastest.
        let idx = self.springs.chars().position(|c| c == UNK).unwrap();

        [OK, DAMAGED]
            .iter()
            .map(|v| {
                let springs = [&self.springs[..idx], &self.springs[idx + 1..]].join(&v.to_string());
                let row = Row::new(springs, self.groups.clone());
                row.map(|row| row.compute_arrangements(cache)).unwrap_or(0)
            })
            .sum()
    }
}

pub fn solver(part: Part, input: &str) -> String {
    let mut sum = 0;
    for (_i, line) in input.lines().enumerate() {
        let row = Row::parse(part, line);
        //println!("ROW:{i:4}: {row:?}");
        let mut cache = HashMap::new();
        let n = row.compute_arrangements(&mut cache);
        //println!("   -> {n}");
        sum += n;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.txt")),
            "21"
        );
    }
    #[test]
    fn example_part1_0() {
        assert_eq!(solver(Part::Part1, "???.### 1,1,3"), "1");
    }
    #[test]
    fn example_part2_0() {
        assert_eq!(solver(Part::Part2, "???.### 1,1,3"), "1");
    }
    #[test]
    fn example_part2_1() {
        assert_eq!(solver(Part::Part2, ".??..??...?##. 1,1,3"), "16384");
    }
    #[test]
    fn example_part2_2() {
        assert_eq!(solver(Part::Part2, "?#?#?#?#?#?#?#? 1,3,1,6"), "1");
    }
    #[test]
    fn example_part2_3() {
        assert_eq!(solver(Part::Part2, "????.#...#... 4,1,1"), "16");
    }
    #[test]
    fn example_part2_4() {
        assert_eq!(solver(Part::Part2, "????.######..#####. 1,6,5"), "2500");
    }
    #[test]
    fn example_part2_5() {
        assert_eq!(solver(Part::Part2, "?###???????? 3,2,1"), "506250");
    }
    #[test]
    fn my_input_1() {
        assert_eq!(solver(Part::Part1, "??.#??.???#? 2,2"), "2");
    }
}

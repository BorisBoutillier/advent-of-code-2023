use rand::prelude::*;
use std::io::Write;
use std::{collections::HashMap, fs::OpenOptions};

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
    fn simplify(&self) -> Option<Row> {
        let mut start_groups = vec![];
        let mut cur_count = 0;
        let mut last_ok = None;
        for (i, spring) in self.springs.chars().enumerate() {
            match spring {
                UNK => {
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
            Some(self.clone())
        }
    }
    fn known_damaged_groups(&self) -> HashMap<usize, Vec<usize>> {
        let mut groups = HashMap::new();
        let mut cur = None;
        let mut cur_count = 0;
        let mut prev_is_ok = true;
        for (i, spring) in self.springs.chars().enumerate() {
            if cur.is_none() {
                cur = Some(spring);
                cur_count = 1;
            } else if cur == Some(spring) {
                cur_count += 1;
            } else {
                if prev_is_ok && cur == Some(DAMAGED) && spring == OK {
                    groups
                        .entry(cur_count)
                        .or_insert(vec![])
                        .push(i - cur_count);
                }
                prev_is_ok = cur == Some(OK);
                cur = Some(spring);
                cur_count = 1;
            }
        }
        if prev_is_ok && cur == Some(DAMAGED) {
            groups
                .entry(cur_count)
                .or_insert(vec![])
                .push(self.springs.len() - cur_count);
        }
        groups
    }
    fn match_groups(&self, springs: &str) -> bool {
        let (mut groups, cur, cur_count) = springs.chars().fold(
            (vec![], None, 0),
            |(mut groups, mut cur, mut cur_count), spring| {
                if cur.is_none() {
                    cur = Some(spring);
                    cur_count = 1;
                } else if Some(spring) == cur {
                    cur_count += 1;
                } else {
                    if cur == Some(DAMAGED) {
                        groups.push(cur_count);
                    }
                    cur = Some(spring);
                    cur_count = 1;
                }
                (groups, cur, cur_count)
            },
        );
        if cur == Some(DAMAGED) {
            groups.push(cur_count);
        }
        groups == self.groups
    }
    fn compute_arrangements(&self, cache: &mut (u32, HashMap<Row, u32>)) -> u32 {
        cache.0 += 1;
        if cache.0 % 1000000 == 0 {
            println!("   {:10}:  {self:?}", cache.0);
        }
        if let Some(c) = cache.1.get(self) {
            //println!("CACHED: {self:?} -> {c}");
            *c
        } else {
            let c = self.lcl_compute_arrangements(cache);
            //if self.springs.len() > 3 && !self.groups.is_empty() {
            //    cache.1.insert(self.clone(), c);
            //}
            c
        }
    }
    fn lcl_compute_arrangements(&self, cache: &mut (u32, HashMap<Row, u32>)) -> u32 {
        if self.groups.is_empty() {
            return !self.springs.contains(DAMAGED) as u32;
        }
        if self.springs.is_empty() {
            return 0;
        }
        if self.simplify().is_none() {
            //println!("ILLEGAL: {self:?}");
            return 0;
        }
        let min_groups = self
            .springs
            .split(OK)
            .filter(|s| !s.is_empty() && !s.chars().all(|c| c == UNK))
            .count();
        if min_groups > self.groups.len() {
            //println!("TOO SMALL {self:?} vs {min_groups}");
            return 0;
        }
        if self
            .springs
            .chars()
            .filter(|c| c == &DAMAGED || c == &UNK)
            .count()
            < self.groups.iter().sum()
        {
            return 0;
        }
        // Check if there is a separated damaged groups only existing once
        let known_damaged_groups = self.known_damaged_groups();
        //println!("SELF: {self:?}");
        //println!(" knowns_damaged_group: {known_damaged_groups:?}");
        for (group_size, group_idx_starts) in known_damaged_groups.into_iter() {
            let count = self.groups.iter().filter(|g| *g == &group_size).count();
            match count.cmp(&group_idx_starts.len()) {
                std::cmp::Ordering::Less => {
                    //println!("SKIP");
                    return 0;
                }
                std::cmp::Ordering::Equal => {
                    let group_pos = self
                        .groups
                        .iter()
                        .enumerate()
                        .filter(|(_, g)| *g == &group_size)
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();
                    //println!(
                    //    "  EQUALS: {} , {:?} vs {:?}",
                    //    group_size, group_idx_starts, group_pos
                    //);
                    let mut rows = vec![];
                    let mut cur_springs_idx = 0;
                    let mut cur_groups_idx = 0;
                    for (group_idx_start, group_group_pos) in
                        group_idx_starts.iter().zip(group_pos.iter())
                    {
                        rows.push(Row {
                            springs: self.springs[cur_springs_idx..*group_idx_start].to_string(),
                            groups: self.groups[cur_groups_idx..*group_group_pos].to_vec(),
                        });
                        cur_springs_idx = group_idx_start + group_size;
                        cur_groups_idx = *group_group_pos + 1;
                    }
                    rows.push(Row {
                        springs: self.springs[cur_springs_idx..].to_string(),
                        groups: self.groups[cur_groups_idx..].to_vec(),
                    });
                    rows.sort_by_key(|row| row.springs.chars().filter(|c| c == &UNK).count());
                    //println!("SPLIT");
                    //for row in rows.iter() {
                    //    println!("    -> {row:?}");
                    //}
                    //println!(
                    //    "SPLIT {:?}",
                    //    rows.iter().map(|row| row.springs.len()).collect::<Vec<_>>()
                    //);
                    let mut product = 1;
                    for row in rows {
                        let n = row.compute_arrangements(cache);
                        if n == 0 {
                            return 0;
                        }
                        product *= n;
                    }
                    return product;
                }
                std::cmp::Ordering::Greater => (),
            }
        }
        // Find all unknowns positions in
        let mut unknowns = self
            .springs
            .chars()
            .enumerate()
            .filter(|(_, s)| *s == UNK)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if unknowns.is_empty() {
            if self.match_groups(&self.springs) {
                1
            } else {
                0
            }
        } else {
            //// pick an unknown in the middle and force it to the two possible values
            //let bests = unknowns
            //    .iter()
            //    .enumerate()
            //    .filter(|(_i, &idx)| {
            //        (idx == 0 || self.springs.chars().nth(idx - 1).unwrap() != UNK)
            //            && (idx == self.springs.len() - 1
            //                || self.springs.chars().nth(idx + 1).unwrap() != UNK)
            //    })
            //    .map(|(i, _)| i)
            //    .collect::<Vec<_>>();
            //let mut rng = rand::thread_rng();
            //let idx = unknowns.remove(if !bests.is_empty() {
            //    *bests.choose(&mut rng).unwrap()
            //} else {
            //    rng.gen_range(0..unknowns.len())
            //});
            let idx = unknowns.remove(0);

            let mut order = vec![OK.to_string(), DAMAGED.to_string()];
            //order.shuffle(&mut rng);
            let springs_1 = [&self.springs[..idx], &self.springs[idx + 1..]].join(&order[0]);
            let row1 = Row {
                springs: springs_1,
                groups: self.groups.clone(),
            };

            let springs_2 = [&self.springs[..idx], &self.springs[idx + 1..]].join(&order[1]);
            let row2 = Row {
                springs: springs_2,
                groups: self.groups.clone(),
            };
            //println!("ADD");
            //println!("  {row1:?}");
            //println!("  {row2:?}");
            row1.compute_arrangements(cache) + row2.compute_arrangements(cache)
        }
    }
}

const CACHE_FILE_NAME: &str = "Day-12.cache";
pub fn solver(part: Part, input: &str) -> String {
    let mut cache = HashMap::new();
    if let Ok(s) = std::fs::read_to_string(CACHE_FILE_NAME) {
        for cache_line in s.lines() {
            let (line, res) = cache_line.split_once(" == ").unwrap();
            cache.insert(line.to_string(), res.parse::<u32>().unwrap());
        }
    }
    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        if let Some(n) = cache.get(line) {
            println!("ROW:{i:4} CACHED: {line} -> {n}");
            sum += n;
        } else {
            let row = Row::parse(part, line);
            println!("ROW:{i:4}: {row:?}");
            let mut cache = (0, HashMap::<Row, u32>::new());
            let n = row.compute_arrangements(&mut cache);
            println!("   -> {n}");
            // Open a file with append option
            // Update cache
            {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(CACHE_FILE_NAME)
                    .expect("cannot open file");
                file.write_all(format!("{line} == {n}\n").as_bytes())
                    .expect("Failed to write cache line");
            }
            sum += n;
        }
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
}

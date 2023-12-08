use std::collections::HashMap;

use aoc_commons::Part;
use num::integer::lcm;

pub fn solver(part: Part, input: &str) -> String {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => 1usize,
            'L' => 0usize,
            _ => panic!("What to do with {c} ?"),
        })
        .collect::<Vec<_>>();
    lines.next();
    let mut map = HashMap::new();
    let mut map_starts = vec![];
    let mut map_ends = vec![];
    for entry in lines {
        let (start, ends) = entry.split_once(" = ").unwrap();
        let (left, right) = ends[1..ends.len() - 1].split_once(", ").unwrap();
        assert_eq!(start.len(), 3);
        match start.chars().nth(2).unwrap() {
            'A' => map_starts.push(start),
            'Z' => map_ends.push(start),
            _ => (),
        }
        map.insert(start, [left, right]);
    }
    match part {
        Part::Part1 => {
            let mut step = 0;
            let mut cur = "AAA";
            for instruction in instructions.into_iter().cycle() {
                step += 1;
                cur = map[&cur][instruction];
                if cur == "ZZZ" {
                    break;
                }
            }
            step
        }
        // Brute force solution, does not work
        //Part::Part2 => {
        //    let mut step = 0;
        //    let mut curs = starts;
        //    for instruction in instructions.into_iter().cycle() {
        //        step += 1;
        //        for cur in curs.iter_mut() {
        //            *cur = map[cur][instruction];
        //        }
        //        if curs.iter().all(|node| node.chars().nth(2).unwrap() == 'Z') {
        //            break;
        //        }
        //    }
        //    step
        //}
        // This solution does not work with example, because repetions have an offset in the example
        // But this works with my input and was used to get fill my result.
        /* Part::Part2 => {
            let n_instructions = instructions.len();
            let mut m = vec![];
            for start in map_starts {
                let mut seq = vec![start];
                let mut cur = start;
                let mut offset = 0;
                for (step, &instruction) in instructions.iter().cycle().enumerate() {
                    let next = map[cur][instruction];
                    if let Some(pos) = seq.iter().position(|&node| node == next) {
                        if (step + 1 - pos) % n_instructions == 0 {
                            //println!("Found for {start} : {seq:?} , repeat at pos {pos}");
                            offset = pos;
                            seq = seq[pos..].to_vec();
                            break;
                        }
                    }
                    cur = next;
                    seq.push(cur);
                }
                println!("For start: {start}");
                for end in map_ends.iter() {
                    for (i, _) in seq.iter().enumerate().filter(|(_, node)| node == &end) {
                        println!("  Repeat {}+{}x", offset + i, seq.len());
                        assert_eq!(offset + i, seq.len());
                        m.push(offset + i);
                    }
                }
            }
            m.into_iter().reduce(lcm).unwrap()
        } */
        // Better solution adapted from the internet, not mine.
        Part::Part2 => map
            .keys()
            .filter(|key| key.ends_with('A'))
            .copied()
            .map(|mut element| {
                let result = instructions
                    .iter()
                    .cycle()
                    .position(|instruction| {
                        element = map.get(element).unwrap()[*instruction];
                        element.ends_with('Z')
                    })
                    .unwrap()
                    + 1;
                println!("For {element} found {result}");
                result
            })
            .reduce(lcm)
            .unwrap(),
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
            "2"
        );
    }
    #[test]
    fn example_part1_2() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.2.txt")),
            "6"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "6"
        );
    }
}

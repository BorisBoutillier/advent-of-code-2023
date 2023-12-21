use std::collections::HashMap;

use aoc_commons::*;
use range_set_blaze::prelude::*;

const MAX: u64 = 4000;

struct Xmas {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}
impl Xmas {
    pub fn parse(line: &str) -> Xmas {
        let (mut x, mut m, mut a, mut s) = (0, 0, 0, 0);
        for v in line[1..(line.len() - 1)].split(',') {
            let (l, vv) = v.split_once('=').unwrap();
            let vv = vv.parse::<u64>().unwrap();
            match l {
                "x" => x = vv,
                "m" => m = vv,
                "a" => a = vv,
                "s" => s = vv,
                _ => panic!(),
            }
        }
        Xmas { x, m, a, s }
    }
    pub fn eval(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
    pub fn get(&self, c: char) -> u64 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
struct XmasRange {
    x: RangeSetBlaze<u64>,
    m: RangeSetBlaze<u64>,
    a: RangeSetBlaze<u64>,
    s: RangeSetBlaze<u64>,
}
impl Default for XmasRange {
    fn default() -> Self {
        Self {
            x: RangeSetBlaze::from_iter([1..=MAX]),
            m: RangeSetBlaze::from_iter([1..=MAX]),
            a: RangeSetBlaze::from_iter([1..=MAX]),
            s: RangeSetBlaze::from_iter([1..=MAX]),
        }
    }
}
impl XmasRange {
    fn empty() -> XmasRange {
        Self {
            x: RangeSetBlaze::new(),
            m: RangeSetBlaze::new(),
            a: RangeSetBlaze::new(),
            s: RangeSetBlaze::new(),
        }
    }
    fn is_valid(&self) -> bool {
        !self.x.is_empty() && !self.m.is_empty() && !self.a.is_empty() && !self.s.is_empty()
    }
    fn split(&self, c: char, r: RangeSetBlaze<u64>) -> (XmasRange, XmasRange) {
        let mut r1 = self.clone();
        let mut r2 = self.clone();
        match c {
            'x' => {
                r1.x = r1.x & r.clone();
                r2.x = r2.x - r;
            }
            'm' => {
                r1.m = r1.m & r.clone();
                r2.m = r2.m - r;
            }
            'a' => {
                r1.a = r1.a & r.clone();
                r2.a = r2.a - r;
            }
            's' => {
                r1.s = r1.s & r.clone();
                r2.s = r2.s - r;
            }
            _ => panic!(),
        }
        (r1, r2)
    }
    fn count(&self) -> u128 {
        let count = self.x.len() * self.m.len() * self.a.len() * self.s.len();
        assert!(count > 0);
        count
    }
}

#[derive(Debug)]
struct Rule {
    v1: char,
    cond: Option<char>,
    v2: u64,
    dest: String,
}
impl Rule {
    fn parse(s: &str) -> Rule {
        if let Some((cond, dest)) = s.split_once(':') {
            if let Some((v1, v2)) = cond.split_once('<') {
                Rule {
                    v1: v1.chars().nth(0).unwrap(),
                    cond: Some('<'),
                    v2: v2.parse::<u64>().unwrap(),
                    dest: dest.to_string(),
                }
            } else {
                let (v1, v2) = cond.split_once('>').unwrap();
                Rule {
                    v1: v1.chars().nth(0).unwrap(),
                    cond: Some('>'),
                    v2: v2.parse::<u64>().unwrap(),
                    dest: dest.to_string(),
                }
            }
        } else {
            Rule {
                v1: ' ',
                cond: None,
                v2: 0,
                dest: s.to_string(),
            }
        }
    }
    fn apply(&self, xmas: &Xmas) -> Option<String> {
        match self.cond {
            None => Some(self.dest.clone()),
            Some('<') => (xmas.get(self.v1) < self.v2).then_some(self.dest.clone()),
            Some('>') => (xmas.get(self.v1) > self.v2).then_some(self.dest.clone()),
            _ => panic!(),
        }
    }
    fn apply_range(&self, xmas: &XmasRange) -> (String, XmasRange, XmasRange) {
        match self.cond {
            None => (self.dest.clone(), xmas.clone(), XmasRange::empty()),
            Some('<') => {
                let (x1, x2) = xmas.split(self.v1, RangeSetBlaze::from_iter([1..=(self.v2 - 1)]));
                (self.dest.clone(), x1, x2)
            }
            Some('>') => {
                let (x1, x2) = xmas.split(self.v1, RangeSetBlaze::from_iter([(self.v2 + 1)..=MAX]));
                (self.dest.clone(), x1, x2)
            }
            _ => panic!(),
        }
    }
}
struct Workflow {
    rules: Vec<Rule>,
}
impl Workflow {
    fn parse(line: &str) -> Workflow {
        let rules = line.split(',').map(Rule::parse).collect::<Vec<_>>();
        Workflow { rules }
    }
    fn apply(&self, xmas: &Xmas) -> String {
        self.rules
            .iter()
            .filter_map(|rule| rule.apply(xmas))
            .nth(0)
            .unwrap()
    }
    fn apply_range(&self, xmas: &XmasRange) -> Vec<(String, XmasRange)> {
        let mut dests = vec![];
        let mut cur = xmas.clone();
        for rule in self.rules.iter() {
            let (dest, r1, r2) = rule.apply_range(&cur);
            if r1.is_valid() {
                dests.push((dest, r1));
                cur = r2;
            }
        }
        dests
    }
}
pub fn solver(part: Part, input: &str) -> String {
    let (s, xmases) = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    for line in s.lines() {
        let (name, workflow_s) = line.trim_end_matches('}').split_once('{').unwrap();
        workflows.insert(name.to_string(), Workflow::parse(workflow_s));
    }
    match part {
        Part::Part1 => {
            let parts = xmases.lines().map(Xmas::parse).collect::<Vec<_>>();
            let mut result = 0;
            for part in parts.iter() {
                let mut workflow = "in".to_string();
                while workflow != "R" && workflow != "A" {
                    workflow = workflows[&workflow].apply(part);
                }
                if workflow == "A" {
                    result += part.eval();
                }
            }
            result.to_string()
        }
        Part::Part2 => {
            let mut result = 0;
            let mut stack = vec![("in".to_string(), XmasRange::default())];
            while let Some((dest, range)) = stack.pop() {
                if dest == "A" {
                    result += range.count();
                } else if dest != "R" {
                    stack.extend_from_slice(&workflows[&dest].apply_range(&range));
                }
            }
            result.to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1_1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "19114");
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.txt")),
            "167409079868000"
        );
    }
}

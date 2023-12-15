use aoc_commons::Part;

pub fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |hash, c| ((hash + c as usize) * 17) % 256)
}

pub fn solver(part: Part, input: &str) -> String {
    match part {
        Part::Part1 => input.trim_end().split(',').map(hash).sum::<usize>(),
        Part::Part2 => {
            let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
            for s in input.trim_end().split(',') {
                if let Some(label) = s.strip_suffix('-') {
                    let b = hash(label);
                    if let Some(idx) = boxes[b].iter().position(|lens| lens.0 == label) {
                        boxes[b].remove(idx);
                    }
                } else {
                    assert!(s.contains('='));
                    let (label, focal) = s.split_once('=').unwrap();
                    let b = hash(label);
                    let f = focal.parse::<usize>().unwrap();
                    if let Some(lens) = boxes[b].iter_mut().find(|lens| lens.0 == label) {
                        lens.1 = f;
                    } else {
                        boxes[b].push((label, f));
                    }
                }
            }
            boxes
                .iter()
                .enumerate()
                .map(|(i, lenses)| {
                    lenses
                        .iter()
                        .enumerate()
                        .map(|(j, (_label, focal))| (i + 1) * (j + 1) * focal)
                        .sum::<usize>()
                })
                .sum::<usize>()
        }
    }
    .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "1320");
    }
    #[test]
    fn example_part2() {
        assert_eq!(solver(Part::Part2, include_str!("../example.txt")), "145");
    }
}

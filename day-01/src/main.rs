use aoc_commons::{solve_aoc, Part};

const SPELLED_OUT: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];
fn main() {
    solve_aoc(include_str!("../input.txt"), solver);
}

fn solver(part: Part, input: &str) -> String {
    use Part::*;
    // For each input line we extract the digits in a Vec
    let calibrations: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    c.to_digit(10).or_else(|| {
                        if part == Part2 {
                            SPELLED_OUT.iter().find_map(|&(letter, digit)| {
                                line[i..].starts_with(letter).then_some(digit)
                            })
                        } else {
                            None
                        }
                    })
                })
                .collect()
        })
        .collect();
    calibrations
        .iter()
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.txt")),
            "142"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "281"
        );
    }
}

use aoc_commons::Part;

fn next_in_seq(values: &[i64]) -> i64 {
    if values.iter().all(|v| v == &0) {
        0
    } else {
        let new_values = values.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
        next_in_seq(&new_values) + values.last().unwrap()
    }
}
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}
pub fn solver(part: Part, input: &str) -> String {
    let values = parse(input);
    match part {
        Part::Part1 => values
            .into_iter()
            .map(|rows| next_in_seq(&rows))
            .sum::<i64>(),
        Part::Part2 => values
            .into_iter()
            .map(|mut rows| {
                rows.reverse();
                next_in_seq(&rows)
            })
            .sum::<i64>(),
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
            "114"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "2"
        );
    }
}

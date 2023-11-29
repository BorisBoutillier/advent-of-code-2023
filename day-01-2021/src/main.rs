use aoc_commons::{solve_aoc, Part};

fn main() {
    solve_aoc(
        include_str!("../example.txt"),
        include_str!("../input.txt"),
        solver,
    );
}

fn solver(part: Part, input: String) -> String {
    let values = input
        .lines()
        .map(|d| d.parse::<u32>().expect("not a number"))
        .collect::<Vec<_>>();
    match part {
        Part::Part1 => values.windows(2).filter(|v| v[1] > v[0]).count(),
        Part::Part2 => values
            .windows(3)
            .map(|v| v[0] + v[1] + v[2])
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|v| v[1] > v[0])
            .count(),
    }
    .to_string()
}

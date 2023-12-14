use aoc_commons::Part;

fn find_h_mirror(pat: &[Vec<char>], n_error: usize) -> usize {
    for row in 0..(pat.len() - 1) {
        let up = (0..=row).rev();
        let down = (row + 1)..(pat.len());
        let diffs = up
            .zip(down)
            .map(|(r1, r2)| {
                pat[r1]
                    .iter()
                    .zip(pat[r2].iter())
                    .map(|(c1, c2)| (c1 != c2) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>();
        if n_error == diffs {
            return row + 1;
        }
    }
    0
}

struct Pattern {
    rows: Vec<Vec<char>>,
}
impl Pattern {
    fn parse(input: &str) -> Self {
        Pattern {
            rows: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }
    fn find_horizontal_mirror(&self, n_error: usize) -> usize {
        find_h_mirror(&self.rows, n_error)
    }
    fn find_vertical_mirror(&self, n_error: usize) -> usize {
        // Compute cols
        let n_col = self.rows[0].len();
        let mut cols = vec![vec![]; n_col];
        for row in self.rows.iter() {
            for (j, c) in row.iter().enumerate() {
                cols[j].push(*c);
            }
        }
        find_h_mirror(&cols, n_error)
    }
    fn compute(&self, part: Part) -> usize {
        let n_error = match part {
            Part::Part1 => 0,
            Part::Part2 => 1,
        };
        self.find_horizontal_mirror(n_error) * 100 + self.find_vertical_mirror(n_error)
    }
}
pub fn solver(part: Part, input: &str) -> String {
    input
        .split("\n\n")
        .map(|s| Pattern::parse(s).compute(part))
        .sum::<usize>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.txt")),
            "405"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part1.txt")),
            "400"
        );
    }
}

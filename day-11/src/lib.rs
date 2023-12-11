use aoc_commons::Part;

pub fn solver(part: Part, input: &str) -> String {
    let expansion = match part {
        Part::Part1 => 1,
        Part::Part2 => 1000000 - 1,
    };
    let mut galaxies = vec![];
    let mut empty_cols = vec![];
    let mut cur_expanded_y = 0i64;
    for line in input.lines() {
        if empty_cols.is_empty() {
            empty_cols = vec![true; line.len()]
        }
        let mut row_galaxies = vec![];
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    empty_cols[x] = false;
                    row_galaxies.push((x as i64, cur_expanded_y));
                }
                '.' => (),
                _ => panic!("{c}"),
            }
        }
        cur_expanded_y += 1 + if row_galaxies.is_empty() {
            expansion
        } else {
            0
        };
        galaxies.extend_from_slice(&row_galaxies);
    }
    let mut empty_cols_before = vec![];
    let mut n_empty_before = 0;
    for is_empty in empty_cols.into_iter() {
        empty_cols_before.push(n_empty_before);
        if is_empty {
            n_empty_before += 1;
        }
    }
    for (x, _) in galaxies.iter_mut() {
        *x += empty_cols_before[*x as usize] * expansion;
    }
    let mut sum = 0;
    for (i, (x1, y1)) in galaxies.iter().enumerate() {
        for (x2, y2) in galaxies[(i + 1)..].iter() {
            sum += (x2 - x1).abs() + (y2 - y1).abs();
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
            "374"
        );
    }
}

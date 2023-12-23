use aoc_commons::*;

pub fn search(
    part: Part,
    start: (usize, usize),
    map: &[Vec<char>],
    seen: &mut [Vec<bool>],
) -> Option<usize> {
    let mut cur = start;
    let mut count = 0;
    loop {
        let (x, y) = cur;
        count += 1;
        seen[y][x] = true;
        if y == map.len() - 1 {
            return Some(count);
        }
        let mut nexts = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;
                let mut cond = !seen[new_y][new_x] && map[new_y][new_x] != '#';
                if part == Part::Part1 {
                    cond &= map[new_y][new_x]
                        != match (dx, dx) {
                            (0, -1) => 'v',
                            (0, 1) => '^',
                            (-1, 0) => '<',
                            (1, 0) => '>',
                            _ => panic!(),
                        };
                }
                cond.then_some((new_x, new_y))
            })
            .collect::<Vec<_>>();

        if nexts.is_empty() {
            return None;
        }
        if nexts.len() == 1 {
            cur = nexts.pop().unwrap()
        } else {
            return nexts
                .into_iter()
                .filter_map(|next| {
                    let mut my_seen = seen.to_vec();
                    search(part, next, map, &mut my_seen)
                })
                .max()
                .map(|v| count + v);
        }
    }
}
pub fn solver(part: Part, input: &str) -> String {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen = vec![vec![false; map[0].len()]; map.len()];
    seen[0][1] = true;
    let count = search(part, (1, 1), &map, &mut seen).unwrap();

    count.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "94");
    }
    //#[test]
    //fn example_part2() {
    //    assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "7");
    //}
}

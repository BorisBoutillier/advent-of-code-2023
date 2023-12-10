use std::collections::HashSet;

use aoc_commons::Part;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}
impl Direction {
    fn inv(&self) -> Self {
        use Direction::*;
        match self {
            N => S,
            S => N,
            E => W,
            W => E,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pipe {
    WN,
    WE,
    WS,
    NS,
    NE,
    SE,
}
//
//   X ->
//  Y
//  |
//  v
//
impl Pipe {
    pub fn parse(c: char) -> Option<Self> {
        use Pipe::*;
        match c {
            '|' => Some(NS),
            '-' => Some(WE),
            'F' => Some(SE),
            'L' => Some(NE),
            '7' => Some(WS),
            'J' => Some(WN),
            _ => None,
        }
    }
    pub fn from_directions(d1: Direction, d2: Direction) -> Self {
        use Direction::*;
        use Pipe::*;
        match (d1, d2) {
            (N, E) | (E, N) => NE,
            (N, W) | (W, N) => WN,
            (N, S) | (S, N) => NS,
            (S, E) | (E, S) => SE,
            (S, W) | (W, S) => WS,
            (W, E) | (E, W) => WE,
            _ => panic!("What {d1:?} ,{d2:?}"),
        }
    }
    pub fn next(&self, x: i32, y: i32, from: Direction) -> Option<(i32, i32, Direction)> {
        use Direction::*;
        use Pipe::*;
        match (self, from) {
            (NS, N) => Some((x, y + 1, N)),
            (NS, S) => Some((x, y - 1, S)),
            (NE, N) => Some((x + 1, y, W)),
            (NE, E) => Some((x, y - 1, S)),
            (WE, W) => Some((x + 1, y, W)),
            (WE, E) => Some((x - 1, y, E)),
            (WN, W) => Some((x, y - 1, S)),
            (WN, N) => Some((x - 1, y, E)),
            (WS, W) => Some((x, y + 1, N)),
            (WS, S) => Some((x - 1, y, E)),
            (SE, S) => Some((x + 1, y, W)),
            (SE, E) => Some((x, y + 1, N)),
            _ => None,
        }
    }
}
pub fn solver(part: Part, input: &str) -> String {
    let mut start = None;
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Some((x, y));
                        None
                    } else {
                        Pipe::parse(c)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let n_row = grid.len() as i32;
    let n_col = grid[0].len() as i32;
    let (start_x, start_y) = start.unwrap();
    for (dx, dy, start_direction) in [
        (0, -1, Direction::S),
        (1, 0, Direction::W),
        (0, 1, Direction::N),
        (-1, 0, Direction::E),
    ] {
        let mut count = 0;
        let mut loop_tiles = HashSet::from([(start_x, start_y)]);
        let (mut cur_x, mut cur_y, mut cur_direction) =
            (start_x as i32 + dx, start_y as i32 + dy, start_direction);
        let found = loop {
            count += 1;
            if cur_x < 0 || cur_x >= n_col || cur_y < 0 || cur_y >= n_row {
                break false;
            }
            if cur_x == start_x as i32 && cur_y == start_y as i32 {
                grid[start_y][start_x] =
                    Some(Pipe::from_directions(cur_direction, start_direction.inv()));
                break true;
            }
            loop_tiles.insert((cur_x as usize, cur_y as usize));
            if let Some(pipe) = grid[cur_y as usize][cur_x as usize] {
                if let Some((x, y, direction)) = pipe.next(cur_x, cur_y, cur_direction) {
                    (cur_x, cur_y, cur_direction) = (x, y, direction);
                } else {
                    break false;
                }
            } else {
                break false;
            }
        };
        if found {
            match part {
                Part::Part1 => return (count / 2).to_string(),
                Part::Part2 => {
                    let mut inside = 0;
                    let mut last = Pipe::NS;
                    for y in 0..n_row as usize {
                        let mut is_inside = false;
                        for x in 0..n_col as usize {
                            if loop_tiles.contains(&(x, y)) {
                                match grid[y][x] {
                                    Some(Pipe::NS) => {
                                        is_inside = !is_inside;
                                    }
                                    Some(Pipe::WE) => (),
                                    Some(Pipe::NE) => {
                                        last = Pipe::NE;
                                    }
                                    Some(Pipe::SE) => {
                                        last = Pipe::SE;
                                    }
                                    Some(Pipe::WN) => {
                                        if last == Pipe::SE {
                                            is_inside = !is_inside;
                                        } else {
                                            assert!(last == Pipe::NE);
                                        }
                                    }
                                    Some(Pipe::WS) => {
                                        if last == Pipe::NE {
                                            is_inside = !is_inside;
                                        } else {
                                            assert!(last == Pipe::SE);
                                        }
                                    }
                                    None => (),
                                }
                            } else if is_inside {
                                inside += 1;
                            }
                        }
                    }
                    return inside.to_string();
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.txt")),
            "4"
        );
    }
    #[test]
    fn example_part1_2() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.2.txt")),
            "8"
        );
    }
    #[test]
    fn example_part2_1() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "4"
        );
    }
    #[test]
    fn example_part2_2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.2.txt")),
            "4"
        );
    }
    #[test]
    fn example_part2_3() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.3.txt")),
            "8"
        );
    }
}

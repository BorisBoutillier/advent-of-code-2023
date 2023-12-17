use std::collections::HashSet;

use aoc_commons::Part;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    N,
    W,
    S,
    E,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Tile {
    Empty,
    MirrorNWSE,
    MirrorNESW,
    SplitNS,
    SplitEW,
}
impl Tile {
    fn parse(c: char) -> Tile {
        use Tile::*;
        match c {
            '.' => Empty,
            '\\' => MirrorNWSE,
            '/' => MirrorNESW,
            '|' => SplitNS,
            '-' => SplitEW,
            _ => panic!(),
        }
    }
    fn propagate(&self, x: isize, y: isize, dir: Direction) -> Vec<(isize, isize, Direction)> {
        use Direction::*;
        use Tile::*;
        match (self, dir) {
            (Empty, N) => vec![(x, y + 1, N)],
            (Empty, S) => vec![(x, y - 1, S)],
            (Empty, W) => vec![(x + 1, y, W)],
            (Empty, E) => vec![(x - 1, y, E)],
            (MirrorNESW, N) => vec![(x - 1, y, E)],
            (MirrorNESW, S) => vec![(x + 1, y, W)],
            (MirrorNESW, W) => vec![(x, y - 1, S)],
            (MirrorNESW, E) => vec![(x, y + 1, N)],
            (MirrorNWSE, N) => vec![(x + 1, y, W)],
            (MirrorNWSE, S) => vec![(x - 1, y, E)],
            (MirrorNWSE, W) => vec![(x, y + 1, N)],
            (MirrorNWSE, E) => vec![(x, y - 1, S)],
            (SplitNS, N) => vec![(x, y + 1, N)],
            (SplitNS, S) => vec![(x, y - 1, S)],
            (SplitNS, W) => vec![(x, y + 1, N), (x, y - 1, S)],
            (SplitNS, E) => vec![(x, y + 1, N), (x, y - 1, S)],
            (SplitEW, N) => vec![(x + 1, y, W), (x - 1, y, E)],
            (SplitEW, S) => vec![(x + 1, y, W), (x - 1, y, E)],
            (SplitEW, W) => vec![(x + 1, y, W)],
            (SplitEW, E) => vec![(x - 1, y, E)],
        }
    }
}
pub fn solver(part: Part, input: &str) -> String {
    use Direction::*;
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect())
        .collect();
    let len_y = tiles.len() as isize;
    let len_x = tiles[0].len() as isize;

    let starters = match part {
        Part::Part1 => vec![(0, 0, W)],
        Part::Part2 => (0..len_x)
            .flat_map(|x| vec![(x, 0, N), (x, len_y - 1, S)])
            .chain((0..len_y).flat_map(|y| vec![(0, y, W), (len_x - 1, y, E)]))
            .collect(),
    };
    starters
        .into_iter()
        .map(|starter| {
            let mut cur = vec![starter];
            let mut seen_tiles = HashSet::new();
            let mut seen_beam = HashSet::new();
            while let Some((x, y, dir)) = cur.pop() {
                if x < 0 || y < 0 || x >= len_x || y >= len_y || seen_beam.contains(&(x, y, dir)) {
                    continue;
                }
                seen_beam.insert((x, y, dir));
                seen_tiles.insert((x, y));
                cur.extend(tiles[y as usize][x as usize].propagate(x, y, dir));
            }
            seen_tiles.len()
        })
        .max()
        .unwrap()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solver(Part::Part1, include_str!("../example.txt")), "46");
    }
    #[test]
    fn example_part2() {
        assert_eq!(solver(Part::Part2, include_str!("../example.txt")), "51");
    }
}

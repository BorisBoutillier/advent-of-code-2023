use aoc_commons::{solve_aoc, Part};

fn main() {
    solve_aoc(include_str!("../input.txt"), solver);
}
struct Pick {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}
impl Pick {
    pub fn parse(s: &str) -> Pick {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for s1 in s.split(',') {
            let mut split = s1.trim().split(' ');
            let v = split.next().unwrap().parse::<u32>().expect("Not a number");
            let color = split.next().unwrap();
            match color {
                "red" => red = v,
                "blue" => blue = v,
                "green" => green = v,
                _ => panic!("Unexpected color '{color}'"),
            }
        }
        Pick { red, blue, green }
    }
    pub fn is_compatible(&self, red: u32, blue: u32, green: u32) -> bool {
        self.red <= red && self.blue <= blue && self.green <= green
    }
}
struct Game {
    id: u32,
    picks: Vec<Pick>,
}

impl Game {
    pub fn parse(line: &str) -> Game {
        let mut split = line.trim().split(':');
        let id = split
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let picks = split
            .next()
            .unwrap()
            .split(';')
            .map(Pick::parse)
            .collect::<Vec<_>>();
        Game { id, picks }
    }
    pub fn is_compatible(&self, red: u32, blue: u32, green: u32) -> bool {
        self.picks
            .iter()
            .all(|pick| pick.is_compatible(red, blue, green))
    }
    pub fn minimum_set_power(&self) -> u32 {
        let (min_red, min_blue, min_green) =
            self.picks
                .iter()
                .fold((0, 0, 0), |(red, blue, green), pick| {
                    (
                        red.max(pick.red),
                        blue.max(pick.blue),
                        green.max(pick.green),
                    )
                });
        min_red * min_blue * min_green
    }
}
fn solver(part: Part, input: &str) -> String {
    use Part::*;
    let games = input.lines().map(Game::parse).collect::<Vec<_>>();
    if part == Part1 {
        games
            .iter()
            .filter(|game| game.is_compatible(12, 14, 13))
            .map(|game| game.id)
            .sum::<u32>()
            .to_string()
    } else {
        games
            .iter()
            .map(|game| game.minimum_set_power())
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.txt")),
            "8"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "2286"
        );
    }
}

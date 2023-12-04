use std::collections::HashSet;

use aoc_commons::Part;

#[derive(Debug)]
struct Card {
    matching: u32,
}

impl Card {
    pub fn parse(line: &str) -> Card {
        let numbers = line.split(':').nth(1).unwrap();
        let mut split = numbers.split('|');
        let winner = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let have = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let matching = winner.intersection(&have).count() as u32;
        Card { matching }
    }

    pub fn score(&self) -> u32 {
        match self.matching {
            0 => 0,
            _ => 1 << (self.matching - 1),
        }
    }
    pub fn n_winner(&self) -> u32 {
        self.matching
    }
}

pub fn solver(part: Part, input: &str) -> String {
    use Part::*;
    let cards = input.lines().map(Card::parse).collect::<Vec<_>>();
    match part {
        Part1 => cards.iter().map(|card| card.score()).sum(),
        Part2 => {
            let mut treated_cards = 0;
            let mut owned_cards = vec![1; cards.len()];
            for i in 0..owned_cards.len() {
                treated_cards += owned_cards[i];
                for j in i + 1..i + 1 + cards[i].n_winner() as usize {
                    owned_cards[j] += owned_cards[i];
                }
            }
            treated_cards
        }
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
            "13"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "30"
        );
    }
}

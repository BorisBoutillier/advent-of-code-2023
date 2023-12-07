use std::collections::HashMap;

use aoc_commons::Part;
const CARDS_P1: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARDS_P2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Clone, PartialEq, Eq, Hash)]
struct Hand {
    part: Part,
    cards: Vec<char>,
    sets: Vec<usize>,
}
impl Hand {
    fn parse(part: Part, s: &str) -> Hand {
        let cards = s.chars().collect::<Vec<_>>();
        assert_eq!(cards.len(), 5);
        assert!(cards.iter().all(|c| CARDS_P1.contains(c)));
        let mut card_counts = HashMap::<char, usize>::new();
        let mut n_joker = 0;
        for card in cards.iter() {
            if part == Part::Part2 && card == &'J' {
                n_joker += 1;
            } else {
                *card_counts.entry(*card).or_insert(0) += 1;
            }
        }
        let mut sets = vec![0; 6];
        sets[0] = 1; // There is a least one card with 0 count. Avoids a corner case for JJJJJ.
        for v in card_counts.into_values() {
            sets[v] += 1;
        }
        if n_joker > 0 {
            let best = sets.iter().rev().position(|v| v != &0).unwrap();
            sets[5 - best] -= 1;
            sets[5 - best + n_joker] += 1;
        }
        Hand { part, cards, sets }
    }
    fn sort_key(&self) -> (Vec<usize>, Vec<usize>) {
        let card_value = if self.part == Part::Part1 {
            CARDS_P1
        } else {
            CARDS_P2
        };
        let card_index = self
            .cards
            .iter()
            .map(|c| card_value.iter().position(|card| card == c).unwrap())
            .collect();
        // number of 5 of a kind is better than number of singles
        let mut rev_sets = self.sets.clone();
        rev_sets.reverse();
        (rev_sets, card_index)
    }
}
impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards_s = self.cards.iter().collect::<String>();
        let sets_s = self
            .sets
            .iter()
            .enumerate()
            .map(|(i, &v)| (0..v).map(|_| i.to_string()).collect::<String>())
            .collect::<String>();
        write!(f, "Hand: {} ; {}", cards_s, sets_s)
    }
}
pub fn solver(part: Part, input: &str) -> String {
    let mut hands = Vec::new();
    let mut bids = HashMap::new();
    for line in input.lines() {
        let (hand_s, bid_s) = line.split_once(' ').unwrap();
        let hand = Hand::parse(part, hand_s);
        hands.push(hand.clone());
        bids.insert(hand, bid_s.parse::<u32>().unwrap());
    }
    hands.sort_by_key(|hand| hand.sort_key());
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * *bids.get(hand).unwrap())
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
            "6440"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "5905"
        );
    }
}

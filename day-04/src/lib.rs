use aoc_commons::Part;

pub fn parse_n_winner(line: &str) -> usize {
    let numbers = line.split(':').nth(1).unwrap();
    let mut split = numbers.split('|');
    let winning = split
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let n_winner = split
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .filter(|number| winning.contains(number))
        .count();
    n_winner
}

pub fn solver(part: Part, input: &str) -> String {
    use Part::*;
    let n_winner_iter = input.lines().map(parse_n_winner);
    match part {
        Part1 => n_winner_iter
            .map(|n_winner| match n_winner {
                0 => 1,
                _ => 1 << (n_winner - 1),
            })
            .sum(),
        Part2 => {
            n_winner_iter
                .fold(
                    (0, vec![]),
                    |(mut treated_cards, mut owned_cards), n_winner| {
                        owned_cards.resize((1 + n_winner).max(owned_cards.len()), 1);
                        let owned = owned_cards.remove(0);
                        treated_cards += owned;
                        for owned_card in owned_cards[0..n_winner].iter_mut() {
                            *owned_card += owned;
                        }
                        (treated_cards, owned_cards)
                    },
                )
                .0
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

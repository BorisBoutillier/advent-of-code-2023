use aoc_commons::Part;
#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}
impl Race {
    fn n_winner(&self) -> u64 {
        let disciminant = (self.time * self.time - 4 * self.record) as f32;
        if disciminant.is_sign_positive() {
            let s1 = (self.time as f32 + f32::sqrt(disciminant)) / 2.0;
            let s2 = (self.time as f32 - f32::sqrt(disciminant)) / 2.0;
            s1.ceil() as u64 - s2.floor() as u64 - 1
        } else {
            0
        }
    }
}
//Distance per time waited
//solve for x: x*(time-x) == record
//           : -x^2 + x*time - record == 0
// ax^2 + bx + c = 0  solutions depends on b^2-4ac sign
// here b^2 - 4ac = time^2 - 4 * record
fn parse(part: Part, input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace();
    let records = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace();
    match part {
        Part::Part1 => times
            .zip(records)
            .map(|(time, record)| Race {
                time: time.parse::<u64>().unwrap(),
                record: record.parse::<u64>().unwrap(),
            })
            .collect(),
        Part::Part2 => {
            let time = times.collect::<Vec<_>>().concat().parse::<u64>().unwrap();
            let record = records.collect::<Vec<_>>().concat().parse::<u64>().unwrap();
            vec![Race { time, record }]
        }
    }
}
pub fn solver(part: Part, input: &str) -> String {
    let races = parse(part, input);
    races
        .iter()
        .map(|race| race.n_winner())
        .product::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.txt")),
            "288"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "71503"
        );
    }
}

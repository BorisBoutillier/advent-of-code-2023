use std::{ops::Range, str::Lines};

use aoc_commons::Part;

struct MapEntry {
    src: Range<i64>,
    diff: i64,
}
impl MapEntry {
    pub fn parse(line: &str) -> MapEntry {
        let values = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().expect("Not a number"))
            .collect::<Vec<_>>();
        assert_eq!(values.len(), 3);
        MapEntry {
            src: Range {
                start: values[1],
                end: values[1] + values[2],
            },
            diff: values[0] - values[1],
        }
    }
    pub fn src_start(&self) -> i64 {
        self.src.start
    }
    pub fn map(&self, range: &Range<i64>) -> (Vec<Range<i64>>, Vec<Range<i64>>) {
        if self.src.contains(&range.start) {
            if range.end <= self.src.end {
                (
                    vec![],
                    vec![Range {
                        start: range.start + self.diff,
                        end: range.end + self.diff,
                    }],
                )
            } else {
                (
                    vec![Range {
                        start: self.src.end,
                        end: range.end,
                    }],
                    vec![Range {
                        start: range.start + self.diff,
                        end: self.src.end + self.diff,
                    }],
                )
            }
        } else if range.contains(&self.src.start) {
            if self.src.end < range.end {
                (
                    vec![
                        Range {
                            start: range.start,
                            end: self.src.start,
                        },
                        Range {
                            start: self.src.end,
                            end: range.end,
                        },
                    ],
                    vec![Range {
                        start: self.src.start + self.diff,
                        end: self.src.end + self.diff,
                    }],
                )
            } else {
                (
                    vec![Range {
                        start: range.start,
                        end: self.src.start,
                    }],
                    vec![Range {
                        start: self.src.start + self.diff,
                        end: range.end + self.diff,
                    }],
                )
            }
        } else {
            (vec![range.clone()], vec![])
        }
    }
}
struct Map {
    entries: Vec<MapEntry>,
}
impl Map {
    fn parse(lines: &mut Lines) -> Map {
        let mut entries = vec![];
        let header = lines.next().expect("");
        assert!(header.contains(" map:"));
        for line in lines {
            if line.is_empty() {
                break;
            }
            entries.push(MapEntry::parse(line));
        }
        entries.sort_by_key(|entry: &MapEntry| entry.src_start());
        Map { entries }
    }
    fn map_one(&self, range: &Range<i64>) -> Vec<Range<i64>> {
        let mut current = vec![range.clone()];
        let mut output = vec![];
        for entry in self.entries.iter() {
            let mut remains = vec![];
            for r in current.iter() {
                let (remain, mapped) = entry.map(r);
                remains.extend_from_slice(&remain);
                output.extend_from_slice(&mapped)
            }
            current = remains;
        }

        [output, current].concat()
    }
    pub fn map(&self, input: &[Range<i64>]) -> Vec<Range<i64>> {
        input.iter().flat_map(|range| self.map_one(range)).collect()
    }
}
struct Almanac {
    seeds: Vec<Range<i64>>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}
impl Almanac {
    pub fn parse_seeds(part: Part, line: &str) -> Vec<Range<i64>> {
        let values = line
            .split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        match part {
            Part::Part1 => values
                .into_iter()
                .map(|v| Range {
                    start: v,
                    end: v + 1,
                })
                .collect(),
            Part::Part2 => values
                .chunks(2)
                .map(|c| Range {
                    start: c[0],
                    end: c[0] + c[1],
                })
                .collect(),
        }
    }
    pub fn parse(part: Part, input: &str) -> Almanac {
        let mut lines = input.lines();
        let seeds = Self::parse_seeds(part, lines.next().expect(""));
        lines.next();
        Almanac {
            seeds,
            seed_to_soil: Map::parse(&mut lines),
            soil_to_fertilizer: Map::parse(&mut lines),
            fertilizer_to_water: Map::parse(&mut lines),
            water_to_light: Map::parse(&mut lines),
            light_to_temperature: Map::parse(&mut lines),
            temperature_to_humidity: Map::parse(&mut lines),
            humidity_to_location: Map::parse(&mut lines),
        }
    }
    pub fn smallest_location(&self) -> i64 {
        let soil = self.seed_to_soil.map(&self.seeds);
        let fertilizer = self.soil_to_fertilizer.map(&soil);
        let water = self.fertilizer_to_water.map(&fertilizer);
        let light = self.water_to_light.map(&water);
        let temperature = self.light_to_temperature.map(&light);
        let humidity = self.temperature_to_humidity.map(&temperature);
        let location = self.humidity_to_location.map(&humidity);

        location.iter().map(|range| range.start).min().unwrap()
    }
}
pub fn solver(part: Part, input: &str) -> String {
    let almanac = Almanac::parse(part, input);
    almanac.smallest_location().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(
            solver(Part::Part1, include_str!("../example.part1.txt")),
            "35"
        );
    }
    #[test]
    fn example_part2() {
        assert_eq!(
            solver(Part::Part2, include_str!("../example.part2.txt")),
            "46"
        );
    }
}

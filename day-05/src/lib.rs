use std::str::Lines;

use aoc_commons::Part;
use range_set_blaze::prelude::*;

struct MapEntry {
    src_start: i64,
    src_end: i64,
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
            src_start: values[1],
            src_end: values[1] + values[2],
            diff: values[0] - values[1],
        }
    }
    pub fn src_start(&self) -> i64 {
        self.src_start
    }
    pub fn map(&self, range: RangeSetBlaze<i64>) -> (RangeSetBlaze<i64>, RangeSetBlaze<i64>) {
        let mut lower = range;
        let mut middle = lower.split_off(self.src_start);
        let upper = middle.split_off(self.src_end);
        let mapped = middle
            .ranges()
            .map(|range| (range.start() + self.diff)..=(range.end() + self.diff))
            .collect();
        let remains = lower | upper;
        (remains, mapped)
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
    pub fn map(&self, input: RangeSetBlaze<i64>) -> RangeSetBlaze<i64> {
        let mut current = input.clone();
        let mut output = vec![];
        for entry in self.entries.iter() {
            let (remains, mapped) = entry.map(current);
            output.push(mapped);
            current = remains;
        }
        // Remaining input are mapped without translation
        output.push(current);
        output.union()
    }
}
struct Almanac {
    seeds: RangeSetBlaze<i64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}
impl Almanac {
    pub fn parse_seeds(part: Part, line: &str) -> RangeSetBlaze<i64> {
        let values = line
            .split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        match part {
            Part::Part1 => values.into_iter().collect(),
            Part::Part2 => values.chunks(2).map(|c| c[0]..=(c[0] + c[1] - 1)).collect(),
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
        let soil = self.seed_to_soil.map(self.seeds.clone());
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        let location = self.humidity_to_location.map(humidity);

        location.ranges().map(|range| *range.start()).min().unwrap()
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

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Range;
use std::sync::OnceLock;

use crate::input::{read_lines, FilterNotEmpty};
use crate::Solution;

mod input;

fn seed_to_soil_map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| Map::from(read_lines(input::SEED_TO_SOIL_MAP).filter_not_empty()))
}

fn soil_to_fertilizer_map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| Map::from(read_lines(input::SOIL_TO_FERTILIZER_MAP).filter_not_empty()))
}

fn fertilizer_to_water_map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| Map::from(read_lines(input::FERTILIZER_TO_WATER_MAP).filter_not_empty()))
}

fn water_to_light_map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| Map::from(read_lines(input::WATER_TO_LIGHT_MAP).filter_not_empty()))
}

fn light_to_temperature_map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| Map::from(read_lines(input::LIGHT_TO_TEMPERATURE_MAP).filter_not_empty()))
}

fn temperature_to_humidity_map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| Map::from(read_lines(input::TEMPERATURE_TO_HUMIDITY_MAP).filter_not_empty()))
}

fn humidity_to_location_map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| Map::from(read_lines(input::HUMIDITY_TO_LOCATION_MAP).filter_not_empty()))
}

pub struct Day5;

impl Solution for Day5 {
    fn day(&self) -> u8 {
        5
    }

    fn part_one(&self) -> String {
        let maps = &[
            seed_to_soil_map(),
            soil_to_fertilizer_map(),
            fertilizer_to_water_map(),
            water_to_light_map(),
            light_to_temperature_map(),
            temperature_to_humidity_map(),
            humidity_to_location_map(),
        ];
        let min_location = input::SEEDS
            .iter()
            .map(|seed| map_all(maps, *seed))
            .min()
            .unwrap();
        format!("Minimal location: {}", min_location)
    }

    fn part_two(&self) -> String {
        let maps = &[
            seed_to_soil_map(),
            soil_to_fertilizer_map(),
            fertilizer_to_water_map(),
            water_to_light_map(),
            light_to_temperature_map(),
            temperature_to_humidity_map(),
            humidity_to_location_map(),
        ];
        let ranges = input::SEEDS
            .iter()
            .tuples()
            .map(|(start, length)| *start..(start + length))
            .collect::<Vec<_>>();
        let min_location = map_range_all(maps, ranges)
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap();
        format!("Minimal location with ranges: {}", min_location)
    }
}

fn map_all(maps: &[&Map], source: u32) -> u32 {
    maps.iter().fold(source, |value, map| map.map(value))
}

fn map_range_all(maps: &[&Map], ranges: Vec<Range<u32>>) -> Vec<Range<u32>> {
    maps.iter().fold(ranges, |ranges, map| {
        ranges
            .into_iter()
            .flat_map(|range| map.map_range(range))
            .collect()
    })
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct MapEntry {
    source_start: u32,
    target_start: u32,
    range_length: u32,
}

impl Ord for MapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_start.cmp(&other.source_start)
    }
}

impl PartialOrd for MapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl MapEntry {
    fn source_end(&self) -> u32 {
        self.source_start.saturating_add(self.range_length)
    }

    fn try_match(&self, source: u32) -> Option<u32> {
        if self.matches(source) {
            Some(self.map(source))
        } else {
            None
        }
    }

    fn matches(&self, source: u32) -> bool {
        source >= self.source_start && source - self.source_start < self.range_length
    }

    fn map(&self, source: u32) -> u32 {
        source - self.source_start + self.target_start
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Map(BTreeSet<MapEntry>);

impl Map {
    fn map(&self, source: u32) -> u32 {
        self.0
            .iter()
            .find_map(|entry| entry.try_match(source))
            .unwrap_or(source)
    }

    fn map_range(&self, range: Range<u32>) -> Vec<Range<u32>> {
        let mut result = vec![];

        let mut current = range.start;
        let mut entries = self.0.iter();
        let mut entry = entries.next();
        while current < range.end {
            match entry {
                Some(map_entry) if map_entry.source_end() <= current => {
                    entry = entries.next();
                }
                Some(entry) if entry.source_start < range.end => {
                    if current < entry.source_start {
                        result.push(current..entry.source_start);
                        current = entry.source_start;
                    }
                    let start = entry.map(current);
                    let length = (entry.range_length - (current - entry.source_start))
                        .min(range.end - current);
                    let end = start.saturating_add(length);
                    result.push(start..end);
                    current = current.saturating_add(length);
                }
                _ => {
                    result.push(current..range.end);
                    break;
                }
            }
        }

        result
    }
}

impl<I: IntoIterator<Item = String>> From<I> for Map {
    fn from(value: I) -> Self {
        Map(value
            .into_iter()
            .map(|entry: String| {
                let numbers = entry
                    .split(' ')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u32>>();
                MapEntry {
                    source_start: numbers[1],
                    target_start: numbers[0],
                    range_length: numbers[2],
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_seed_to_soil_map() -> &'static Map {
        static MAP: OnceLock<Map> = OnceLock::new();
        MAP.get_or_init(|| {
            Map::from(
                read_lines(
                    b"
50 98 2
52 50 48
"
                    .as_slice(),
                )
                .filter_not_empty(),
            )
        })
    }

    fn example_soil_to_fertilizer_map() -> &'static Map {
        static MAP: OnceLock<Map> = OnceLock::new();
        MAP.get_or_init(|| {
            Map::from(
                read_lines(
                    b"
0 15 37
37 52 2
39 0 15
"
                    .as_slice(),
                )
                .filter_not_empty(),
            )
        })
    }

    fn example_fertilizer_to_water_map() -> &'static Map {
        static MAP: OnceLock<Map> = OnceLock::new();
        MAP.get_or_init(|| {
            Map::from(
                read_lines(
                    b"
49 53 8
0 11 42
42 0 7
57 7 4
"
                    .as_slice(),
                )
                .filter_not_empty(),
            )
        })
    }

    fn example_water_to_light_map() -> &'static Map {
        static MAP: OnceLock<Map> = OnceLock::new();
        MAP.get_or_init(|| {
            Map::from(
                read_lines(
                    b"
88 18 7
18 25 70
"
                    .as_slice(),
                )
                .filter_not_empty(),
            )
        })
    }

    fn example_light_to_temperature_map() -> &'static Map {
        static MAP: OnceLock<Map> = OnceLock::new();
        MAP.get_or_init(|| {
            Map::from(
                read_lines(
                    b"
45 77 23
81 45 19
68 64 13
"
                    .as_slice(),
                )
                .filter_not_empty(),
            )
        })
    }

    fn example_temperature_to_humidity_map() -> &'static Map {
        static MAP: OnceLock<Map> = OnceLock::new();
        MAP.get_or_init(|| {
            Map::from(
                read_lines(
                    b"
0 69 1
1 0 69
"
                    .as_slice(),
                )
                .filter_not_empty(),
            )
        })
    }

    fn example_humidity_to_location_map() -> &'static Map {
        static MAP: OnceLock<Map> = OnceLock::new();
        MAP.get_or_init(|| {
            Map::from(
                read_lines(
                    b"
60 56 37
56 93 4
"
                    .as_slice(),
                )
                .filter_not_empty(),
            )
        })
    }

    #[test]
    fn parse_example() {
        assert_eq!(
            example_seed_to_soil_map(),
            &Map(BTreeSet::from([
                MapEntry {
                    source_start: 50,
                    target_start: 52,
                    range_length: 48,
                },
                MapEntry {
                    source_start: 98,
                    target_start: 50,
                    range_length: 2,
                },
            ])),
        );
    }

    #[test]
    fn mapping_seed_79_to_soil_should_return_81() {
        assert_eq!(example_seed_to_soil_map().map(79), 81);
    }

    #[test]
    fn map_single_range_before() {
        let map = Map::from(["200 50 10".into()]);

        assert_eq!(map.map_range(60..80), vec![60..80]);
    }

    #[test]
    fn map_single_range_after() {
        let map = Map::from(["200 50 10".into()]);

        assert_eq!(map.map_range(40..50), vec![40..50]);
    }

    #[test]
    fn map_single_range_around() {
        let map = Map::from(["200 50 10".into()]);

        assert_eq!(map.map_range(50..60), vec![200..210]);
    }

    #[test]
    fn map_single_range_inside() {
        let map = Map::from(["200 50 10".into()]);

        assert_eq!(map.map_range(40..70), vec![40..50, 200..210, 60..70]);
    }

    #[test]
    fn map_single_range_intersecting() {
        let map = Map::from(["200 50 10".into()]);

        assert_eq!(map.map_range(55..500), vec![205..210, 60..500]);
    }

    #[test]
    fn part2_example() {
        let maps = &[
            example_seed_to_soil_map(),
            example_soil_to_fertilizer_map(),
            example_fertilizer_to_water_map(),
            example_water_to_light_map(),
            example_light_to_temperature_map(),
            example_temperature_to_humidity_map(),
            example_humidity_to_location_map(),
        ];
        let seed_ranges = vec![79..(79 + 14), 55..(55 + 13)];

        let min_location = map_range_all(maps, seed_ranges)
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap();

        assert_eq!(min_location, 46);
    }
}

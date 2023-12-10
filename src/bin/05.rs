use itertools::Itertools;
use std::cmp::min;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct Range {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

#[derive(Debug)]
struct RangeParseError;

impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers_iter = s
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap());
        let destination_start = numbers_iter.next().unwrap();
        let source_start = numbers_iter.next().unwrap();
        let length = numbers_iter.next().unwrap();
        Ok(Range {
            source_start,
            destination_start,
            length,
        })
    }
}

impl Range {
    fn mapped_value(&self, value: u64) -> Option<u64> {
        if value >= self.source_start && value < self.source_start + self.length {
            Some(self.destination_start + (value - self.source_start))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct SeedRange {
    source_start: u64,
    length: u64,
}

#[derive(Debug, PartialEq)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn mapped_value(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if let Some(mapped_value) = range.mapped_value(value) {
                return mapped_value;
            }
        }
        value
    }
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct AlmanacParseError;

impl FromStr for Almanac {
    type Err = AlmanacParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seeds = Vec::new();
        let mut maps = Vec::new();

        let mut all_lines = s.lines();

        // seeds
        let mut lines = all_lines.by_ref().take_while(|line| !line.is_empty());
        for seed in lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .filter(|s| !s.is_empty())
        {
            seeds.push(seed.parse::<u64>().unwrap());
        }
        all_lines.next();

        for _ in 0..7 {
            let mut ranges = Vec::new();
            let mut lines = all_lines.by_ref().take_while(|line| !line.is_empty());
            lines.next();
            for range in lines.filter(|s| !s.is_empty()) {
                ranges.push(range.parse::<Range>().unwrap());
            }
            maps.push(Map { ranges });
        }
        Ok(Almanac { seeds, maps })
    }
}

#[derive(Debug, PartialEq)]
struct AlmanacSeedRange {
    seed_ranges: Vec<SeedRange>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct AlmanacSeedRangeParseError;

impl FromStr for AlmanacSeedRange {
    type Err = AlmanacSeedRangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seed_ranges = Vec::new();
        let mut maps = Vec::new();

        let mut all_lines = s.lines();

        // seeds
        let mut lines = all_lines.by_ref().take_while(|line| !line.is_empty());
        let seeds = lines.next().unwrap().split(":").nth(1).unwrap();
        // take 2 item from iterator until iterator is empty
        for mut chunk in &seeds.split_whitespace().filter(|s| !s.is_empty()).chunks(2) {
            let source_start = chunk.next().unwrap().parse::<u64>().unwrap();
            let length = chunk.next().unwrap().parse::<u64>().unwrap();
            seed_ranges.push(SeedRange {
                source_start,
                length,
            });
        }
        all_lines.next();

        for _ in 0..7 {
            let mut ranges = Vec::new();
            let mut lines = all_lines.by_ref().take_while(|line| !line.is_empty());
            lines.next();
            for range in lines.filter(|s| !s.is_empty()) {
                ranges.push(range.parse::<Range>().unwrap());
            }
            maps.push(Map { ranges });
        }
        Ok(AlmanacSeedRange { seed_ranges, maps })
    }
}

impl AlmanacSeedRange {
    fn calculate_for_seed(&self, seed: u64) -> u64 {
        self.maps
            .iter()
            .fold(seed, |value, map| map.mapped_value(value))
    }
    fn calculate_for_seed_range(&self, start: u64, length: u64) -> u64 {
        if length == 1 {
            return min(
                self.calculate_for_seed(start),
                self.calculate_for_seed(start + 1),
            );
        }
        let step = length / 2;

        let start_value = self.calculate_for_seed(start);
        let middle_value = self.calculate_for_seed(start + step);
        let end_value = self.calculate_for_seed(start + length);

        let mut min_value = u64::MAX;
        if start_value + step != middle_value {
            min_value = self.calculate_for_seed_range(start, step);
        }
        if middle_value + (length - step) != end_value {
            min_value = min(
                min_value,
                self.calculate_for_seed_range(start + step, length - step),
            );
        }
        min_value
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac = input.parse::<Almanac>().unwrap();
    almanac
        .seeds
        .iter()
        .map(|seed| {
            almanac
                .maps
                .iter()
                .fold(*seed, |value, map| map.mapped_value(value))
        })
        .min()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let num_threads = 4;
    let almanac = input.parse::<AlmanacSeedRange>().unwrap();
    let pool = ThreadPool::new(num_threads);
    let almanac = Arc::new(almanac);

    let (tx, rx) = channel();
    let mut work_count = 0;
    for seed_range in almanac.seed_ranges.clone() {
        work_count += 1;
        let tx = tx.clone();
        let arc_almanac = almanac.clone();
        let seed_range = seed_range.clone();
        pool.execute(move || {
            let result =
                arc_almanac.calculate_for_seed_range(seed_range.source_start, seed_range.length);
            tx.send(result).unwrap();
        });
    }

    Some(rx.iter().take(work_count).min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_from_str() {
        let input = "10 20 30";
        let expected = Range {
            source_start: 20,
            destination_start: 10,
            length: 30,
        };
        let result = input.parse::<Range>();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_range_mapped_value() {
        let range = Range {
            source_start: 20,
            destination_start: 10,
            length: 30,
        };
        assert_eq!(range.mapped_value(25), Some(15));
        assert_eq!(range.mapped_value(50), None);
    }

    #[test]
    fn test_map_mapped_value() {
        let ranges = vec![
            Range {
                source_start: 20,
                destination_start: 10,
                length: 30,
            },
            Range {
                source_start: 60,
                destination_start: 50,
                length: 30,
            },
        ];
        let map = Map { ranges };

        assert_eq!(map.mapped_value(25), 15);
        assert_eq!(map.mapped_value(65), 55);
        assert_eq!(map.mapped_value(100), 100);
    }

    #[test]
    fn test_almanac_from_str() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37

fertilizer-to-water map:
49 53 8

water-to-light map:
88 18 7

light-to-temperature map:
45 77 23

temperature-to-humidity map:
0 69 1

humidity-to-location map:
60 56 37";

        let expected = Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                Map {
                    ranges: vec![
                        Range {
                            source_start: 98,
                            destination_start: 50,
                            length: 2,
                        },
                        Range {
                            source_start: 50,
                            destination_start: 52,
                            length: 48,
                        },
                    ],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 15,
                        destination_start: 0,
                        length: 37,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 53,
                        destination_start: 49,
                        length: 8,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 18,
                        destination_start: 88,
                        length: 7,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 77,
                        destination_start: 45,
                        length: 23,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 69,
                        destination_start: 0,
                        length: 1,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 56,
                        destination_start: 60,
                        length: 37,
                    }],
                },
            ],
        };

        let result = input.parse::<Almanac>();
        println!("{:#?}", result);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_almanacseedrange_from_str() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37

fertilizer-to-water map:
49 53 8

water-to-light map:
88 18 7

light-to-temperature map:
45 77 23

temperature-to-humidity map:
0 69 1

humidity-to-location map:
60 56 37";

        let expected = AlmanacSeedRange {
            seed_ranges: vec![
                SeedRange {
                    source_start: 79,
                    length: 14,
                },
                SeedRange {
                    source_start: 55,
                    length: 13,
                },
            ],
            maps: vec![
                Map {
                    ranges: vec![
                        Range {
                            source_start: 98,
                            destination_start: 50,
                            length: 2,
                        },
                        Range {
                            source_start: 50,
                            destination_start: 52,
                            length: 48,
                        },
                    ],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 15,
                        destination_start: 0,
                        length: 37,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 53,
                        destination_start: 49,
                        length: 8,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 18,
                        destination_start: 88,
                        length: 7,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 77,
                        destination_start: 45,
                        length: 23,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 69,
                        destination_start: 0,
                        length: 1,
                    }],
                },
                Map {
                    ranges: vec![Range {
                        source_start: 56,
                        destination_start: 60,
                        length: 37,
                    }],
                },
            ],
        };

        let result = input.parse::<AlmanacSeedRange>();
        println!("{:#?}", result);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}

use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    ops::Range,
    rc::Rc,
};

fn main() {
    let almanac = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&almanac));
}

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<Rc<str>, Mapping>,
}

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    to: Rc<str>,
    range_mappings: Vec<RangeMapping>,
}

#[derive(Debug, PartialEq, Eq)]
struct RangeMapping {
    dest_start: u64,
    source_start: u64,
    size: u64,
}

impl RangeMapping {
    fn source_range(&self) -> Range<u64> {
        self.source_start..self.source_start + self.size
    }
    fn source_to_dest(&self, source_number: u64) -> Option<u64> {
        if !self.source_range().contains(&source_number) {
            return None;
        }
        let distance = source_number - self.source_start;
        Some(self.dest_start + distance)
    }
}

fn parse_input(input: impl BufRead) -> Almanac {
    let input = std::io::read_to_string(input).unwrap();
    let mut sections = input.split("\n\n");
    let seeds_section = sections.next().unwrap();
    let (_, seed_numbers) = seeds_section.split_once(':').unwrap();
    let seeds = seed_numbers
        .trim()
        .split(' ')
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let maps = sections
        .map(|section| {
            let (name, ranges) = section.split_once(':').unwrap();
            let (from, to) = parse_section_name(name);
            let range_mappings = parse_ranges(ranges);
            (from, Mapping { to, range_mappings })
        })
        .collect();
    Almanac { seeds, maps }
}

fn parse_section_name(section_name: &str) -> (Rc<str>, Rc<str>) {
    let (name_without_space_map, _) = section_name.split_once(' ').unwrap();
    let (from, to) = name_without_space_map.split_once("-to-").unwrap();
    (from.into(), to.into())
}

fn parse_ranges(ranges: &str) -> Vec<RangeMapping> {
    ranges
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split(' ').map(|num| num.parse::<u64>().unwrap());
            let dest_start = nums.next().unwrap();
            let source_start = nums.next().unwrap();
            let size = nums.next().unwrap();
            RangeMapping {
                dest_start,
                source_start,
                size,
            }
        })
        .collect()
}

fn part_1(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|seed_number| seed_to_location(almanac, *seed_number))
        .min()
        .unwrap()
}

fn seed_to_location(almanac: &Almanac, seed_number: u64) -> u64 {
    let mut current_type: &str = "seed";
    let mut current_value: u64 = seed_number;

    for _ in 0..100 {
        let mapping = almanac.maps.get(current_type).unwrap();
        current_value = mapping
            .range_mappings
            .iter()
            .find_map(|range_mapping| range_mapping.source_to_dest(current_value))
            .unwrap_or(current_value);
        current_type = mapping.to.as_ref();
        if current_type == "location" {
            return current_value;
        }
    }
    panic!("never reached location in series of maps")
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_parse_input_example() {
        let almanac = parse_input(EXAMPLE.as_bytes());
        assert_eq!(&almanac.seeds, &[79, 14, 55, 13]);

        assert_eq!(
            almanac.maps.get("seed").unwrap(),
            &Mapping {
                to: "soil".into(),
                range_mappings: vec![
                    RangeMapping {
                        dest_start: 50,
                        source_start: 98,
                        size: 2
                    },
                    RangeMapping {
                        dest_start: 52,
                        source_start: 50,
                        size: 48
                    }
                ]
            }
        )
    }

    #[test]
    fn test_part_1_example() {
        let almanac = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&almanac), 35);
    }
}

use anyhow::Result;
use itertools::*;
use rayon::prelude::*;

#[derive(Debug)]
struct ConversionMap {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

impl ConversionMap {
    fn from_str(input: &str) -> Result<Self> {
        let mut numbers = Vec::new();

        for number in input.split(" ") {
            let value = u64::from_str_radix(number, 10)?;
            numbers.push(value);
        }

        if numbers.len() != 3 {
            anyhow::bail!("Expected exactly three numbers.");
        }

        Ok(ConversionMap {
            source_start: *numbers.get(1).unwrap(),
            destination_start: *numbers.get(0).unwrap(),
            range: *numbers.get(2).unwrap(),
        })
    }
}

fn parse_seeds(input: &str) -> Vec<u64> {
    let mut seeds = Vec::new();
    for value in input.split(" ") {
        if let Ok(seed_value) = u64::from_str_radix(value, 10) {
            seeds.push(seed_value);
        }
    }
    seeds
}

fn part_two_parse_seeds_range(input: &str) -> Vec<u64> {
    let mut seeds = Vec::new();
    for value in input.split(" ") {
        if let Ok(seed_value) = u64::from_str_radix(value, 10) {
            seeds.push(seed_value);
        }
    }
    let mut final_seeds = Vec::new();
    let mut seeds = seeds.into_iter();
    while let Some((x, y)) = seeds.next_tuple() {
        for k in x..x+y {
            final_seeds.push(k);
        }
    }
    final_seeds
}

fn find_location_of_seed(seed: u64, conversion_map: &Vec<Vec<ConversionMap>>) -> u64 {
    let mut current_val = seed;
    for types in conversion_map {
        'outer: for conversion in types {
            let source_start = conversion.source_start;
            let source_end = conversion.source_start + conversion.range;
            if current_val >= source_start && current_val < source_end {
                let destination_start = conversion.destination_start;
                let destination_end = conversion.destination_start + conversion.range;
                let next = conversion.destination_start + (current_val - conversion.source_start);

                if next >= destination_start && next < destination_end {
                    current_val = next;
                    break 'outer;
                }
            }
        }
    }
    current_val
}

pub fn find_lowest_location(input: &str, part_two: bool) -> u64 {
    let mut seeds = Vec::new();
    let mut maps: Vec<Vec<ConversionMap>> = Vec::new();
    for line in input.lines() {
        match line {
            line if line.contains("seeds:") => {
                if part_two {
                    seeds = part_two_parse_seeds_range(line);
                } else {
                    seeds = parse_seeds(line);
                }
            }
            line if line.contains("map:") => {
                maps.push(Vec::new());
            }
            _ => {
                if let Some(map) = maps.last_mut() {
                    if let Ok(conversion_map) = ConversionMap::from_str(line) {
                        map.push(conversion_map);
                    }
                }
            }
        }
    }
    let location = seeds
        .par_iter()
        .map(|seed| find_location_of_seed(*seed, &maps))
        .min();
    if let Some(location) = location {
        return location;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn sample_find_lowest_location() {
        let result = find_lowest_location(SAMPLE, false);
        assert_eq!(result, 35);
    }

    #[test]
    fn sample_find_lowest_location_seed_range() {
        let result = find_lowest_location(SAMPLE, true);
        assert_eq!(result, 46);
    }

    #[test]
    fn input_find_lowest_location() {
        let result = find_lowest_location(INPUT, false);
        assert_eq!(result, 551761867);
    }

    #[test]
    fn input_find_lowest_location_seed_range() {
        let result = find_lowest_location(INPUT, true);
        assert_eq!(result, 57451709);
    }
}

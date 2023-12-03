use itertools::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, Hash, PartialEq)]
struct PartsCoordinate {
    x: usize,
    y: usize,
    y_range: (usize, usize),
    part_number: u32,
}

pub fn sum_of_engine_parts(input: &str) -> u32 {
    let mut adjacent_parts = 0;

    let mut symbol_coordinates: Vec<(usize, usize)> = Vec::new();
    let mut part_numbers: HashMap<(usize, usize), PartsCoordinate> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let mut part_number = None;
        let mut starting_part_number_coordinate = 0;
        let mut last_part_number_coordinate = 0;
        for (j, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    if let Some(number) = c.to_digit(10) {
                        if let Some(part) = part_number {
                            part_number = Some((part * 10) + number);
                            last_part_number_coordinate = j;
                        } else {
                            starting_part_number_coordinate = j;
                            last_part_number_coordinate = j;
                            part_number = Some(number);
                        }
                    }
                }
                _ => {
                    if let Some(part) = part_number {
                        for y in starting_part_number_coordinate..=last_part_number_coordinate {
                            part_numbers.insert(
                                (i, y),
                                PartsCoordinate {
                                    x: i,
                                    y,
                                    y_range: (
                                        starting_part_number_coordinate,
                                        last_part_number_coordinate,
                                    ),
                                    part_number: part,
                                },
                            );
                        }
                    }
                    part_number = None;

                    if c != '.' {
                        symbol_coordinates.push((i, j));
                    }
                }
            }
        }

        if let Some(part) = part_number {
            for y in starting_part_number_coordinate..=last_part_number_coordinate {
                part_numbers.insert(
                    (i, y),
                    PartsCoordinate {
                        x: i,
                        y,
                        y_range: (starting_part_number_coordinate, last_part_number_coordinate),
                        part_number: part,
                    },
                );
            }
        }
    }

    let mut seen_numbers: HashSet<(usize, usize)> = HashSet::new();
    for (i, j) in symbol_coordinates {
        let neighbors = ((i - 1)..=(i + 1)).cartesian_product((j - 1)..=(j + 1));

        for (x, y) in neighbors {
            if seen_numbers.contains(&(x, y)) {
                continue;
            }
            if let Some(part) = part_numbers.get(&(x, y)) {
                adjacent_parts += part.part_number;
                for l in part.y_range.0..=part.y_range.1 {
                    seen_numbers.insert((x, l));
                }
            }
        }
    }
    adjacent_parts
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = include_str!("sample.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn sample_sum_of_engine_parts() {
        let result = sum_of_engine_parts(SAMPLE);
        assert_eq!(result, 4361);
    }

    #[test]
    fn input_sum_of_engine_parts() {
        let result = sum_of_engine_parts(INPUT);
        assert_eq!(result, 525119);
    }
}

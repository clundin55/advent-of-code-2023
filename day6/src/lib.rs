fn calculate_possible_winning_races(time: u128, record: u128) -> u128 {
    let mut combinations = 0;

    for hold in 1..time {
        let remainder = time - hold;
        if hold > record / remainder {
            combinations += 1;
        }
    }
    combinations
}

pub fn boat_race(input: &str) -> u128 {
    let mut times = Vec::new();
    let mut distances = Vec::new();
    for line in input.lines() {
        match line {
            line if line.contains("Time") => {
                times = line
                    .split_whitespace()
                    .filter_map(|digit| u128::from_str_radix(digit, 10).ok())
                    .collect();
            }
            line if line.contains("Distance") => {
                distances = line
                    .split_whitespace()
                    .filter_map(|digit| u128::from_str_radix(digit, 10).ok())
                    .collect();
            }
            _ => (),
        }
    }
    let mut race_possibilities = Vec::new();
    for (time, distance) in times.into_iter().zip(distances) {
        race_possibilities.push(calculate_possible_winning_races(time, distance));
    }

    race_possibilities.iter().product()
}

pub fn boat_race_part_two(input: &str) -> u128 {
    let mut time: u128 = 0;
    let mut distance: u128 = 0;
    for line in input.lines() {
        match line {
            line if line.contains("Time") => {
                for digit in line.chars().filter(|c| c.is_numeric()) {
                    if let Some(digit) = digit.to_digit(10) {
                        let digit: u128 = digit.into();
                        if time == 0 {
                            time = digit.into();
                        } else {
                            time = (time * 10) + digit;
                        }
                    }
                }
            }
            line if line.contains("Distance") => {
                for digit in line.chars().filter(|c| c.is_numeric()) {
                    if let Some(digit) = digit.to_digit(10) {
                        let digit: u128 = digit.into();
                        if distance == 0 {
                            distance = digit.into();
                        } else {
                            distance = (distance * 10) + digit;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    calculate_possible_winning_races(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn sample_boat_race() {
        let result = boat_race(SAMPLE);
        assert_eq!(result, 288);
    }

    #[test]
    fn sample_boat_race_part_two() {
        let result = boat_race_part_two(SAMPLE);
        assert_eq!(result, 71503);
    }

    #[test]
    fn input_boat_race() {
        let result = boat_race(INPUT);
        assert_eq!(result, 1413720);
    }

    #[test]
    fn input_boat_race_part_two() {
        let result = boat_race_part_two(INPUT);
        assert_eq!(result, 30565288);
    }
}

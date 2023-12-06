fn calculate_possible_winning_races(time: u32, record: u32) -> u32 {
    let mut combinations = 0;
    for hold in 1..time {
        let remainder = time - hold;
        let distance = hold * remainder;
        if distance >record {
            combinations += 1;
        }
    }
    combinations
}

pub fn boat_race(input: &str) -> u32 {
    let mut times = Vec::new();
    let mut distances = Vec::new();
    for line in input.lines() {
        match line {
            line if line.contains("Time") => {
                times = line.split_whitespace().filter_map(|digit| u32::from_str_radix(digit, 10).ok()).collect();
            },
            line if line.contains("Distance") => {
                distances = line.split_whitespace().filter_map(|digit| u32::from_str_radix(digit, 10).ok()).collect();
            },
            _ => (),
        }
    }
    let mut race_possibilities = Vec::new();
    for (time, distance) in times.into_iter().zip(distances) {
        race_possibilities.push(calculate_possible_winning_races(time,distance));
    }

    race_possibilities.iter().product()
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
    fn input_boat_race() {
        let result = boat_race(INPUT);
        assert_eq!(result, 288);
    }
}

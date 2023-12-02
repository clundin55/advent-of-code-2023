pub fn possible_games(input: &str, red_cubes: u32, green_cubes: u32, blue_cubes: u32) -> u32 {
    let mut valid_games = 0;
    for line in input.lines() {
        let mut line = line.split(": ");
        let game_id = line.next().expect("Missing a :");
        let game_id = game_id
            .split(" ")
            .nth(1)
            .expect("Expected a space between Game and game id.");
        let game_id = u32::from_str_radix(game_id, 10).expect("Unable to parse u32");

        dbg!(&game_id);
        dbg!(&line);
        let game_sequence = line.next().expect("Game sequence starts after :");
        let mut valid_game = true;
        for set in game_sequence.split(";") {
            for color in set.split(",") {
                let mut count = 0;
                for item in color.split(" ") {
                    match item {
                        "red" => {
                            if count > red_cubes {
                                valid_game = false;
                            }
                        }
                        "green" => {
                            if count > green_cubes {
                                valid_game = false;
                            }
                        }
                        "blue" => {
                            if count > blue_cubes {
                                valid_game = false;
                            }
                        }
                        _ => {
                            if let Ok(color_count) = u32::from_str_radix(item, 10) {
                                count = color_count;
                            }
                        }
                    }
                }
            }
        }
        dbg!(valid_game);
        if valid_game {
            valid_games += game_id;
        }
    }

    valid_games
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = include_str!("sample.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn sample_possible_games() {
        let result = possible_games(SAMPLE, 12, 13, 14);
        assert_eq!(result, 8);
    }

    #[test]
    fn input_possible_games() {
        let result = possible_games(INPUT, 12, 13, 14);
        assert_eq!(result, 2600);
    }
}

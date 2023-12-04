use std::collections::HashSet;

pub fn scratch_card_point(input: &str) -> u32 {
    let mut points = 0;
    for line in input.lines() {
        let mut midpoint = line.split(" | ");
        let mut winner_cards: HashSet<u32> = HashSet::new();
        if let Some(winner_card) = midpoint.next() {
            if let Some(cards) = winner_card.split(": ").nth(1) {
                let cards = cards
                    .split(" ")
                    .filter_map(|card| u32::from_str_radix(card, 10).ok());
                winner_cards = cards.collect();
            }
        }

        if let Some(cards) = midpoint.next() {
            let mut winners = 0;
            for card in cards.split(" ") {
                if let Ok(card_value) = u32::from_str_radix(card, 10) {
                    if winner_cards.contains(&card_value) {
                        winners += 1;
                    }
                }
            }
            if winners > 0 {
                points += 2_u32.pow(winners - 1)
            }
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = include_str!("sample.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn sample_scratch_card_point() {
        let result = scratch_card_point(SAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn input_scratch_card_point() {
        let result = scratch_card_point(INPUT);
        assert_eq!(result, 13);
    }
}

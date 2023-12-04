use std::collections::{HashMap, HashSet};

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

#[derive(Clone, Debug)]
struct ScratchCard {
    index: usize,
    cards: Vec<u32>,
}

pub fn scratch_card_duplication(input: &str) -> u32 {
    let mut all_winner_cards: HashMap<usize, HashSet<u32>> = HashMap::new();
    let mut original_cards: Vec<ScratchCard> = Vec::new();

    for (idx, line) in input.lines().enumerate() {
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
        all_winner_cards.insert(idx, winner_cards);

        if let Some(cards) = midpoint.next() {
            let parsed_cards: Vec<u32> = cards
                .split(" ")
                .filter_map(|card| u32::from_str_radix(card, 10).ok())
                .collect();
            original_cards.push(ScratchCard {
                index: idx,
                cards: parsed_cards,
            });
        }
    }

    let original_cards_duplicate = original_cards.clone();

    let mut total_cards = 0;
    while let Some(card) = original_cards.pop() {
        total_cards += 1;
        let mut winners = 0;

        if let Some(winning_cards) = all_winner_cards.get(&card.index) {
            for c in card.cards {
                if winning_cards.contains(&c) {
                    winners += 1;
                }
            }
        }
        for winner in 1..=winners {
            let duplicate_card_idx = card.index + winner;
            if let Some(card_clone) = original_cards_duplicate.get(duplicate_card_idx) {
                original_cards.push(card_clone.clone());
            }
        }
    }
    total_cards
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
    fn sample_scratch_card_duplication() {
        let result = scratch_card_duplication(SAMPLE);
        assert_eq!(result, 30);
    }

    #[test]
    fn input_scratch_card_point() {
        let result = scratch_card_point(INPUT);
        assert_eq!(result, 18653);
    }

    #[test]
    fn input_scratch_card_duplication() {
        let result = scratch_card_duplication(INPUT);
        assert_eq!(result, 5921508);
    }
}

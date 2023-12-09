use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    card: String,
    kind: HandType,
    bet: u32,
}

impl Hand {
    fn new(card: String, kind: HandType, bet: u32) -> Self {
        Self{ card, kind, bet}
    }
}

#[derive(Debug, PartialEq, Eq,  PartialOrd)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

fn convert_card_to_value(card: &char) -> u32 {
    match card {
        'A' => 15,
        'K' => 14,
        'Q' => 13,
        'J' => 12,
        'T' => 11,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Unexpected value!"),
    }
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind > other.kind {
            return Some(std::cmp::Ordering::Greater);
        }
        if self.kind < other.kind {
            return Some(std::cmp::Ordering::Less);
        }

        for (c1, c2) in self.card.chars().zip(other.card.chars()) {
            if c1 == c2 {
                continue;
            }

            let v1 = convert_card_to_value(&c1);
            let v2 = convert_card_to_value(&c2);

            if v1 > v2 {
                return Some(std::cmp::Ordering::Greater);
            }
            if v1 < v2 {
                return Some(std::cmp::Ordering::Less);
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind > other.kind {
            return std::cmp::Ordering::Greater;
        }
        if self.kind < other.kind {
            return std::cmp::Ordering::Less;
        }
        for (c1, c2) in self.card.chars().zip(other.card.chars()) {
            if c1 == c2 {
                continue;
            }

            let v1 = convert_card_to_value(&c1);
            let v2 = convert_card_to_value(&c2);

            if v1 > v2 {
                return std::cmp::Ordering::Greater;
            }
            if v1 < v2 {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal
    }
}

fn hand_type(hand: &HashMap<char, u32>) -> HandType {
    let mut seen_two = false;
    let mut seen_three = false;

    for value in hand.values() {
        match value {
            5 => return HandType::FiveOfAKind,
            4 => return HandType::FourOfAKind,
            2 if seen_two => return HandType::TwoPair,
            2 => seen_two = true,
            3 => seen_three = true,
            _ => (),
        };
    }

    if seen_two && seen_three {
        return HandType::FullHouse;
    }

    if seen_three {
        return HandType::ThreeOfAKind;
    }

    if seen_two {
        return HandType::OnePair;
    }

    HandType::HighCard
}

pub fn total_winnings(input: &str) -> u32 {
    let mut  hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let cards = split.next().unwrap();
        let points = split.next().unwrap();
        let points = u32::from_str_radix(points, 10).unwrap();

        let mut hand = HashMap::new();
        for c in cards.chars() {
            let val = hand.entry(c).or_insert(0);
            *val += 1;
        }
        let kind = hand_type(&hand);
        hands.push(Hand::new(cards.to_owned(), kind, points));
    }
    hands.sort();

    let mut winnings = 0;
    let mut multiplier = 1;

    for hand in hands {
        winnings += hand.bet * multiplier;
        multiplier += 1;
    }

    winnings
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn sample_total_winnings() {
        let result = total_winnings(SAMPLE);
        assert_eq!(result, 6440);
    }

    #[test]
    fn input_total_winnings() {
        let result = total_winnings(INPUT);
        assert_eq!(result, 251058093);
    }
}

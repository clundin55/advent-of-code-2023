pub fn trebuchet_launch(calibrations: &str) -> u32 {
    let mut calculations = Vec::new();
    for line in calibrations.lines() {
        let digits: Vec<u32> = line
            .chars()
            .filter(|c| c.is_numeric())
            .filter_map(|c| c.to_digit(10))
            .collect();

        let first_digit = digits.first().expect("Expected a u32");
        let second_digit = digits.last().expect("Expected a u32");

        let calculation = (*first_digit * 10) + second_digit;
        calculations.push(calculation);
    }
    calculations.iter().sum()
}

pub fn trebuchet_launch_with_words(calibrations: &str) -> u32 {
    // Too lazy to write a Trie or graph solution.
    let mut calculations = Vec::new();
    let mut digit_strings = std::collections::HashMap::new();
    digit_strings.insert("one".to_owned(), 1);
    digit_strings.insert("two".to_owned(), 2);
    digit_strings.insert("three".to_owned(), 3);
    digit_strings.insert("four".to_owned(), 4);
    digit_strings.insert("five".to_owned(), 5);
    digit_strings.insert("six".to_owned(), 6);
    digit_strings.insert("seven".to_owned(), 7);
    digit_strings.insert("eight".to_owned(), 8);
    digit_strings.insert("nine".to_owned(), 9);

    for line in calibrations.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        let mut first_word = String::new();
        let mut last_word = String::new();

        for (first_char, last_char) in line.chars().zip(line.chars().rev()) {
            match first_char {
                'w' | 'e' | 's' | 'h' | 'o' | 'g' | 'n' | 't' | 'f' | 'u' | 'r' | 'v' | 'x'
                | 'i' => first_word.push(first_char),
                '1'..='9' => {
                    if first_digit.is_none() {
                        first_digit = first_char.to_digit(10)
                    }
                }
                _ => (),
            }

            match last_char {
                'w' | 'e' | 's' | 'h' | 'o' | 'g' | 'n' | 't' | 'f' | 'u' | 'r' | 'v' | 'x'
                | 'i' => last_word.insert(0, last_char),
                '1'..='9' => {
                    if last_digit.is_none() {
                        last_digit = last_char.to_digit(10)
                    }
                }
                _ => (),
            }

            // Brute force check. If I used a Trie I could just traverse the graph based on the
            // seen characters.
            for (k, v) in &digit_strings {
                if first_word.contains(k) && first_digit.is_none() {
                    first_digit = Some(*v);
                }
                if last_word.contains(k) && last_digit.is_none() {
                    last_digit = Some(*v);
                }
            }

            if first_digit.is_some() && last_digit.is_some() {
                break;
            }
        }

        let first_digit = first_digit.expect("Expected a valid digit.");
        let last_digit = last_digit.expect("Expected a valid digit.");

        let calculation = (first_digit * 10) + last_digit;
        calculations.push(calculation);
    }
    calculations.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("sample.txt");
    const SAMPLE_INPUT_WITH_WORDS: &str = include_str!("sample_with_words.txt");
    const INPUT: &str = include_str!("input.txt");
    const INPUT_WITH_WORDS: &str = include_str!("input_with_words.txt");

    #[test]
    fn check_sample() {
        let result = trebuchet_launch(SAMPLE_INPUT);
        assert_eq!(result, 142);
    }

    #[test]
    fn check_input() {
        let result = trebuchet_launch(INPUT);
        assert_eq!(result, 55621);
    }

    #[test]
    fn check_sample_with_words() {
        let result = trebuchet_launch_with_words(SAMPLE_INPUT_WITH_WORDS);
        assert_eq!(result, 281);
    }

    #[test]
    fn check_input_with_words() {
        let result = trebuchet_launch_with_words(INPUT_WITH_WORDS);
        assert_eq!(result, 53592);
    }
}

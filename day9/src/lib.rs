fn predict_next(values: &Vec<Vec<i32>>) -> i32 {
    let mut prev_last = 0;
    for value in values {
        if let Some(last) = value.iter().last() {
            let prediction = prev_last + last;
            prev_last = prediction;
        }
    }
    prev_last
}

fn predict_next_part_two(values: &Vec<Vec<i32>>) -> i32 {
    let mut prev_first = 0;
    for value in values.iter().rev() {
        if let Some(first) = value.iter().next() {
            let prediction = first - prev_first;
            prev_first = prediction;
        }
    }
    prev_first
}

pub fn extrapolate_history(input: &str) -> i32 {
    let mut predictions = Vec::new();
    for line in input.lines() {
        let integers = line
            .split_whitespace()
            .filter_map(|c| i32::from_str_radix(c, 10).ok());

        let mut differences: Vec<Vec<i32>> = Vec::new();
        differences.push(integers.collect());

        loop {
            let mut difference: Vec<i32> = Vec::new();
            if let Some(head) = differences.last() {
                let mut head = head.into_iter().peekable();
                while let Some(curr) = head.next() {
                    if let Some(&next) = head.peek() {
                        difference.push(next - curr);
                    }
                }
            }
            let sum = difference.iter().sum::<i32>();
            differences.push(difference);
            if sum == 0 {
                let prediction = predict_next(&mut differences);
                predictions.push(prediction);
                break;
            }
        }
    }
    predictions.iter().sum::<i32>()
}

pub fn extrapolate_history_part_two(input: &str) -> i32 {
    let mut predictions = Vec::new();
    for line in input.lines() {
        let integers = line
            .split_whitespace()
            .filter_map(|c| i32::from_str_radix(c, 10).ok());

        let mut differences: Vec<Vec<i32>> = Vec::new();
        differences.push(integers.collect());

        loop {
            let mut difference: Vec<i32> = Vec::new();
            if let Some(head) = differences.last() {
                let mut head = head.into_iter().peekable();
                while let Some(curr) = head.next() {
                    if let Some(&next) = head.peek() {
                        difference.push(next - curr);
                    }
                }
            }
            let sum = difference.iter().sum::<i32>();
            differences.push(difference);
            if sum == 0 {
                let prediction = predict_next_part_two(&mut differences);
                predictions.push(prediction);
                break;
            }
        }
    }
    predictions.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn sample_extrapolate_history() {
        let result = extrapolate_history(SAMPLE);
        assert_eq!(result, 114);
    }

    #[test]
    fn sample_extrapolate_history_part_two() {
        let result = extrapolate_history_part_two(SAMPLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn input_extrapolate_history() {
        let result = extrapolate_history(INPUT);
        assert_eq!(result, 1684566095);
    }

    #[test]
    fn input_extrapolate_history_part_two() {
        let result = extrapolate_history_part_two(INPUT);
        assert_eq!(result, 1684566095);
    }
}

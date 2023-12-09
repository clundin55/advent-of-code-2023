fn predict_next(values: &mut Vec<Vec<i32>>) -> i32 {
    let mut prev_last = 0;
    for value in values {
        if let Some(last) = value.iter().last() {
            let prediction = prev_last + last;
            value.push(prediction);
            prev_last = prediction;
        }
    }
    prev_last
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
    fn input_extrapolate_history() {
        let result = extrapolate_history(INPUT);
        assert_eq!(result, 114);
    }
}

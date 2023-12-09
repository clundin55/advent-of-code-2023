use std::collections::HashMap;

pub fn desert_map(input: &str) -> u32 {
    let mut traversal: HashMap<String, (String, String)> = HashMap::new();

    let mut lines = input.lines();
    let directions = lines.next();
    let mut directions = directions.expect("Directions are missing").chars().into_iter().cycle();

    for line in lines {
        if line.contains("=") {
            let mut split = line.split(" = ");
            let current_node = split.next().unwrap().to_owned();
            let next_nodes = split.next().unwrap();
            let mut split = next_nodes.split(", ");

            let left = split.next().unwrap();
            let right = split.next().unwrap();

            let left = left.replace("(", "");
            let right = right.replace(")", "");
            traversal.insert(current_node, (left, right));
        }
    }
    let mut current_node = "AAA";
    let mut steps = 0;
    while current_node != "ZZZ" {
        if let Some(next_nodes) = traversal.get(current_node) {
            if let Some(direction) = directions.next() {
                match direction {
                    'L' => {
                        current_node = &next_nodes.0;
                    }
                    'R' => {
                        current_node = &next_nodes.1;
                    },
                    _ => panic!("Unexpected direction!"),
                }

            }
        }
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample.txt");
    const SAMPLE2: &str = include_str!("sample2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn sample_desert_map() {
        let result = desert_map(SAMPLE);
        assert_eq!(result, 2);
        
        let result = desert_map(SAMPLE2);
        assert_eq!(result, 6);
    }

    #[test]
    fn input_desert_map() {
        let result = desert_map(INPUT);
        assert_eq!(result, 2);
    }
}

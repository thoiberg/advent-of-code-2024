use std::{collections::HashMap, iter::zip};

fn main() {
    let puzzle_input = include_str!("../data/puzzle_input.txt");
    let data = process_data(puzzle_input);

    let part_one_answer = part_one_solution(&data);
    println!("The answer to Part One is: {part_one_answer}");

    let part_two_answer = part_two_solution(&data);
    println!("The answer to Part Two is: {part_two_answer}")
}

type TestData = (Vec<u32>, Vec<u32>);

fn process_data(input: &str) -> TestData {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    input.lines().for_each(|line| {
        let parts: Vec<_> = line
            .split_whitespace()
            .map(|part| part.parse::<u32>().unwrap())
            .collect();

        left.push(parts[0]);
        right.push(parts[1]);
    });

    (left, right)
}

fn part_one_solution((left, right): &TestData) -> u32 {
    let mut sorted_left = left.clone();
    let mut sorted_right = right.clone();
    sorted_left.sort_unstable();
    sorted_right.sort_unstable();

    zip(sorted_left, sorted_right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part_two_solution((left, right): &TestData) -> u32 {
    let mut right_tally: HashMap<u32, u32> = HashMap::new();
    for i in right {
        if let Some(count) = right_tally.get_mut(i) {
            *count += 1;
        } else {
            right_tally.insert(*i, 1);
        }
    }

    left.iter()
        .map(|x| {
            let count = right_tally.get(x).unwrap_or(&0);

            count * x
        })
        .sum()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn example_data() -> TestData {
        process_data(include_str!("../data/test_input.txt"))
    }

    fn puzzle_data() -> TestData {
        process_data(include_str!("../data/puzzle_input.txt"))
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(&example_data()), 11);
    }

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(&puzzle_data()), 1388114);
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two_solution(&example_data()), 31);
    }

    #[test]
    fn test_part_two_solution() {
        assert_eq!(part_two_solution(&puzzle_data()), 23529853);
    }
}

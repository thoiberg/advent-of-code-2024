fn main() {
    let data = process_input(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&data);
    println!("The Part One answer is {part_one_answer}");
}

type Calibration = (u64, Vec<u64>);

fn part_one_solution(data: &[Calibration]) -> u64 {
    data.iter()
        .filter_map(|(expected_total, values)| {
            let all_totals = calculate(values);

            let matching_totals: Vec<_> = all_totals
                .into_iter()
                .filter(|total| total == expected_total)
                .collect();

            if matching_totals.is_empty() {
                None
            } else {
                Some(expected_total)
            }
        })
        .sum()
}

fn process_input(input: &str) -> Vec<Calibration> {
    input
        .lines()
        .map(|line| {
            let (total, values) = line.split_once(':').unwrap();

            (
                total.parse::<u64>().unwrap(),
                values
                    .trim()
                    .split_ascii_whitespace()
                    .map(|val| val.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn calculate(values: &[u64]) -> Vec<u64> {
    vec![
        run_through(values[0], Operation::Add, &values[1..]),
        run_through(values[0], Operation::Multiply, &values[1..]),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn run_through(total: u64, operation: Operation, remaining: &[u64]) -> Vec<u64> {
    let next_number = remaining[0];

    let new_total = match operation {
        Operation::Add => total + next_number,
        Operation::Multiply => total * next_number,
    };

    if remaining.len() > 1 {
        vec![
            run_through(new_total, Operation::Add, &remaining[1..]),
            run_through(new_total, Operation::Multiply, &remaining[1..]),
        ]
        .into_iter()
        .flatten()
        .collect()
    } else {
        vec![new_total]
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let data = process_input(include_str!("../data/test_input.txt"));

        assert_eq!(part_one_solution(&data), 3749);
    }

    #[test]
    fn test_part_one_solution() {
        let data = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&data), 5_837_374_519_342);
    }

    #[test]
    fn test_calculate_returns_correct_totals() {
        assert_eq!(calculate(&[10, 19]), [29, 190]);
    }
}

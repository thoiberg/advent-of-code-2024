fn main() {
    let data = process_input(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&data);
    println!("The Part One answer is {part_one_answer}");
}

type Calibration = (u64, Vec<u64>);

fn part_one_solution(data: &[Calibration]) -> u64 {
    data.iter()
        .filter_map(|(expected_total, values)| {
            let totals = calculate(*expected_total, values);

            if totals.iter().any(|total| total == expected_total) {
                Some(expected_total)
            } else {
                None
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

fn calculate(limit: u64, values: &[u64]) -> Vec<u64> {
    [
        run_through(limit, values[0], Operation::Add, &values[1..]),
        run_through(limit, values[0], Operation::Multiply, &values[1..]),
    ]
    .concat()
}

fn run_through(limit: u64, total: u64, operation: Operation, remaining: &[u64]) -> Vec<u64> {
    let next_number = remaining[0];

    let new_total = match operation {
        Operation::Add => total + next_number,
        Operation::Multiply => total * next_number,
    };

    if new_total > limit {
        return vec![];
    }

    if remaining.len() > 1 {
        [
            run_through(limit, new_total, Operation::Add, &remaining[1..]),
            run_through(limit, new_total, Operation::Multiply, &remaining[1..]),
        ]
        .concat()
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
        // assert_eq!(calculate(&[10, 19]), [29, 190]);
    }
}

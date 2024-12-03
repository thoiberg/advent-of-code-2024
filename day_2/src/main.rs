fn main() {
    let data = process_data(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&data);
    println!("The Part One answer is {part_one_answer}");
}

fn process_data(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|char| char.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn part_one_solution(data: &[Vec<i32>]) -> u32 {
    data.iter()
        .map(|report| if report_is_safe(report) { 1 } else { 0 })
        .sum()
}

fn report_is_safe(report: &[i32]) -> bool {
    let mut direction = 0;
    let mut safe = true;
    for pair in report.windows(2) {
        let a = pair[0];
        let b = pair[1];

        if (a - b) > -4 && (a - b) < 0 {
            match direction {
                0 => direction = -1,
                (1..) => {
                    safe = false;
                    break;
                }
                _ => (),
            }
        } else if (a - b) < 4 && (a - b) > 0 {
            match direction {
                0 => direction = 1,
                (..0) => {
                    safe = false;
                    break;
                }
                _ => (),
            }
        } else {
            safe = false;
        }
    }

    safe
}
#[cfg(test)]
mod test_super {
    use super::*;

    fn part_one_example_data() -> Vec<Vec<i32>> {
        process_data(include_str!("../data/test_input.txt"))
    }

    fn part_one_puzzle_data() -> Vec<Vec<i32>> {
        process_data(include_str!("../data/puzzle_input.txt"))
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(&part_one_example_data()), 2);
    }

    #[test]
    fn test_report_is_safe() {
        assert!(report_is_safe(&[7, 6, 4, 2, 1]));
        assert!(!report_is_safe(&[1, 2, 7, 8, 9]));
        assert!(!report_is_safe(&[9, 7, 6, 2, 1]));
        assert!(!report_is_safe(&[1, 3, 2, 4, 5]));
        assert!(!report_is_safe(&[8, 6, 4, 4, 1]));
        assert!(report_is_safe(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_part_one_answer() {
        assert_eq!(part_one_solution(&part_one_puzzle_data()), 299);
    }
}

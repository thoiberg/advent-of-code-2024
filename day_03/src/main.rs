use regex::Regex;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");

    let part_one_answer = part_one_solution(input);
    println!("Part One Answer is: {part_one_answer}");

    let part_two_answer = part_two_solution(input);
    println!("Part Two Answer is: {part_two_answer}");
}

fn part_one_solution(input: &str) -> u32 {
    let instructions = part_one_process_data(input);

    instructions.iter().map(|(a, b)| a * b).sum()
}

fn part_two_solution(input: &str) -> u32 {
    let instructions = part_two_process_data(input);

    instructions.iter().map(|(a, b)| a * b).sum()
}

fn part_two_process_data(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(don't)|(do)").unwrap();
    let mut process_instructions = true;

    re.captures_iter(input)
        .filter_map(|caps| {
            // safe as idx 0 is guaranteed to return a non-None value
            let match_string = caps.get(0).unwrap().as_str();

            if match_string == "do" {
                process_instructions = true;
            } else if match_string == "don't" {
                process_instructions = false;
            }

            if match_string.starts_with("mul") && process_instructions {
                let a = caps
                    .get(1)
                    .and_then(|a| a.as_str().parse::<u32>().ok())
                    .unwrap();
                let b = caps
                    .get(2)
                    .and_then(|b| b.as_str().parse::<u32>().ok())
                    .unwrap();

                return Some((a, b));
            }

            None
        })
        .collect()
}

fn part_one_process_data(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn part_one_example_data() -> String {
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    }

    fn part_two_example_data() -> String {
        String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
    }

    #[test]
    fn test_process_data_finds_all_instances() {
        let matches = part_one_process_data(&part_one_example_data());

        assert_eq!(matches.len(), 4);
        assert_eq!(matches[0], (2, 4));
        assert_eq!(matches[1], (5, 5));
        assert_eq!(matches[2], (11, 8));
        assert_eq!(matches[3], (8, 5));
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_solution(&part_one_example_data()), 161);
    }

    #[test]
    fn test_part_one_answer() {
        assert_eq!(
            part_one_solution(include_str!("../data/puzzle_input.txt")),
            190_604_937
        );
    }

    #[test]
    fn test_part_two_process_data_only_finds_enabled_instructions() {
        let matches = part_two_process_data(&part_two_example_data());

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0], (2, 4));
        assert_eq!(matches[1], (8, 5));
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two_solution(&part_two_example_data()), 48);
    }

    #[test]
    fn test_part_two_answer() {
        assert_eq!(
            part_two_solution(include_str!("../data/puzzle_input.txt")),
            82_857_512
        );
    }
}

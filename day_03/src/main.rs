use regex::Regex;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let data = process_data(input);

    let part_one_answer = part_one_solution(&data);
    println!("Part One Answer is: {part_one_answer}");
}

fn part_one_solution(data: &[(u32, u32)]) -> u32 {
    data.iter().map(|(a, b)| a * b).sum()
}

fn process_data(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    input
        .lines()
        .flat_map(|line| {
            re.captures_iter(line)
                .map(|caps| {
                    let (_, [a, b]) = caps.extract();
                    (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn part_one_example_data() -> String {
        String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
    }

    #[test]
    fn test_process_data_finds_all_instances() {
        let matches = process_data(&part_one_example_data());

        assert_eq!(matches.len(), 4);
        assert_eq!(matches[0], (2, 4));
        assert_eq!(matches[1], (5, 5));
        assert_eq!(matches[2], (11, 8));
        assert_eq!(matches[3], (8, 5));
    }

    #[test]
    fn test_part_one_example() {
        let matches = process_data(&part_one_example_data());
        assert_eq!(part_one_solution(&matches), 161);
    }

    #[test]
    fn test_part_one_answer() {
        let matches = process_data(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&matches), 190_604_937);
    }
}

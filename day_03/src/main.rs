use regex::Regex;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let data = process_data(input);

    let part_one_answer = part_one_solution(&data);
    println!("Part One Answer is: {part_one_answer}");
}

fn part_one_solution(data: &[String]) -> u32 {
    data.iter()
        .map(|instruction| {
            let chars: Vec<char> = instruction.chars().collect();
            let num_string: String = (chars[4..chars.len() - 1]).iter().cloned().collect();
            let vals: Vec<_> = num_string
                .split(',')
                .map(|f| f.parse::<u32>().unwrap())
                .collect();

            vals[0] * vals[1]
        })
        .sum()
}

fn process_data(input: &str) -> Vec<String> {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
    input
        .lines()
        .flat_map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str().to_owned())
                .collect::<Vec<String>>()
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_process_data_finds_all_instances() {
        let test_data = include_str!("../data/test_input.txt");
        let matches = process_data(test_data);

        assert_eq!(matches.len(), 4);
        assert_eq!(matches[0], "mul(2,4)");
        assert_eq!(matches[1], "mul(5,5)");
        assert_eq!(matches[2], "mul(11,8)");
        assert_eq!(matches[3], "mul(8,5)");
    }

    #[test]
    fn test_part_one_example() {
        let matches = process_data(include_str!("../data/test_input.txt"));

        assert_eq!(part_one_solution(&matches), 161);
    }

    #[test]
    fn test_part_one_answer() {
        let matches = process_data(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&matches), 190_604_937);
    }
}

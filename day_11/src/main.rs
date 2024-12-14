use std::collections::HashMap;

fn main() {
    let stones = process_input("3279 998884 1832781 517 8 18864 28 0");

    let part_one_answer = part_one_solution(&stones);
    println!("Part One Answer is {part_one_answer}");

    let part_two_answer = part_two_solution(&stones);
    println!("Part Two Answer is {part_two_answer}");
}

fn part_one_solution(stones: &[u64]) -> u64 {
    change_stones(stones, 25)
}

fn part_two_solution(stones: &[u64]) -> u64 {
    change_stones(stones, 75)
}

fn change_stones(stones: &[u64], times: u32) -> u64 {
    let mut stone_count: HashMap<u64, u64> = HashMap::new();
    for stone in stones {
        if let Some(stone_count) = stone_count.get_mut(stone) {
            *stone_count += 1;
        } else {
            stone_count.insert(*stone, 1);
        }
    }

    for _ in 0..times {
        let mut new_count: HashMap<u64, u64> = HashMap::new();

        for (stone, count) in stone_count {
            let new_stones = blink(stone);

            for new_stone in new_stones {
                if let Some(blink_count) = new_count.get_mut(&new_stone) {
                    *blink_count += count;
                } else {
                    new_count.insert(new_stone, count);
                }
            }
        }

        stone_count = new_count;
    }

    stone_count.values().sum()
}

fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().len() % 2 == 0 {
        let stone_str = stone.to_string();
        let (first, last) = stone_str.split_at(stone_str.len() / 2);

        vec![first.parse::<u64>().unwrap(), last.parse::<u64>().unwrap()]
    } else {
        vec![stone * 2024]
    }
}

fn process_input(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn example_data() -> Vec<u64> {
        process_input("125 17")
    }

    fn puzzle_data() -> Vec<u64> {
        process_input("3279 998884 1832781 517 8 18864 28 0")
    }

    #[test]
    fn test_part_one_example() {
        let data = example_data();

        assert_eq!(part_one_solution(&data), 55312);
    }

    #[test]
    fn test_blink() {
        assert_eq!(blink(0), vec![1]);
        assert_eq!(blink(1), vec![2024]);
        assert_eq!(blink(2024), vec![20, 24]);
        assert_eq!(blink(1000), vec![10, 0]);
    }

    #[test]
    fn test_part_one_answer() {
        assert_eq!(part_one_solution(&puzzle_data()), 218956);
    }

    #[test]
    fn test_part_two_answer() {
        assert_eq!(part_two_solution(&puzzle_data()), 259_593_838_049_805);
    }
}

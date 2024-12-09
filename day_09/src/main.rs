fn main() {
    let disk_map = process_input(include_str!("../src/data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(disk_map);
    println!("Part One answer is {part_one_answer}");
}

fn part_one_solution(mut disk_map: Vec<i32>) -> usize {
    let mut decrementor = disk_map.len() - 1;

    for idx in 0..disk_map.len() {
        if disk_map[idx] >= 0 {
            continue;
        }

        if decrementor <= idx {
            break;
        }

        while disk_map[decrementor] < 0 {
            decrementor -= 1;
        }

        disk_map.swap(idx, decrementor);
    }

    disk_map
        .iter()
        .enumerate()
        .filter(|(_, val)| val > &&0)
        .map(|(idx, val)| (*val as usize) * idx)
        .sum()
}

fn process_input(input: &str) -> Vec<i32> {
    let mut disk_map: Vec<i32> = vec![];

    let mut file_idx = 0;
    for (idx, data) in input.chars().enumerate() {
        if idx % 2 == 0 {
            for _ in 0..data.to_digit(10).unwrap() {
                disk_map.push(file_idx);
            }
            file_idx += 1;
        } else {
            for _ in 0..data.to_digit(10).unwrap() {
                disk_map.push(-1);
            }
        }
    }

    disk_map
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn example_data() -> String {
        "2333133121414131402".to_owned()
    }

    #[test]
    fn test_process_input_expands_disk_map() {
        let data = process_input(&example_data());

        let expected = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];

        assert_eq!(data.len(), 42);
        assert_eq!(data, expected);
    }

    #[test]
    fn test_part_one_example() {
        let data = process_input(&example_data());

        assert_eq!(part_one_solution(data), 1928);
    }

    #[test]
    fn test_part_one_solution() {
        let disk_map = process_input(include_str!("../src/data/puzzle_input.txt"));

        assert_eq!(part_one_solution(disk_map), 6_471_961_544_878);
    }
}

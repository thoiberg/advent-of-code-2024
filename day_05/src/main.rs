use std::collections::{HashMap, HashSet};

fn main() {
    let (ordering_rules, updates) = process_input(include_str!("../data/puzzle_input.txt"));

    let (ordered, unordered) = partition_ordered(&ordering_rules, updates);

    let part_one_answer = part_one_solution(&ordered);
    println!("The Part One answer is {part_one_answer}");

    let part_two_answer = part_two_solution(&ordering_rules, &unordered);
    println!("The Part Two answer is {part_two_answer}");
}

type PrintQueue = (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>);

fn part_one_solution(ordered_updates: &[Vec<u32>]) -> u32 {
    ordered_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_two_solution(
    ordering_rules: &HashMap<u32, Vec<u32>>,
    unordered_updates: &[Vec<u32>],
) -> u32 {
    unordered_updates
        .iter()
        .map(|update| {
            let mut ordered: Vec<u32> = vec![];

            update.iter().for_each(|page| {
                let page_deps = ordering_rules.get(page);

                if ordered.is_empty() || page_deps.is_none() {
                    ordered.push(*page);
                } else {
                    for i in 0..ordered.len() {
                        if page_deps.unwrap().contains(&ordered[i]) {
                            ordered.insert(i, *page);
                            break;
                        }

                        if i == (ordered.len() - 1) {
                            ordered.push(*page);
                        }
                    }
                }
            });

            ordered[ordered.len() / 2]
        })
        .sum()
}

fn partition_ordered(
    ordering_rules: &HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    updates.into_iter().partition(|update| {
        let mut expected_pages: HashSet<u32> = HashSet::new();

        update.iter().for_each(|page| {
            expected_pages.remove(page);

            if let Some(dependencies) = ordering_rules.get(page) {
                dependencies.iter().for_each(|dep| {
                    // TODO: look at better way to check
                    if update.contains(dep) {
                        let _ = expected_pages.insert(*dep);
                    }
                });
            }
        });

        expected_pages.is_empty()
    })
}

fn process_input(input: &str) -> PrintQueue {
    let (ordering_rules, update_pages) = input.split_once("\n\n").unwrap();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule in ordering_rules.lines() {
        let (page_number_str, dependency_str) = rule.split_once("|").unwrap();
        let page_number = page_number_str.parse::<u32>().unwrap();
        let dependency = dependency_str.parse::<u32>().unwrap();

        if let Some(page_number) = rules.get_mut(&page_number) {
            page_number.push(dependency);
        } else {
            rules.insert(page_number, vec![dependency]);
        }
    }

    let updates = update_pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|update| update.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> PrintQueue {
        process_input(include_str!("../data/test_input.txt"))
    }

    #[test]
    fn test_process_input() {
        let (rules, updates) = test_data();

        assert_eq!(rules.len(), 6);
        assert_eq!(rules.get(&97).unwrap(), &vec![13, 61, 47, 29, 53, 75]);

        assert_eq!(updates.len(), 6);
        assert_eq!(updates[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_part_one_example() {
        let test_data = test_data();
        let (ordered, _) = partition_ordered(&test_data.0, test_data.1);

        assert_eq!(part_one_solution(&ordered), 143);
    }

    #[test]
    fn test_part_one_solution() {
        let puzzle_data = process_input(include_str!("../data/puzzle_input.txt"));
        let (ordered, _) = partition_ordered(&puzzle_data.0, puzzle_data.1);

        assert_eq!(part_one_solution(&ordered), 5713);
    }

    #[test]
    fn test_part_two_example() {
        let test_data = test_data();
        let (_, unordered) = partition_ordered(&test_data.0, test_data.1);

        assert_eq!(part_two_solution(&test_data.0, &unordered), 123);
    }

    #[test]
    fn test_part_two_answer() {
        let puzzle_data = process_input(include_str!("../data/puzzle_input.txt"));
        let (_, unordered) = partition_ordered(&puzzle_data.0, puzzle_data.1);

        assert_eq!(part_two_solution(&puzzle_data.0, &unordered), 5180);
    }
}

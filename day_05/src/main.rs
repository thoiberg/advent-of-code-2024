use std::collections::{HashMap, HashSet};

fn main() {
    let data = process_input(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&data);
    println!("The Part One answer is {part_one_answer}");
}

type PrintQueue = (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>);

fn part_one_solution((ordering_rules, updates): &PrintQueue) -> u32 {
    updates
        .iter()
        .map(|update| {
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

            if expected_pages.is_empty() {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum()
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

        assert_eq!(part_one_solution(&test_data), 143);
    }

    #[test]
    fn test_part_one_solution() {
        let data = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&data), 5713);
    }
}

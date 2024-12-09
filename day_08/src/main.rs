use std::collections::{HashMap, HashSet};

use ndarray::Array2;

fn main() {
    let grid = process_input(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&grid);
    println!("The Part One answer is: {part_one_answer}");
}

type Map = Array2<char>;

fn part_one_solution(grid: &Map) -> usize {
    let mut antenna_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    grid.indexed_iter()
        .filter(|(_, val)| val != &&'.')
        .for_each(|(coords, char)| {
            if let Some(positions) = antenna_positions.get_mut(char) {
                positions.push(coords);
            } else {
                antenna_positions.insert(*char, vec![coords]);
            }
        });

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for positions in antenna_positions.values() {
        for (y, x) in positions {
            for (other_y, other_x) in positions {
                if (y, x) == (other_y, other_x) {
                    continue;
                }

                let y_diff = (*y as i32) - (*other_y as i32);
                let x_diff = (*x as i32) - (*other_x as i32);

                let new_y = if y_diff.is_positive() {
                    y.checked_add((y_diff).unsigned_abs() as usize)
                } else {
                    y.checked_sub((y_diff).unsigned_abs() as usize)
                };

                let new_x = if x_diff.is_positive() {
                    x.checked_add((x_diff).unsigned_abs() as usize)
                } else {
                    x.checked_sub((x_diff).unsigned_abs() as usize)
                };

                if let (Some(new_y), Some(new_x)) = (new_y, new_x) {
                    let possible_antinode = (new_y, new_x);

                    if grid.get(possible_antinode).is_some() {
                        antinodes.insert(possible_antinode);
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn process_input(data: &str) -> Map {
    let chars: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let y_len = chars.len();
    let x_len = chars[0].len();

    Array2::from_shape_vec((y_len, x_len), chars.iter().flatten().cloned().collect()).unwrap()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let grid = process_input(include_str!("../data/test_input.txt"));

        assert_eq!(part_one_solution(&grid), 14);
    }

    #[test]
    fn test_part_one_with_single_node() {
        let grid = process_input(include_str!("../data/single_node_test_input.txt"));

        assert_eq!(part_one_solution(&grid), 2);
    }

    #[test]
    fn test_part_one_solution() {
        let grid = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&grid), 332);
    }
}

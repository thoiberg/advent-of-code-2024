use std::collections::{HashMap, HashSet};

use ndarray::Array2;

fn main() {
    let grid = process_input(include_str!("../data/puzzle_input.txt"));
    let antennas = antennas(&grid);

    let part_one_answer = part_one_solution(&grid, &antennas);
    println!("The Part One answer is: {part_one_answer}");
}

type Map = Array2<char>;
type AntennaLocations = HashMap<char, Vec<(usize, usize)>>;

fn part_one_solution(grid: &Map, antennas: &AntennaLocations) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for antennas in antennas.values() {
        for antenna in antennas {
            for other in antennas {
                if antenna == other {
                    continue;
                }

                let distance = antenna_distance(antenna, other);

                if let Some(antinode) = antinode_location(antenna, &distance, grid) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}

fn antenna_distance((y, x): &(usize, usize), (other_y, other_x): &(usize, usize)) -> (i32, i32) {
    let y_diff = (*y as i32) - (*other_y as i32);
    let x_diff = (*x as i32) - (*other_x as i32);

    (y_diff, x_diff)
}

fn antinode_location(
    (y, x): &(usize, usize),
    (y_delta, x_delta): &(i32, i32),
    grid: &Map,
) -> Option<(usize, usize)> {
    let new_y = if y_delta.is_positive() {
        y.checked_add((y_delta).unsigned_abs() as usize)
    } else {
        y.checked_sub((y_delta).unsigned_abs() as usize)
    };

    let new_x = if x_delta.is_positive() {
        x.checked_add((x_delta).unsigned_abs() as usize)
    } else {
        x.checked_sub((x_delta).unsigned_abs() as usize)
    };

    if let (Some(new_y), Some(new_x)) = (new_y, new_x) {
        grid.get((new_y, new_x)).map(|_| (new_y, new_x))
    } else {
        None
    }
}

fn antennas(grid: &Map) -> AntennaLocations {
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

    antenna_positions
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
        let antennas = antennas(&grid);

        assert_eq!(part_one_solution(&grid, &antennas), 14);
    }

    #[test]
    fn test_part_one_with_single_node() {
        let grid = process_input(include_str!("../data/single_node_test_input.txt"));
        let antennas = antennas(&grid);

        assert_eq!(part_one_solution(&grid, &antennas), 2);
    }

    #[test]
    fn test_part_one_solution() {
        let grid = process_input(include_str!("../data/puzzle_input.txt"));
        let antennas = antennas(&grid);

        assert_eq!(part_one_solution(&grid, &antennas), 332);
    }
}

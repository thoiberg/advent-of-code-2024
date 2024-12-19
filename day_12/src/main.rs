use std::collections::{HashMap, HashSet};

use ndarray::Array2;

fn main() {
    let farm = process_input(include_str!("../data/puzzle_input.txt"));
    let plots = divide_into_plots(&farm);

    let part_one_answer = part_one_solution(&plots);
    println!("The Part One answer is {part_one_answer}");
}

type Coordinate = (usize, usize); // (y, x)
type Plant = (Coordinate, char, Direction);

fn part_one_solution(gardens: &[Garden]) -> usize {
    gardens.iter().map(|garden| garden.fence_price()).sum()
}

fn divide_into_plots(farm: &Array2<char>) -> Vec<Garden> {
    let full_sides = HashSet::from([
        Direction::Up,
        Direction::Left,
        Direction::Bottom,
        Direction::Right,
    ]);
    let mut gardens = vec![];

    let mut other_shape_coords_to_check: Vec<Coordinate> = Vec::new();
    other_shape_coords_to_check.push((0, 0));

    let mut checked_coords: HashSet<Coordinate> = HashSet::new();

    while let Some(plant) = other_shape_coords_to_check.pop() {
        if checked_coords.contains(&plant) {
            continue;
        }

        let name = farm.get(plant).unwrap();
        let mut garden = Garden::new(*name);
        let mut current_shape_coords_to_check: Vec<Coordinate> = vec![plant];

        while let Some(plant) = current_shape_coords_to_check.pop() {
            if checked_coords.contains(&plant) {
                continue;
            }

            checked_coords.insert(plant);

            let (same_region, different_region): (Vec<Plant>, Vec<Plant>) =
                find_neighbours(farm, &plant)
                    .iter()
                    .partition(|(_, plant_type, _)| plant_type == name);

            let perimeters: HashSet<_> = same_region
                .iter()
                .map(|(_, _, direction)| *direction)
                .collect();

            garden
                .plants
                .insert(plant, full_sides.difference(&perimeters).cloned().collect());

            same_region.into_iter().for_each(|(coord, _, _)| {
                if !checked_coords.contains(&coord) {
                    current_shape_coords_to_check.push(coord);
                }
            });
            different_region.into_iter().for_each(|(coord, _, _)| {
                if !checked_coords.contains(&coord) {
                    other_shape_coords_to_check.push(coord);
                }
            });
        }

        gardens.push(garden);
    }

    gardens
}

fn find_neighbours(
    farm: &Array2<char>,
    plant: &Coordinate,
) -> HashSet<(Coordinate, char, Direction)> {
    [
        (plant.0.checked_sub(1), Some(plant.1), Direction::Up), // top
        (Some(plant.0), plant.1.checked_add(1), Direction::Right), // right
        (plant.0.checked_add(1), Some(plant.1), Direction::Bottom), // bottom
        (Some(plant.0), plant.1.checked_sub(1), Direction::Left), // left
    ]
    .into_iter()
    .filter_map(|coords| {
        if let (Some(y), Some(x), direction) = coords {
            farm.get((y, x)).map(|plant| ((y, x), *plant, direction))
        } else {
            None
        }
    })
    .collect()
}

fn process_input(input: &str) -> Array2<char> {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let y = chars.len();
    let x = chars[0].len();

    Array2::from_shape_vec((y, x), chars.iter().flatten().cloned().collect()).unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Bottom,
    Left,
}

struct Garden {
    #[allow(dead_code)]
    region: char,
    plants: HashMap<Coordinate, HashSet<Direction>>,
}

impl Garden {
    fn new(region: char) -> Self {
        Self {
            plants: HashMap::new(),
            region,
        }
    }

    fn perimeter(&self) -> usize {
        self.plants
            .values()
            .map(|perimeters| perimeters.len())
            .sum()
    }

    fn fence_price(&self) -> usize {
        self.plants.len() * self.perimeter()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_small_example() {
        let farm = process_input(include_str!("../data/small_test_input.txt"));
        let plots = divide_into_plots(&farm);

        assert_eq!(part_one_solution(&plots), 140);
    }

    #[test]
    fn test_divide_plots_small_example() {
        let farm = process_input(include_str!("../data/small_test_input.txt"));

        let plots = divide_into_plots(&farm);

        assert_eq!(plots.len(), 5);

        let a_plot = &plots[0];
        assert_eq!(a_plot.region, 'A');
        assert_eq!(a_plot.plants.len(), 4);
        assert_eq!(a_plot.perimeter(), 10);

        let d_plot = &plots[1];
        assert_eq!(d_plot.region, 'D');
        assert_eq!(d_plot.plants.len(), 1);
        assert_eq!(d_plot.perimeter(), 4);

        let c_plot = &plots[2];
        assert_eq!(c_plot.region, 'C');
        assert_eq!(c_plot.plants.len(), 4);
        assert_eq!(c_plot.perimeter(), 10);

        let e_plot = &plots[3];
        assert_eq!(e_plot.region, 'E');
        assert_eq!(e_plot.plants.len(), 3);
        assert_eq!(e_plot.perimeter(), 8);

        let b_plot = &plots[4];
        assert_eq!(b_plot.region, 'B');
        assert_eq!(b_plot.plants.len(), 4);
        assert_eq!(b_plot.perimeter(), 8);
    }

    #[test]
    fn test_fence_price() {
        let farm = process_input(include_str!("../data/small_test_input.txt"));

        let plots = divide_into_plots(&farm);

        let a_plot = plots.iter().find(|garden| garden.region == 'A').unwrap();
        assert_eq!(a_plot.fence_price(), 40);

        let b_plot = plots.iter().find(|garden| garden.region == 'B').unwrap();
        assert_eq!(b_plot.fence_price(), 32);

        let c_plot = plots.iter().find(|garden| garden.region == 'C').unwrap();
        assert_eq!(c_plot.fence_price(), 40);

        let d_plot = plots.iter().find(|garden| garden.region == 'D').unwrap();
        assert_eq!(d_plot.fence_price(), 4);

        let e_plot = plots.iter().find(|garden| garden.region == 'E').unwrap();
        assert_eq!(e_plot.fence_price(), 24);
    }

    #[test]
    fn test_find_neighbours_small_example() {
        let farm = process_input(include_str!("../data/small_test_input.txt"));
        let neighbours = find_neighbours(&farm, &(1, 0));

        assert_eq!(neighbours.len(), 3);
        assert_eq!(
            neighbours,
            HashSet::from([
                ((0, 0), 'A', Direction::Up),
                ((1, 1), 'B', Direction::Right),
                ((2, 0), 'B', Direction::Bottom)
            ])
        );
    }

    #[test]
    fn test_part_one_nested_example() {
        let farm = process_input(include_str!("../data/nested_test_input.txt"));
        let plots = divide_into_plots(&farm);

        assert_eq!(part_one_solution(&plots), 772);
    }

    #[test]
    fn test_part_one_large_example() {
        let farm = process_input(include_str!("../data/large_test_input.txt"));
        let plots = divide_into_plots(&farm);

        assert_eq!(part_one_solution(&plots), 1930);
    }

    #[test]
    fn test_part_one_answer() {
        let farm = process_input(include_str!("../data/puzzle_input.txt"));
        let plots = divide_into_plots(&farm);

        assert_eq!(part_one_solution(&plots), 1_319_878);
    }
}

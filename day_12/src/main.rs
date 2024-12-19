use std::collections::HashSet;

use ndarray::Array2;

fn main() {
    let farm = process_input(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&farm);
    println!("The Part One answer is {part_one_answer}");
}

type Coordinate = (usize, usize); // (y, x)
type Plant = (Coordinate, char);

fn part_one_solution(farm: &Array2<char>) -> usize {
    let farms = divide_into_plots(farm);

    farms.iter().map(|farm| farm.fence_price()).sum()
}

fn divide_into_plots(farm: &Array2<char>) -> Vec<Garden> {
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
                    .partition(|(_, plant_type)| plant_type == name);

            garden.perimeter += 4 - same_region.len();

            if same_region.len() == 4 {
                garden.areas.insert(plant)
            } else {
                garden.perimeters.insert(plant)
            };

            same_region.into_iter().for_each(|(coord, _)| {
                if !checked_coords.contains(&coord) {
                    current_shape_coords_to_check.push(coord);
                }
            });
            different_region.into_iter().for_each(|(coord, _)| {
                if !checked_coords.contains(&coord) {
                    other_shape_coords_to_check.push(coord);
                }
            });
        }

        gardens.push(garden);
    }

    gardens
}

fn find_neighbours(farm: &Array2<char>, plant: &Coordinate) -> Vec<(Coordinate, char)> {
    [
        (plant.0.checked_sub(1), Some(plant.1)), // top
        (Some(plant.0), plant.1.checked_add(1)), // right
        (plant.0.checked_add(1), Some(plant.1)), // bottom
        (Some(plant.0), plant.1.checked_sub(1)), // left
    ]
    .into_iter()
    .filter_map(|coords| {
        if let (Some(y), Some(x)) = coords {
            farm.get((y, x)).map(|plant| ((y, x), *plant))
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

struct Garden {
    areas: HashSet<Coordinate>,
    perimeters: HashSet<Coordinate>,
    perimeter: usize,
    #[allow(dead_code)]
    region: char,
}

impl Garden {
    fn new(region: char) -> Self {
        Self {
            areas: HashSet::new(),
            perimeters: HashSet::new(),
            perimeter: 0,
            region,
        }
    }

    fn fence_price(&self) -> usize {
        (self.areas.len() + self.perimeters.len()) * self.perimeter
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_small_example() {
        let farm = process_input(include_str!("../data/small_test_input.txt"));

        assert_eq!(part_one_solution(&farm), 140);
    }

    #[test]
    fn test_divide_plots_small_example() {
        let farm = process_input(include_str!("../data/small_test_input.txt"));

        let plots = divide_into_plots(&farm);

        assert_eq!(plots.len(), 5);

        let a_plot = &plots[0];
        assert_eq!(a_plot.region, 'A');
        assert_eq!(a_plot.areas.len(), 0);
        assert_eq!(a_plot.perimeters.len(), 4);
        assert_eq!(a_plot.perimeter, 10);

        let d_plot = &plots[1];
        assert_eq!(d_plot.region, 'D');
        assert_eq!(d_plot.areas.len(), 0);
        assert_eq!(d_plot.perimeters.len(), 1);
        assert_eq!(d_plot.perimeter, 4);

        let c_plot = &plots[2];
        assert_eq!(c_plot.region, 'C');
        assert_eq!(c_plot.areas.len(), 0);
        assert_eq!(c_plot.perimeters.len(), 4);
        assert_eq!(c_plot.perimeter, 10);

        let e_plot = &plots[3];
        assert_eq!(e_plot.region, 'E');
        assert_eq!(e_plot.areas.len(), 0);
        assert_eq!(e_plot.perimeters.len(), 3);
        assert_eq!(e_plot.perimeter, 8);

        let b_plot = &plots[4];
        assert_eq!(b_plot.region, 'B');
        assert_eq!(b_plot.areas.len(), 0);
        assert_eq!(b_plot.perimeters.len(), 4);
        assert_eq!(b_plot.perimeter, 8);
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
            vec![((0, 0), 'A'), ((1, 1), 'B'), ((2, 0), 'B')]
        );
    }

    #[test]
    fn test_part_one_nested_example() {
        let farm = process_input(include_str!("../data/nested_test_input.txt"));

        assert_eq!(part_one_solution(&farm), 772);
    }

    #[test]
    fn test_part_one_large_example() {
        let farm = process_input(include_str!("../data/large_test_input.txt"));

        assert_eq!(part_one_solution(&farm), 1930);
    }

    #[test]
    fn test_part_one_answer() {
        let farm = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&farm), 1_319_878);
    }
}

use std::collections::{HashMap, HashSet};

use ndarray::Array2;

fn main() {
    let data = process_input(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&data);
    println!("The Part One answer is {part_one_answer}");

    let part_two_answer = part_two_solution(&data);
    println!("The Part Two answer is {part_two_answer}");
}

type Coordinate = (usize, usize);

fn part_one_solution(map: &Array2<u32>) -> usize {
    let mut trailhead_courses: HashMap<Coordinate, Vec<Vec<Coordinate>>> = HashMap::new();
    let mut possible_courses = find_trailhead_starts(map);

    while let Some(course) = possible_courses.pop() {
        let first_step = course.first().unwrap();
        let last_step = course.last().unwrap();
        let value = map.get(*last_step).unwrap();

        if value == &9 {
            if let Some(courses_from_same_start) = trailhead_courses.get_mut(first_step) {
                courses_from_same_start.push(course.clone());
            } else {
                trailhead_courses.insert(*first_step, vec![course.clone()]);
            }

            continue;
        }

        cardinal_neighbours(*last_step)
            .iter()
            .filter_map(|coord| map.get(*coord).map(|value| (coord, value)))
            .filter(|(_, step)| step == &&(value + 1))
            .for_each(|(coords, _)| {
                let mut new_course = course.clone();
                new_course.push(*coords);
                possible_courses.push(new_course);
            });
    }

    trailhead_courses
        .values()
        .map(|courses| {
            let unique_courses: HashSet<&Coordinate> = courses
                .iter()
                .map(|course| course.last().unwrap())
                .collect();

            unique_courses.len()
        })
        .sum()
}

fn part_two_solution(map: &Array2<u32>) -> usize {
    let mut trailhead_courses: HashMap<Coordinate, Vec<Vec<Coordinate>>> = HashMap::new();
    let mut possible_courses = find_trailhead_starts(map);

    while let Some(course) = possible_courses.pop() {
        let first_step = course.first().unwrap();
        let last_step = course.last().unwrap();
        let value = map.get(*last_step).unwrap();

        if value == &9 {
            if let Some(courses_from_same_start) = trailhead_courses.get_mut(first_step) {
                courses_from_same_start.push(course.clone());
            } else {
                trailhead_courses.insert(*first_step, vec![course.clone()]);
            }

            continue;
        }

        cardinal_neighbours(*last_step)
            .iter()
            .filter_map(|coord| map.get(*coord).map(|value| (coord, value)))
            .filter(|(_, step)| step == &&(value + 1))
            .for_each(|(coords, _)| {
                let mut new_course = course.clone();
                new_course.push(*coords);
                possible_courses.push(new_course);
            });
    }

    trailhead_courses
        .values()
        .map(|courses| courses.len())
        .sum()
}

fn find_trailhead_starts(map: &Array2<u32>) -> Vec<Vec<Coordinate>> {
    map.indexed_iter()
        .filter(|(_, val)| val == &&0)
        .map(|(coords, _)| vec![coords])
        .collect()
}

fn cardinal_neighbours((y, x): Coordinate) -> Vec<Coordinate> {
    [
        (y.checked_sub(1), Some(x)), // top
        (Some(y), x.checked_add(1)), // right
        (y.checked_add(1), Some(x)), // bottom
        (Some(y), x.checked_sub(1)), // left
    ]
    .into_iter()
    .filter_map(|coords| {
        if let (Some(y), Some(x)) = coords {
            Some((y, x))
        } else {
            None
        }
    })
    .collect()
}

fn process_input(input: &str) -> Array2<u32> {
    let data: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let y_size = data.len();
    let x_size = data[0].len();

    Array2::from_shape_vec((y_size, x_size), data.into_iter().flatten().collect()).unwrap()
}

#[cfg(test)]
mod test_super {
    use ndarray::array;

    use super::*;

    #[test]
    fn test_process_input() {
        let input = include_str!("../data/small_test_input.txt");
        let data = process_input(input);

        assert_eq!(data.dim(), (4, 4));
        assert_eq!(data.row(0), array![0, 1, 2, 3]);
        assert_eq!(data.row(1), array![1, 2, 3, 4]);
        assert_eq!(data.row(2), array![8, 7, 6, 5]);
        assert_eq!(data.row(3), array![9, 8, 7, 6]);
    }

    #[test]
    fn test_part_one_small_example() {
        let data = process_input(include_str!("../data/small_test_input.txt"));
        assert_eq!(part_one_solution(&data), 1);
    }

    #[test]
    fn test_part_one_large_example() {
        let data = process_input(include_str!("../data/large_test_input.txt"));
        assert_eq!(part_one_solution(&data), 36);
    }

    #[test]
    fn test_cardinal_neighbours() {
        let neighbours = cardinal_neighbours((0, 0));

        assert_eq!(neighbours.len(), 2);
        assert_eq!(neighbours[0], (0, 1));
        assert_eq!(neighbours[1], (1, 0));
    }

    #[test]
    fn test_part_one_answer() {
        let data = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&data), 652);
    }

    #[test]
    fn test_part_two_example() {
        let data = process_input(include_str!("../data/large_test_input.txt"));

        assert_eq!(part_two_solution(&data), 81);
    }

    #[test]
    fn test_part_two_answer() {
        let data = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_two_solution(&data), 1432);
    }
}

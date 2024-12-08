// TODO for Part 2:
//  1680 is too high
//  need to simplify the logic, rather than constantly pivoting after moving
//  I want to try only pivoting at the start of each move to see if that provides a more
//  accurate answer. Doing that here resulted in a off by 1 error in all tests

use std::collections::HashSet;

use ndarray::Array2;

fn main() {
    let data = process_input(include_str!("../data/puzzle_input.txt"));
    let guard_path = map_path(&data);

    let part_one_answer = part_one_solution(&guard_path);
    println!("Part one answer is {part_one_answer}");

    let part_two_answer = part_two_solution(&guard_path, &data);
    println!("Part two answer is {part_two_answer}");
}

type GuardPath = Vec<((usize, usize), Direction)>;

fn part_one_solution(path: &GuardPath) -> usize {
    let pos_only = path.iter().map(|(pos, _)| pos);

    let distinct_pos: HashSet<&(usize, usize)> = HashSet::from_iter(pos_only);

    distinct_pos.len()
}

fn part_two_solution(path: &GuardPath, grid: &Array2<char>) -> u32 {
    // 1680 is too high
    // could be a dupe thing? Might try cloning values to another array and skipping if I've already checked
    let mut infinite_loop_count = 0;

    for (i, ((y, x), _)) in path.iter().enumerate().skip(1) {
        let ((prev_y, prev_x), prev_direction) = path[i - 1]; // safe as skipping first element
        let guard = Guard {
            y: prev_y,
            x: prev_x,
            direction: prev_direction,
        };

        // println!("block is at: {:?}", (y, x));
        // println!("guard is at {:?}", guard.position());
        // println!("-------------------------");

        let mut grid_with_block = grid.clone();
        grid_with_block[(*y, *x)] = '#';

        if check_infinite_loop(&grid_with_block, guard) {
            infinite_loop_count += 1;
        };
    }

    infinite_loop_count
}

fn process_input(input: &str) -> Array2<char> {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let y_len = chars.len();
    let x_len = chars[0].len();

    Array2::from_shape_vec((y_len, x_len), chars.iter().flatten().cloned().collect()).unwrap()
}

fn place_guard(grid: &Array2<char>) -> Guard {
    grid.indexed_iter()
        .find(|(_, char)| *char == &'^')
        .map(|((y, x), _)| Guard {
            x,
            y,
            direction: Direction::Top,
        })
        .unwrap()
}

fn check_infinite_loop(grid: &Array2<char>, mut guard: Guard) -> bool {
    let mut still_on_grid = true;
    let mut back_to_start = false;
    let mut walked_places: HashSet<((usize, usize), Direction)> = HashSet::new();

    while still_on_grid && !back_to_start {
        let mut front = guard.front();

        walked_places.insert(guard.position());

        while front
            .and_then(|pos| grid.get(pos))
            .is_some_and(|point| point == &'#')
        {
            guard.turn_right();
            front = guard.front();
        }
        guard.r#move();

        if front.and_then(|f| grid.get(f)).is_some() {
            let next = front.unwrap();

            if walked_places.contains(&(next, guard.direction)) {
                back_to_start = true;
            }
        } else {
            still_on_grid = false;
        }
    }

    back_to_start
}

fn map_path(grid: &Array2<char>) -> GuardPath {
    let mut guard = place_guard(grid);

    let mut still_on_grid = true;
    let mut visited = vec![guard.position()];

    while still_on_grid {
        guard.r#move();

        let mut front = guard.front();

        while front
            .and_then(|pos| grid.get(pos))
            .is_some_and(|point| point == &'#')
        {
            guard.turn_right();
            front = guard.front();
        }

        visited.push(guard.position());

        still_on_grid = front.and_then(|f| grid.get(f)).is_some();
    }

    visited
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn r#move(&mut self) {
        let next_coords = self.front();

        if let Some((next_y, next_x)) = next_coords {
            self.y = next_y;
            self.x = next_x;
        }
    }

    fn position(&self) -> ((usize, usize), Direction) {
        ((self.y, self.x), self.direction)
    }

    fn front(&self) -> Option<(usize, usize)> {
        let next_coords = match self.direction {
            Direction::Top => (self.y.checked_sub(1), Some(self.x)),
            Direction::Left => (Some(self.y), self.x.checked_sub(1)),
            Direction::Bottom => (self.y.checked_add(1), Some(self.x)),
            Direction::Right => (Some(self.y), self.x.checked_add(1)),
        };

        if let (Some(next_y), Some(next_x)) = next_coords {
            Some((next_y, next_x))
        } else {
            None
        }
    }

    fn turn_right(&mut self) {
        let new_direction = match self.direction {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
        };

        self.direction = new_direction;
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let grid = process_input(include_str!("../data/test_input.txt"));
        let guard_path = map_path(&grid);

        assert_eq!(part_one_solution(&guard_path), 41);
    }

    #[test]
    fn test_part_one_solution() {
        let grid = process_input(include_str!("../data/puzzle_input.txt"));
        let guard_path = map_path(&grid);

        assert_eq!(part_one_solution(&guard_path), 4433);
    }

    #[test]
    fn test_part_two_example() {
        let grid = process_input(include_str!("../data/test_input.txt"));
        let guard_path = map_path(&grid);

        assert_eq!(part_two_solution(&guard_path, &grid), 6);
    }

    // #[test]
    // fn test_check_infinite_loop_ironically_does_not_get_stuck() {
    //     let mut grid = process_input(include_str!("../data/puzzle_input.txt"));

    //     //         block is at: (71, 71)
    //     // guard is at: ((72, 71), Top)

    //     grid[(71, 71)] = '#';

    //     let guard = Guard {
    //         x: 71,
    //         y: 72,
    //         direction: Direction::Top,
    //     };

    //     let outcome = check_infinite_loop(&grid, guard, false);

    //     assert!(!outcome);
    // }

    // #[test]
    // fn test_more_checks() {
    //     let mut grid = process_input(include_str!("../data/puzzle_input.txt"));

    //     //         block is at: (65, 117)
    //     // guard is at ((66, 117), Top)

    //     grid[(65, 117)] = '#';

    //     println!("{:?}", grid.get((66, 117)));
    //     println!("{:?}", grid.get((66, 118)));

    //     let guard = Guard {
    //         y: 66,
    //         x: 117,
    //         direction: Direction::Top,
    //     };

    //     let outcome = check_infinite_loop(&grid, guard, false);

    //     assert!(!outcome);
    // }
}

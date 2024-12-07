use std::collections::HashSet;

use ndarray::Array2;

fn main() {
    let data = process_input(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&data);
    println!("Part one answer is {part_one_answer}");
}

fn part_one_solution(grid: &Array2<char>) -> usize {
    let mut guard = place_guard(grid);

    let mut still_walking = true;
    let mut visited = HashSet::new();
    visited.insert(guard.position());

    while still_walking {
        // move the guard
        guard.r#move();

        visited.insert(guard.position());

        let mut front = guard.front();

        while front.is_some()
            && grid.get(front.unwrap()).is_some()
            && grid.get(front.unwrap()).unwrap() == &'#'
        {
            guard.turn_right();
            front = guard.front();
        }

        still_walking = front.and_then(|f| grid.get(f)).is_some();
    }

    visited.len()
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
        let next_coords = match self.direction {
            Direction::Top => (self.y.checked_sub(1), Some(self.x)),
            Direction::Left => (Some(self.y), self.x.checked_sub(1)),
            Direction::Bottom => (self.y.checked_add(1), Some(self.x)),
            Direction::Right => (Some(self.y), self.x.checked_add(1)),
        };

        if let (Some(next_y), Some(next_x)) = next_coords {
            self.y = next_y;
            self.x = next_x;
        }
    }

    fn position(&self) -> (usize, usize) {
        (self.y, self.x)
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

        assert_eq!(part_one_solution(&grid), 41);
    }

    #[test]
    fn test_part_one_solution() {
        let grid = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&grid), 4433);
    }
}

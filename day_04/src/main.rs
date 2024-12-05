use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let data = process_input(input);

    let part_one_answer = part_one_solution(&data);
    println!("The Part One answer is {part_one_answer}");

    let part_two_answer = part_two_solution(&data);
    println!("The Part Two answer is {part_two_answer}");
}

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1), // top left
    (0, -1),  // top
    (1, -1),  // top right
    (-1, 0),  // left
    (1, 0),   // right
    (-1, 1),  // bottom left
    (0, 1),   // bottom
    (1, 1),   // bottom right
];

fn part_one_solution(data: &[Vec<char>]) -> usize {
    let xmas_words = find_xmas_substrings(data);

    xmas_words.len()
}

fn part_two_solution(data: &[Vec<char>]) -> u32 {
    // TODO: reuse searching code, but only look for MAS
    // then I think the rest of the general idea is correct-ish
    let xmas_words = find_mas_substrings(data);

    // iterate through and find all values that have the same A coord
    // check that the corners are present in the word (doesn't matter what is where)
    let mut intersecting_words: HashMap<(usize, usize), Vec<Vec<(usize, usize)>>> = HashMap::new();

    for xmas in xmas_words {
        if let Some(a) = intersecting_words.get_mut(&xmas[1]) {
            a.push(xmas);
        } else {
            intersecting_words.insert(xmas[1], vec![xmas]);
        }
    }

    let mut xdashmas_count = 0;

    intersecting_words
        .iter()
        .for_each(|((a_x, a_y), intersections)| {
            if a_x > &0 && a_y > &0 {
                let top_left = (a_x - 1, a_y - 1); // -1,-1
                let top_right = (a_x + 1, a_y - 1); // 1, -1
                let bottom_left = (a_x - 1, a_y + 1); // -1, 1
                let bottom_right = (a_x + 1, a_y + 1); // 1, 1

                let does_intersect = [top_left, top_right, bottom_right, bottom_left]
                    .iter()
                    .all(|corner| intersections.iter().flatten().any(|pos| pos == corner));

                if does_intersect {
                    xdashmas_count += 1;
                }
            }
            // for a
            // get the corners and check if there is something present in all the corners
            //   if so then add 1
        });

    xdashmas_count
}

fn find_xmas_substrings(grid: &[Vec<char>]) -> Vec<Vec<(usize, usize)>> {
    let mut matching_substr: Vec<Vec<(usize, usize)>> = vec![];

    for (y, row) in grid.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            if letter == &'X' {
                for (x_velocity, y_velocity) in DIRECTIONS {
                    let possible_m = next_char(grid, (x, y), (x_velocity, y_velocity));

                    if let Some((possible_m, m_coordinates)) = possible_m {
                        if possible_m == &'M' {
                            let possible_a =
                                next_char(grid, m_coordinates, (x_velocity, y_velocity));

                            if let Some((possible_a, a_coordinates)) = possible_a {
                                if possible_a == &'A' {
                                    let possible_s =
                                        next_char(grid, a_coordinates, (x_velocity, y_velocity));

                                    if let Some((possible_s, s_coordinates)) = possible_s {
                                        if possible_s == &'S' {
                                            matching_substr.push(vec![
                                                (x, y),
                                                m_coordinates,
                                                a_coordinates,
                                                s_coordinates,
                                            ]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    matching_substr
}

fn find_mas_substrings(grid: &[Vec<char>]) -> Vec<Vec<(usize, usize)>> {
    let mut matching_substr: Vec<Vec<(usize, usize)>> = vec![];

    for (y, row) in grid.iter().enumerate() {
        for (x, letter) in row.iter().enumerate() {
            if letter == &'M' {
                for (x_velocity, y_velocity) in DIRECTIONS {
                    let possible_a = next_char(grid, (x, y), (x_velocity, y_velocity));

                    if let Some((possible_a, a_coordinates)) = possible_a {
                        if possible_a == &'A' {
                            let possible_s =
                                next_char(grid, a_coordinates, (x_velocity, y_velocity));

                            if let Some((possible_s, s_coordinates)) = possible_s {
                                if possible_s == &'S' {
                                    matching_substr.push(vec![
                                        (x, y),
                                        a_coordinates,
                                        s_coordinates,
                                    ]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    matching_substr
}

fn next_char(
    grid: &[Vec<char>],
    current_pos: (usize, usize),
    velocity: (i8, i8),
) -> Option<(&char, (usize, usize))> {
    let x_velocity = velocity.0;
    let y_velocity = velocity.1;
    let x = current_pos.0;
    let y = current_pos.1;

    let next_x = if x_velocity.is_negative() {
        x.checked_sub(x_velocity.unsigned_abs() as usize)
    } else {
        x.checked_add(x_velocity.unsigned_abs() as usize)
    };
    let next_y = if y_velocity.is_negative() {
        y.checked_sub(y_velocity.unsigned_abs() as usize)
    } else {
        y.checked_add(y_velocity.unsigned_abs() as usize)
    };

    if let (Some(x), Some(y)) = (next_x, next_y) {
        let next_char = grid.get(y).and_then(|row| row.get(x));

        next_char.map(|next_char| (next_char, (x, y)))
    } else {
        None
    }
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let test_data = process_input(include_str!("../data/test_input.txt"));

        assert_eq!(part_one_solution(&test_data), 18);
    }

    #[test]
    fn test_part_one_solution() {
        let puzzle_data = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&puzzle_data), 2562);
    }

    #[test]
    fn test_part_two_example() {
        let test_data = process_input(include_str!("../data/test_input.txt"));

        assert_eq!(part_two_solution(&test_data), 9);
    }

    #[test]
    fn test_part_two_solution() {
        let puzzle_data = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_two_solution(&puzzle_data), 1902);
    }
}

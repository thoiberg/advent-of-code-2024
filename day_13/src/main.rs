use regex::Regex;

// maths taken from https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
// because I'm a dummy

fn main() {
    let machines = process_input(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&machines);
    println!("The answer to Part One is {part_one_answer}");

    let part_two_answer = part_two_solution(&machines);
    println!("The answer to Part Two is {part_two_answer}");
}

fn part_one_solution(machines: &[Machine]) -> i64 {
    machines
        .iter()
        .map(|machine| determine_presses(machine, 0))
        .sum()
}

fn part_two_solution(machines: &[Machine]) -> i64 {
    machines
        .iter()
        .map(|machine| determine_presses(machine, 10_000_000_000_000))
        .sum()
}

fn determine_presses(machine: &Machine, offset: i64) -> i64 {
    let ax = machine.ax;
    let ay = machine.ay;
    let bx = machine.bx;
    let by = machine.by;
    let py = machine.py + offset;
    let px = machine.px + offset;

    let determinant = ax * by - ay * bx;
    let a_presses = (px * by - py * bx) / determinant;
    let b_presses = (ax * py - ay * px) / determinant;

    if (
        ay * a_presses + by * b_presses,
        ax * a_presses + bx * b_presses,
    ) == (py, px)
    {
        (3 * a_presses) + b_presses
    } else {
        0
    }
}

fn process_input(input: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"(?m)Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\nButton B: X\+(?<bx>\d+), Y\+(?<by>\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)",
    )
    .unwrap();

    input
        .split("\n\n")
        .map(|machine| {
            let caps = re.captures(machine).unwrap();

            Machine {
                ax: caps.name("ax").unwrap().as_str().parse::<i64>().unwrap(),
                ay: caps.name("ay").unwrap().as_str().parse::<i64>().unwrap(),
                bx: caps.name("bx").unwrap().as_str().parse::<i64>().unwrap(),
                by: caps.name("by").unwrap().as_str().parse::<i64>().unwrap(),
                px: caps.name("px").unwrap().as_str().parse::<i64>().unwrap(),
                py: caps.name("py").unwrap().as_str().parse::<i64>().unwrap(),
            }
        })
        .collect()
}

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_process_input() {
        let machines = process_input(include_str!("../data/test_input.txt"));

        assert_eq!(machines.len(), 4);
        assert_eq!(machines[0].ax, 94);
        assert_eq!(machines[0].ay, 34);
        assert_eq!(machines[0].bx, 22);
        assert_eq!(machines[0].by, 67);
        assert_eq!(machines[0].px, 8400);
        assert_eq!(machines[0].py, 5400);
    }

    #[test]
    fn test_part_one_example() {
        let machines = process_input(include_str!("../data/test_input.txt"));

        assert_eq!(part_one_solution(&machines), 480);
    }

    #[test]
    fn test_part_one_answer() {
        let machines = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&machines), 36_571);
    }

    #[test]
    fn test_part_two_answer() {
        let machines = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_two_solution(&machines), 85_527_711_500_010);
    }
}

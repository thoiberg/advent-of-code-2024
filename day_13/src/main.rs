use regex::Regex;

// maths taken from https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
// because I'm a dummy

fn main() {
    let machines = process_input(include_str!("../data/puzzle_input.txt"));

    let part_one_answer = part_one_solution(&machines);
    println!("The answer to Part One is {part_one_answer}");
}

fn part_one_solution(machines: &[Machine]) -> i32 {
    machines
        .iter()
        .map(|machine| {
            let determinant = machine.ax * machine.by - machine.ay * machine.bx;
            let a_presses = (machine.px * machine.by - machine.py * machine.bx) / determinant;
            let b_presses = (machine.ax * machine.py - machine.ay * machine.px) / determinant;

            if (
                machine.ay * a_presses + machine.by * b_presses,
                machine.ax * a_presses + machine.bx * b_presses,
            ) == (machine.py, machine.px)
            {
                (3 * a_presses) + b_presses
            } else {
                0
            }
        })
        .sum()
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
                ax: caps.name("ax").unwrap().as_str().parse::<i32>().unwrap(),
                ay: caps.name("ay").unwrap().as_str().parse::<i32>().unwrap(),
                bx: caps.name("bx").unwrap().as_str().parse::<i32>().unwrap(),
                by: caps.name("by").unwrap().as_str().parse::<i32>().unwrap(),
                px: caps.name("px").unwrap().as_str().parse::<i32>().unwrap(),
                py: caps.name("py").unwrap().as_str().parse::<i32>().unwrap(),
            }
        })
        .collect()
}

struct Machine {
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    px: i32,
    py: i32,
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
}

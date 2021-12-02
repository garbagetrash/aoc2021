#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
pub struct Command {
    pub dir: Direction,
    pub num: i64,
}

#[aoc_generator(day2)]
pub fn load_input(input: &str) -> Vec<Command> {
    let mut output = vec![];
    for line in input.lines() {
        let cmds: Vec<&str> = line.split(' ').take(2).collect();
        let mut dir = Direction::Up;
        if cmds[0] == "forward" {
            dir = Direction::Forward;
        } else if cmds[0] == "down" {
            dir = Direction::Down;
        }

        let num = cmds[1].parse().unwrap();
        output.push(Command { dir, num });
    }
    output
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> i64 {
    let mut pos = 0;
    let mut depth = 0;
    for d in input {
        if d.dir == Direction::Forward {
            pos += d.num;
        } else if d.dir == Direction::Down {
            depth += d.num;
        } else {
            depth -= d.num;
        }
    }

    pos * depth
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> i64 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for d in input {
        if d.dir == Direction::Forward {
            pos += d.num;
            depth += aim * d.num;
        } else if d.dir == Direction::Down {
            aim += d.num;
        } else {
            aim -= d.num;
        }
    }

    pos * depth
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/02.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/02.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 900);
    }
}

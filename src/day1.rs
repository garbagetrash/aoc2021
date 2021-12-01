use std::collections::VecDeque;

#[aoc_generator(day1)]
pub fn load_input(input: &str) -> Vec<i64> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.parse().unwrap());
    }
    output
}

#[aoc(day1, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut last = input[0];
    let mut inc_count = 0;
    for v1 in input {
        if *v1 > last {
            inc_count += 1;
        }
        last = *v1;
    }

    inc_count
}

#[aoc(day1, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut buf = VecDeque::new();
    buf.push_back(input[0]);
    buf.push_back(input[1]);
    buf.push_back(input[2]);

    let mut last: i64 = buf.iter().sum();

    let mut inc_count = 0;
    for i in 3..input.len() {
        buf.push_back(input[i]);
        buf.pop_front();
        let now = buf.iter().sum();
        if now > last {
            inc_count += 1;
        }
        last = now;
    }

    inc_count
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 5);
    }
}

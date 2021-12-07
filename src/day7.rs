#[aoc_generator(day7)]
pub fn load_input(input: &str) -> Vec<i64> {
    let input = input.lines().next().unwrap();
    let mut output = vec![];
    for num in input.split(',') {
        output.push(num.parse::<i64>().unwrap());
    }
    output
}

#[aoc(day7, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let n = *input.iter().max().unwrap() as i64;
    let mut cost_arr = vec![];
    for col in 0..n {
        let mut cost = 0;
        for value in input {
            cost += (col - *value).abs();
        }
        cost_arr.push(cost);
    }
    *cost_arr.iter().min().unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let n = *input.iter().max().unwrap() as i64;
    let mut cost_arr = vec![];
    for col in 0..n {
        let mut cost = 0;
        for value in input {
            let dist = (col - *value as i64).abs();
            cost += dist * (dist + 1) / 2;
        }
        cost_arr.push(cost);
    }
    *cost_arr.iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/07.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/07.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 168);
    }
}

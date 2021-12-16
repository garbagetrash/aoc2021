use std::collections::HashMap;

#[aoc_generator(day16)]
pub fn load_input(input: &str) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    for line in input.lines() {
        for c in line.chars() {
            let mut hex = c.to_digit(16).unwrap();
            if hex > 7 {
                output.push(1);
            } else {
                output.push(0);
            }

            hex = hex % 8;
            if hex > 3 {
                output.push(1);
            } else {
                output.push(0);
            }
            hex = hex % 4;
            if hex > 1 {
                output.push(1);
            } else {
                output.push(0);
            }
            hex = hex % 2;
            output.push(hex as u8);
        }
    }
    output
}

#[aoc(day16, part1)]
pub fn part1(input: &[u8]) -> usize {
    println!("{:?}", input);
    0
}

#[aoc(day16, part2)]
pub fn part2(input: &[u8]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/16a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 16);

        let input = read_to_string("input/2021/16b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 12);

        let input = read_to_string("input/2021/16c.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 23);

        let input = read_to_string("input/2021/16d.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/16.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}

use std::collections::HashMap;

#[aoc_generator(day21)]
fn load_input(input: &str) -> Vec<Player> {
    let mut output = vec![];
    for line in input.lines() {
        let mut liter = line.split(": ");
        liter.next();
        let pos = liter.next().unwrap().parse::<usize>().unwrap();
        output.push(Player::new(pos));
    }
    output
}

struct DeterministicDice {
    value: usize,
    n_rolls: usize,
}

impl DeterministicDice {
    fn new() -> DeterministicDice {
        DeterministicDice {
            value: 1,
            n_rolls: 0,
        }
    }
}

impl Iterator for DeterministicDice {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let output = self.value;
        self.value += 1;
        if self.value > 100 {
            self.value = 1;
        }
        self.n_rolls += 1;
        Some(self.value)
    }
}

#[derive(Clone, Copy, Debug)]
struct Player {
    score: usize,
    position: usize,
}

impl Player {
    fn new(position: usize) -> Player {
        Player {
            score: 0,
            position: position,
        }
    }
}

#[aoc(day21, part1)]
fn part1(input: &[Player]) -> usize {
    let mut dice = DeterministicDice::new();
    println!("{:?}", input[0]);
    println!("{:?}", input[1]);
    0
}

#[aoc(day21, part2)]
fn part2(input: &[Player]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/21.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/21.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}

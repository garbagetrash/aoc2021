use nalgebra::*;
use std::collections::{HashMap, HashSet};

pub struct InputData(
    Vec<char>,
    HashMap<(char, char), ((char, char), (char, char))>,
);

#[aoc_generator(day14)]
pub fn load_input(input: &str) -> InputData {
    let seed = input.lines().next().unwrap().chars().collect();
    let mut mapping = HashMap::<(char, char), ((char, char), (char, char))>::new();
    for line in input.lines().skip(2) {
        let mut iter = line.split(" -> ");
        let mut keyiter = iter.next().unwrap().chars();
        let first = keyiter.next().unwrap();
        let second = keyiter.next().unwrap();
        let right = iter.next().unwrap().chars().next().unwrap();
        let lvalue = (first, right);
        let rvalue = (right, second);
        mapping.insert((first, second), (lvalue, rvalue));
    }
    InputData(seed, mapping)
}

pub fn create_input_vector(input: &Vec<char>, pairvec: &Vec<(char, char)>) -> Vec<usize> {
    // Create vector of pairs in the input sequence
    let mut inpairs = vec![];
    for i in 0..input.len() - 1 {
        inpairs.push((input[i], input[i + 1]));
    }

    // Create actual vector of count of pairs
    let mut output = vec![];
    for pair in pairvec {
        let s = inpairs.iter().filter(|&x| x == pair).count();
        output.push(s);
    }
    output
}

#[aoc(day14, part1)]
pub fn part1(input: &InputData) -> usize {
    let mut letterset = HashSet::new();
    for (l0, l1) in input.1.keys() {
        letterset.insert(l0);
        letterset.insert(l1);
    }
    let pairvec: Vec<_> = input.1.keys().copied().collect();
    let n = pairvec.len();
    let mut pair_mat_vec = vec![];
    for x in &pairvec {
        let outpairs = input.1.get(x).unwrap();
        let mut column = vec![0usize; n];
        let idx0 = pairvec.iter().position(|&t| t == outpairs.0).unwrap();
        let idx1 = pairvec.iter().position(|&t| t == outpairs.1).unwrap();
        column[idx0] = 1;
        column[idx1] = 1;
        for c in column {
            pair_mat_vec.push(c);
        }
    }
    let pairmatrix = DMatrix::from_vec(n, n, pair_mat_vec);

    let mut outmat = pairmatrix.clone();
    for _ in 1..10 {
        outmat = outmat * pairmatrix.clone();
    }

    let input_vector = DVector::from_vec(create_input_vector(&input.0, &pairvec));
    let counts = outmat * input_vector;

    let mut charcounts = HashMap::new();
    for (i, c) in counts.data.as_vec().iter().enumerate() {
        let pair = pairvec[i];
        if let Some(value) = charcounts.get_mut(&pair.0) {
            *value += *c;
        } else {
            charcounts.insert(pair.0, *c);
        }
    }
    if let Some(value) = charcounts.get_mut(&input.0[input.0.len() - 1]) {
        *value += 1;
    }
    charcounts.values().max().unwrap() - charcounts.values().min().unwrap()
}

#[aoc(day14, part2)]
pub fn part2(input: &InputData) -> usize {
    let mut letterset = HashSet::new();
    for (l0, l1) in input.1.keys() {
        letterset.insert(l0);
        letterset.insert(l1);
    }
    let pairvec: Vec<_> = input.1.keys().copied().collect();
    let n = pairvec.len();
    let mut pair_mat_vec = vec![];
    for x in &pairvec {
        let outpairs = input.1.get(x).unwrap();
        let mut column = vec![0usize; n];
        let idx0 = pairvec.iter().position(|&t| t == outpairs.0).unwrap();
        let idx1 = pairvec.iter().position(|&t| t == outpairs.1).unwrap();
        column[idx0] = 1;
        column[idx1] = 1;
        for c in column {
            pair_mat_vec.push(c);
        }
    }
    let pairmatrix = DMatrix::from_vec(n, n, pair_mat_vec);

    let mut outmat = pairmatrix.clone();
    for _ in 1..40 {
        outmat = outmat * pairmatrix.clone();
    }

    let input_vector = DVector::from_vec(create_input_vector(&input.0, &pairvec));
    let counts = outmat * input_vector;

    let mut charcounts = HashMap::new();
    for (i, c) in counts.data.as_vec().iter().enumerate() {
        let pair = pairvec[i];
        if let Some(value) = charcounts.get_mut(&pair.0) {
            *value += *c;
        } else {
            charcounts.insert(pair.0, *c);
        }
    }
    if let Some(value) = charcounts.get_mut(&input.0[input.0.len() - 1]) {
        *value += 1;
    }
    charcounts.values().max().unwrap() - charcounts.values().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/14.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 1588);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/14.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 2188189693529);
    }
}

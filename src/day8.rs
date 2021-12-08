use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

#[allow(clippy::comparison_to_empty)]
#[aoc_generator(day8)]
pub fn load_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut output = vec![];
    for line in input.lines() {
        let mut asdf = line.split('|');
        let signals: Vec<_> = asdf
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| *x != "")
            .map(String::from)
            .collect();
        let output_values: Vec<_> = asdf
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| *x != "")
            .map(String::from)
            .collect();
        output.push((signals, output_values));
    }
    output
}

#[aoc(day8, part1)]
pub fn part1(input: &[(Vec<String>, Vec<String>)]) -> i64 {
    let mut output = 0;
    for s in input {
        for word in &s.1 {
            let char_cnt = word.chars().count();
            if char_cnt == 2 || char_cnt == 3 || char_cnt == 4 || char_cnt == 7 {
                output += 1;
            }
        }
    }
    output
}

pub fn do_word(s: &(Vec<String>, Vec<String>)) -> i64 {
    let mut dict: HashMap<i64, HashSet<char>> = HashMap::new();
    let mut keys = vec![];
    for word in &s.0 {
        let mut key: HashSet<char> = HashSet::new();
        for c in word.chars() {
            key.insert(c);
        }
        keys.push(key);
    }

    // First ID the unique guys
    for key in &keys {
        let wlen = key.len();
        if wlen == 2 {
            // 1 found
            dict.insert(1, key.clone());
        } else if wlen == 3 {
            // 7 found
            dict.insert(7, key.clone());
        } else if wlen == 4 {
            // 4 found
            dict.insert(4, key.clone());
        } else if wlen == 7 {
            // 8 found
            dict.insert(8, key.clone());
        }
    }

    // 5 doesn't contain 4
    // 9 contains 4!!!
    // 2 doesn't contain 4
    // 3 doesn't contain 4
    // 6 doesn't contain 4
    //
    for key in &keys {
        if key.len() == 2 || key.len() == 3 || key.len() == 4 || key.len() == 7 {
            // Already have it
        } else {
            // Solve 9
            let intersection: HashSet<char> =
                key.intersection(dict.get(&4).unwrap()).copied().collect();
            let set4: &HashSet<char> = dict.get(&4).unwrap();
            if key.len() == 6 && &intersection == set4 {
                dict.insert(9, key.clone());
                continue;
            }

            // Solve 0
            let intersection: HashSet<char> =
                key.intersection(dict.get(&1).unwrap()).copied().collect();
            let set1: &HashSet<char> = dict.get(&1).unwrap();
            if key.len() == 6 && &intersection == set1 {
                dict.insert(0, key.clone());
                continue;
            }

            // Solve 6
            if key.len() == 6 {
                dict.insert(6, key.clone());
                continue;
            }

            // Need 2, 3, 5
        }
    }

    for key in keys {
        if key.len() == 5 {
            // Solve 3
            let intersection: HashSet<char> =
                key.intersection(dict.get(&1).unwrap()).copied().collect();
            let set1: &HashSet<char> = dict.get(&1).unwrap();
            if key.len() == 5 && &intersection == set1 {
                dict.insert(3, key.clone());
                continue;
            }

            // Solve 5
            let intersection: HashSet<char> =
                key.intersection(dict.get(&9).unwrap()).copied().collect();
            let set5: &HashSet<char> = &key;
            if key.len() == 5 && &intersection == set5 {
                dict.insert(5, key.clone());
                continue;
            }

            // Else 2
            dict.insert(2, key.clone());
        }
    }

    // SOLVE
    let mut tempvalue: i64 = 0;
    for word in &s.1 {
        let setword = word.chars().collect::<HashSet<_>>();

        for (k, v) in dict.iter() {
            if *v == setword {
                let digit = k;
                tempvalue *= 10;
                tempvalue += digit;
                break;
            }
        }
    }
    tempvalue
}

#[aoc(day8, part2)]
pub fn part2(input: &[(Vec<String>, Vec<String>)]) -> i64 {
    input.par_iter().map(|s| do_word(s)).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/08.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/08.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 61229);
    }
}

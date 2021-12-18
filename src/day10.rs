use rayon::prelude::*;

#[aoc_generator(day10)]
pub fn load_input(input: &str) -> Vec<String> {
    input.lines().par_bridge().map(String::from).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[String]) -> i64 {
    let mut score = 0;
    for line in input {
        let mut stack = vec![];
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                let last = stack.pop().unwrap();
                if c == ')' && last != '(' {
                    score += 3;
                    break;
                } else if c == ']' && last != '[' {
                    score += 57;
                    break;
                } else if c == '}' && last != '{' {
                    score += 1197;
                    break;
                } else if c == '>' && last != '<' {
                    score += 25137;
                    break;
                }
            }
        }
    }
    score
}

#[allow(clippy::ptr_arg)]
#[allow(clippy::if_same_then_else)]
fn solve_line(line: &String) -> Option<i64> {
    let mut stack = vec![];
    let mut corrupt = false;
    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else {
            let last = stack.pop().unwrap();
            if c == ')' && last != '(' {
                corrupt = true;
                break;
            } else if c == ']' && last != '[' {
                corrupt = true;
                break;
            } else if c == '}' && last != '{' {
                corrupt = true;
                break;
            } else if c == '>' && last != '<' {
                corrupt = true;
                break;
            }
        }
    }

    if !corrupt {
        // If we get here then incomplete line
        let mut score = 0;
        let rev_stack: Vec<_> = stack.iter().rev().collect();
        for c in rev_stack {
            score *= 5;
            if *c == '(' {
                score += 1;
            } else if *c == '[' {
                score += 2;
            } else if *c == '{' {
                score += 3;
            } else if *c == '<' {
                score += 4;
            }
        }
        Some(score)
    } else {
        None
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &[String]) -> i64 {
    let mut scores: Vec<_> = input
        .par_iter()
        .map(solve_line)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    scores.sort_unstable();
    let n = scores.len();
    scores[(n / 2) as usize]
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/10.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/10.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 288957);
    }
}

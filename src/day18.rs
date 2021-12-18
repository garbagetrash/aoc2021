use itertools::Itertools;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FishChar {
    Left,
    Right,
    Value(u32),
    Comma,
}

impl fmt::Debug for FishChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FishChar::Left => write!(f, "\x08\x08["),
            FishChar::Right => write!(f, "\x08\x08]"),
            FishChar::Value(value) => write!(f, "\x08\x08{}", value),
            FishChar::Comma => write!(f, "\x08\x08,"),
        }
    }
}

#[aoc_generator(day18)]
fn load_input(input: &str) -> Vec<Vec<FishChar>> {
    let mut output = vec![];
    for line in input.lines() {
        let mut temp_line = vec![];
        for c in line.chars() {
            if c == '[' {
                temp_line.push(FishChar::Left);
            } else if c == ']' {
                temp_line.push(FishChar::Right);
            } else if c == ',' {
                temp_line.push(FishChar::Comma);
            } else {
                let value = c.to_digit(10).unwrap();
                temp_line.push(FishChar::Value(value));
            }
        }
        output.push(temp_line);
    }
    output
}

fn reduce(fishline: &[FishChar]) -> Vec<FishChar> {
    let mut newline = fishline.to_vec();

    loop {
        // Explode if we can
        let alt_line = explode(&newline);

        if alt_line == newline {
            // Split if no explodes, and we can split
            let alt_line = split(&newline);

            if alt_line == newline {
                // If no exploding or splitting, we're done
                break;
            } else {
                newline = alt_line;
            }
        } else {
            newline = alt_line;
        }
    }
    newline
}

fn split(fishline: &[FishChar]) -> Vec<FishChar> {
    let mut newline = fishline.to_vec();

    for i in 0..fishline.len() {
        if let FishChar::Value(value) = fishline[i] {
            if value >= 10 {
                // SPLITTING FISH
                let left = value / 2;
                let right = value - left;
                newline[i] = FishChar::Value(left);
                newline.insert(i, FishChar::Left);
                newline.insert(i + 2, FishChar::Comma);
                newline.insert(i + 3, FishChar::Value(right));
                newline.insert(i + 4, FishChar::Right);
                break;
            }
        }
    }
    newline
}

fn explode(fishline: &[FishChar]) -> Vec<FishChar> {
    let mut newline = fishline.to_vec();

    let mut depth = 0;
    for i in 0..fishline.len() {
        match fishline[i] {
            FishChar::Left => depth += 1,
            FishChar::Right => depth -= 1,
            _ => (),
        }

        if depth >= 5 {
            // EXPLODING FISH
            // The very next FishChar to the right is the left value
            if let FishChar::Value(left) = fishline[i + 1] {
                // Add left to next left FishChar
                let leftidx = fishline[..i]
                    .iter()
                    .rposition(|&fc| matches!(fc, FishChar::Value(_)));
                if let Some(idx) = leftidx {
                    if let FishChar::Value(lv) = newline[idx] {
                        newline[idx] = FishChar::Value(left + lv);
                    } else {
                        panic!("");
                    }
                }
            } else {
                panic!("This can't happen.");
            }

            if let FishChar::Value(right) = fishline[i + 3] {
                // Add right to next right FishChar
                let rightidx = fishline[i + 4..]
                    .iter()
                    .position(|&fc| matches!(fc, FishChar::Value(_)));
                if let Some(idx) = rightidx {
                    if let FishChar::Value(rv) = newline[i + 4 + idx] {
                        newline[i + 4 + idx] = FishChar::Value(rv + right);
                    } else {
                        panic!("how");
                    }
                }
            } else {
                panic!("This can't happen.");
            }

            // Now we remove this abomination
            newline.remove(i);
            newline.remove(i);
            newline.remove(i);
            if let FishChar::Value(_) = newline[i] {
                newline[i] = FishChar::Value(0);
            } else {
                panic!("Can't happen");
            }
            newline.remove(i + 1);

            return newline;
        }
    }

    newline
}

fn add(left: &[FishChar], right: &[FishChar]) -> Vec<FishChar> {
    let mut output = vec![FishChar::Left];
    output.append(&mut left.to_vec());
    output.push(FishChar::Comma);
    output.append(&mut right.to_vec());
    output.push(FishChar::Right);
    output
}

fn magnitude(fishnum: &[FishChar]) -> u32 {
    let mut newline = fishnum.to_vec();
    while newline.len() > 1 {
        newline = magnitude_iter(&newline);
    }
    if let FishChar::Value(value) = newline[0] {
        value
    } else {
        panic!("Magnitude failed");
    }
}

fn magnitude_iter(fishnum: &[FishChar]) -> Vec<FishChar> {
    let mut newline = fishnum.to_vec();
    for i in 2..fishnum.len() - 2 {
        if fishnum[i] == FishChar::Comma {
            if let FishChar::Value(left) = fishnum[i - 1] {
                if let FishChar::Value(right) = fishnum[i + 1] {
                    let mag = 3 * left + 2 * right;
                    newline.remove(i - 2); // [
                    newline[i - 2] = FishChar::Value(mag); // mag
                    newline.remove(i - 1); // ,
                    newline.remove(i - 1); // old_right
                    newline.remove(i - 1); // ]
                    break;
                }
            }
        }
    }
    newline
}

fn add_mag(left: &[FishChar], right: &[FishChar]) -> u32 {
    let thesum = add(left, right);
    let thesum = reduce(&thesum);
    magnitude(&thesum)
}

#[aoc(day18, part1)]
fn part1(input: &[Vec<FishChar>]) -> u32 {
    let input_clone = input.to_vec();
    let mut accumulator = input[0].clone();
    for line in input_clone.iter().skip(1) {
        accumulator = add(&accumulator, line);
        accumulator = reduce(&accumulator);
    }
    magnitude(&accumulator)
}

#[aoc(day18, part2)]
fn part2(input: &[Vec<FishChar>]) -> u32 {
    input
        .iter()
        .permutations(2)
        .map(|x| add_mag(x[0], x[1]))
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/18.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 4140);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/18.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 3993);
    }
}

use std::collections::{HashSet, HashMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CType {
    East,
    South,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cucumber {
    ctype: CType,
    x: i32,
    y: i32,
}

impl Cucumber {
    fn new(ctype: CType, x: i32, y: i32) -> Cucumber {
        Cucumber {
            ctype,
            x,
            y,
        }
    }
}

#[aoc_generator(day25)]
fn load_input(input: &str) -> HashMap<(i32, i32), Cucumber> {
    let mut output = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'v' {
                let new = Cucumber::new(CType::South, x as i32, y as i32);
                output.insert((x as i32, y as i32), new);
            } else if c == '>' {
                let new = Cucumber::new(CType::East, x as i32, y as i32);
                output.insert((x as i32, y as i32), new);
            }
        }
    }
    output
}

fn step(cucumbers: &mut HashMap<(i32, i32), Cucumber>) -> bool {
    let width = 139;
    let height = 137;

    // First we move the East herd first
    let mut east_moves = vec![];
    for (key, c) in cucumbers.iter() {
        if c.ctype == CType::East {
            let nx = (c.x + 1) % width;
            if cucumbers.contains_key(&(nx, c.y)) {
                // Already a cucumber here, no movement
            } else {
                // Move it East
                east_moves.push(key.clone());
            }
        }
    }

    for m in &east_moves {
        let mut temp = cucumbers.remove(m).unwrap();
        let nx = (temp.x + 1) % width;
        temp.x = nx;
        cucumbers.insert((nx, temp.y), temp);
    }

    // Next we move the South herd
    let mut south_moves = vec![];
    for (key, c) in cucumbers.iter() {
        if c.ctype == CType::South {
            let ny = (c.y + 1) % height;
            if cucumbers.contains_key(&(c.x, ny)) {
                // Already a cucumber here, no movement
            } else {
                // Move it South
                south_moves.push(key.clone());
            }
        }
    }

    for m in &south_moves {
        let mut temp = cucumbers.remove(m).unwrap();
        let ny = (temp.y + 1) % height;
        temp.y = ny;
        cucumbers.insert((temp.x, ny), temp);
    }

    east_moves.len() + south_moves.len() == 0
}

#[aoc(day25, part1)]
fn part1(input: &HashMap<(i32, i32), Cucumber>) -> usize {
    let mut cmap = input.clone();
    let mut counter = 1;
    loop {
        if counter % 1000 == 0 {
            println!("counter: {}", counter);
        }

        if step(&mut cmap) {
            return counter;
        }

        counter += 1;
    }
}

#[aoc(day25, part2)]
fn part2(input: &HashMap<(i32, i32), Cucumber>) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/25.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 58);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/25.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}

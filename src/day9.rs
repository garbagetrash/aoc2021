use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn load_input(input: &str) -> Vec<Vec<u32>> {
    let mut output = vec![];
    for line in input.lines() {
        let mut temp = vec![];
        for value in line.chars() {
            let num = value.to_digit(10).unwrap();
            temp.push(num);
        }
        output.push(temp);
    }
    output
}

fn is_low(map: &[Vec<u32>], x: usize, y: usize) -> bool {
    let check = map[y][x];

    if x > 0 && map[y][x - 1] <= check {
        return false;
    }

    if x < map[0].len() - 1 && map[y][x + 1] <= check {
        return false;
    }

    if y > 0 && map[y - 1][x] <= check {
        return false;
    }

    if y < map.len() - 1 && map[y + 1][x] <= check {
        return false;
    }

    true
}

fn get_neighbors(map: &[Vec<u32>], point: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut output = vec![];
    let x = point.0;
    let y = point.1;
    if x > 0 && map[y][x - 1] != 9 {
        output.push((x - 1, y));
    }

    if x < map[0].len() - 1 && map[y][x + 1] != 9 {
        output.push((x + 1, y));
    }

    if y > 0 && map[y - 1][x] != 9 {
        output.push((x, y - 1));
    }

    if y < map.len() - 1 && map[y + 1][x] != 9 {
        output.push((x, y + 1));
    }
    output
}

#[aoc(day9, part1)]
pub fn part1(input: &[Vec<u32>]) -> i64 {
    let mut output = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            if is_low(input, x, y) {
                output += point + 1;
            }
        }
    }
    output as i64
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec<u32>]) -> i64 {
    let mut low_points = vec![];
    for (y, line) in input.iter().enumerate() {
        for (x, _point) in line.iter().enumerate() {
            if is_low(input, x, y) {
                low_points.push((x, y));
            }
        }
    }

    let mut sizes = vec![];
    for low_point in low_points {
        let mut basin = HashSet::new();
        basin.insert(low_point);

        let mut frontier = HashSet::new();
        for point in get_neighbors(input, &low_point) {
            frontier.insert(point);
        }

        loop {
            // look at points in frontier, but not in basin: frontier.difference(basin)
            let diffset: Vec<_> = frontier.difference(&basin).copied().collect();
            if diffset.is_empty() {
                break;
            }

            for point in &diffset {
                let cands = get_neighbors(input, point);
                for point2 in &cands {
                    if !basin.contains(point2) {
                        // Grow frontier by neighbors of frontier not in basin
                        frontier.insert(*point2);
                    }
                }

                // Grow basin by the diffset (old frontier)
                basin.insert(*point);
            }
        }

        sizes.push(basin.len());
    }

    sizes.sort_unstable();
    sizes = sizes.iter().rev().copied().collect::<Vec<_>>();
    (sizes[0] * sizes[1] * sizes[2]) as i64
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/09.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/09.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 1134);
    }
}

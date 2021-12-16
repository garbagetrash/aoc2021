use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn load_input(input: &str) -> Vec<Vec<u32>> {
    let mut output = vec![];
    for line in input.lines() {
        let mut newline = vec![];
        for c in line.chars() {
            newline.push(c.to_digit(10).unwrap());
        }
        output.push(newline);
    }
    output
}

pub fn print_board(input: &[Vec<u32>]) {
    for row in input {
        for col in row {
            print!("{:?}", col);
        }
        println!("");
    }
}

struct Explorer {
    map: Vec<Vec<u32>>,
    costmap: HashMap<(u32, u32), u32>,
}

impl Explorer {
    pub fn new(map: &Vec<Vec<u32>>) -> Explorer {
        Explorer {
            map: map.clone(),
            costmap: HashMap::new(),
        }
    }

    pub fn explore2(&mut self, goal: (u32, u32)) -> u32 {
        // First we slap the start into the frontier set
        let mut frontier = HashMap::new();
        let start_point = (0_u32, 0_u32);
        frontier.insert(start_point, 0_u32);
        self.costmap.insert(start_point, 0);

        // Now we consider each point in frontier, start
        while frontier.len() > 0 {
            // Grab the lowest cost frontier points first
            let mut cost_arr: Vec<_> = frontier.values().copied().collect();
            cost_arr.sort();
            let mut point = *frontier.keys().next().unwrap();
            for (k, v) in frontier.iter() {
                if *v == cost_arr[0] {
                    point = *k;
                }
            }

            frontier.remove(&point);
            let current_cost = *self.costmap.get(&point).unwrap();

            let neighbors = get_neighbors(point, &self.map);
            for next in neighbors {
                let next_cost = self.map[next.0 as usize][next.1 as usize];
                let cost = current_cost + next_cost;
                if let Some(old_cost) = self.costmap.get_mut(&next) {
                    if *old_cost > cost {
                        *old_cost = cost;
                        frontier.insert(next, cost);
                    }
                } else {
                    self.costmap.insert(next, cost);
                    frontier.insert(next, cost);
                }
            }
        }

        return *self.costmap.get(&goal).unwrap();
    }
}

fn get_neighbors(point: (u32, u32), map: &Vec<Vec<u32>>) -> Vec<(u32, u32)> {
    let mut points = vec![];
    if point.0 > 0 {
        points.push((point.0 - 1, point.1));
    }
    if point.0 < (map.len() - 1) as u32 {
        points.push((point.0 + 1, point.1));
    }
    if point.1 > 0 {
        points.push((point.0, point.1 - 1));
    }
    if point.1 < (map[0].len() - 1) as u32 {
        points.push((point.0, point.1 + 1));
    }

    points
}

#[aoc(day15, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    // 748 is answer
    let mut explorer = Explorer::new(input);
    let goal = ((input.len() - 1) as u32, (input[0].len() - 1) as u32);
    explorer.explore2(goal)
}

#[aoc(day15, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    // Build new map
    // Answer: 3045
    let rx = input.len();
    let ry = input[0].len();
    let mut new_input = vec![];
    for x in 0..5 {
        for ix in 0..rx {
            let mut line = vec![];
            for y in 0..5 {
                let sumidx = x + y;
                for iy in 0..ry {
                    let mut value = input[ix][iy] + sumidx;
                    while value > 9 {
                        value -= 9;
                    }
                    line.push(value);
                }
            }
            new_input.push(line);
        }
    }

    // Solve new map
    let mut explorer = Explorer::new(&new_input);
    let goal = (
        (new_input.len() - 1) as u32,
        (new_input[0].len() - 1) as u32,
    );
    explorer.explore2(goal)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/15.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/15.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 315);
    }
}

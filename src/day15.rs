use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

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
    cost: u32,
    map: Vec<Vec<u32>>,
    costmap: HashMap<(u32, u32), u32>,
}

impl Explorer {
    pub fn new(map: &Vec<Vec<u32>>) -> Explorer {
        Explorer {
            cost: 0,
            map: map.clone(),
            costmap: HashMap::new(),
        }
    }

    pub fn explore(&mut self, goal: (u32, u32)) {
        let prior_value = self.map[goal.0 as usize][goal.1 as usize];
        self.costmap.insert(goal, prior_value);

        // Needed to strobe it twice for answer to settle correctly
        for _ in 0..10 {
            let mut explored: HashSet<(u32, u32)> = HashSet::new();
            explored.insert(goal);
            loop {
                // Create frontier HashSet
                let mut frontier: HashSet<(u32, u32)> = HashSet::new();
                for point in &explored {
                    for pt in get_neighbors(*point, &self.map)
                        .iter()
                        .filter(|x| !explored.contains(x))
                    {
                        frontier.insert(*pt);
                    }
                }

                // Break if no frontier... we're done!
                if frontier.len() == 0 {
                    println!("explored count: {}", explored.len());
                    break;
                }

                // For point in frontier get_neighbors(), filter by neighbors in explored
                // choose among these paths the least cost, save
                for point in &frontier {
                    let costclone = self.costmap.clone();
                    let neighbors = get_neighbors(*point, &self.map);
                    let mut costs: Vec<u32> = vec![];
                    for p in &neighbors {
                        if let Some(value) = costclone.get(p) {
                            costs.push(*value);
                        }
                    }
                    let least_cost_neighbor = costs.iter().min().unwrap();
                    let cost_sum = *least_cost_neighbor + self.map[point.0 as usize][point.1 as usize];
                    if let Some(prior_cost) = self.costmap.get_mut(point) {
                        // Some existing value, take the lesser
                        if *prior_cost > cost_sum {
                            *prior_cost = cost_sum;
                        }
                    } else {
                        self.costmap.insert(*point, cost_sum);
                    }
                }

                // move frontier to explored
                for point in &frontier {
                    explored.insert(*point);
                }
            }
        }
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
    explorer.explore(goal);
    *explorer.costmap.get(&(0, 0)).unwrap() - input[0][0]
}

#[aoc(day15, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    // Build new map
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
    let goal = ((new_input.len() - 1) as u32, (new_input[0].len() - 1) as u32);
    explorer.explore(goal);
    *explorer.costmap.get(&(0, 0)).unwrap() - new_input[0][0]
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

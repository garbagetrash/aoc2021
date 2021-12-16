use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Size {
    Small,
    Large,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Edge {
    pub node0: String,
    pub node1: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Cave {
    pub name: String,
    pub size: Size,
}

impl Cave {
    pub fn new(name: &str) -> Cave {
        let mut size = Size::Small;
        if name.to_uppercase() == name {
            size = Size::Large;
        }
        Cave {
            name: String::from(name),
            size: size,
        }
    }
}

#[derive(Debug)]
pub struct CaveSystem {
    pub caves: HashSet<Cave>,
    pub edges: HashSet<Edge>,
    pub count_map: HashMap<String, usize>,
}

impl CaveSystem {
    pub fn new() -> CaveSystem {
        CaveSystem {
            caves: HashSet::new(),
            edges: HashSet::new(),
            count_map: HashMap::new(),
        }
    }

    pub fn neighbors(&self, cave: &str) -> Vec<String> {
        let mut output = vec![];
        for e in &self.edges {
            if e.node0 == cave || e.node1 == cave {
                if e.node0 != cave {
                    output.push(e.node0.clone());
                } else {
                    output.push(e.node1.clone());
                }
            }
        }
        output
    }

    pub fn traverse(&self, path: Vec<String>) -> Vec<Vec<String>> {
        let start_conns = self.neighbors(&path[path.len() - 1]);
        let valid_choices: Vec<String> = start_conns
            .iter()
            .filter(|c| can_visit(&path, &c))
            .map(|x| (*x).clone())
            .collect();
        let mut paths = vec![path.clone()];
        for conn in valid_choices {
            let mut new_path = path.clone();
            new_path.push(conn.clone());
            if conn == "end" {
                paths.push(new_path);
            } else {
                for rpath in self.traverse(new_path) {
                    paths.push(rpath);
                }
            }
        }
        paths
    }

    pub fn traverse2(&self, path: Vec<String>) -> Vec<Vec<String>> {
        let start_conns = self.neighbors(&path[path.len() - 1]);
        let valid_choices: Vec<String> = start_conns
            .iter()
            .filter(|c| can_visit2(&path, &c))
            .map(|x| (*x).clone())
            .collect();
        let mut paths = vec![path.clone()];
        for conn in valid_choices {
            let mut new_path = path.clone();
            new_path.push(conn.clone());
            if conn == "end" {
                paths.push(new_path);
            } else {
                for rpath in self.traverse2(new_path) {
                    paths.push(rpath);
                }
            }
        }
        paths
    }
}

pub fn visited_any_twice(path: &Vec<String>) -> bool {
    for (i, c) in path.iter().enumerate() {
        for (j, other) in path.iter().enumerate() {
            if c == other && i != j && Cave::new(c).size == Size::Small {
                return true;
            }
        }
    }
    false
}

pub fn can_visit(path: &Vec<String>, cave: &str) -> bool {
    for c in path {
        if cave == c && Cave::new(cave).size == Size::Small {
            return false;
        }
    }
    true
}

pub fn can_visit2(path: &Vec<String>, cave: &str) -> bool {
    if visited_any_twice(path) {
        for c in path {
            if cave == c && Cave::new(cave).size == Size::Small {
                return false;
            }
        }
    }
    if cave == "start" {
        return false;
    }
    true
}

#[aoc_generator(day12)]
pub fn load_input(input: &str) -> CaveSystem {
    let mut cave_system = CaveSystem::new();
    for line in input.lines() {
        let mut linegen = line.split('-');
        let left = linegen.next().unwrap();
        let right = linegen.next().unwrap();
        cave_system.caves.insert(Cave::new(left));
        cave_system.caves.insert(Cave::new(right));
        cave_system.edges.insert(Edge {
            node0: String::from(left),
            node1: String::from(right),
        });
    }
    cave_system
}

#[aoc(day12, part1)]
pub fn part1(input: &CaveSystem) -> usize {
    let cs = input.clone();
    let mut paths = cs.traverse(vec!["start".to_string()]);
    paths = paths
        .iter()
        .filter(|p| p[p.len() - 1] == "end")
        .map(|p| (*p).clone())
        .collect();
    paths.len()
}

#[aoc(day12, part2)]
pub fn part2(input: &CaveSystem) -> usize {
    let cs = input.clone();
    let mut paths = cs.traverse2(vec!["start".to_string()]);
    paths = paths
        .iter()
        .filter(|p| p[p.len() - 1] == "end")
        .map(|p| (*p).clone())
        .collect();
    paths.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 10);

        let input = read_to_string("input/2021/12b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 19);

        let input = read_to_string("input/2021/12c.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 226);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 36);

        let input = read_to_string("input/2021/12b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 103);

        let input = read_to_string("input/2021/12c.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 3509);
    }
}

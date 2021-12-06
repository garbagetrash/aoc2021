use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Fish {
    pub timer: usize,
    pub day_spawned: usize,
}

impl Fish {
    pub fn days_of_reproduction(&self, end: usize) -> Vec<usize> {
        let mut times = vec![];
        for v in (self.day_spawned + self.timer + 1..end + 1).step_by(7) {
            times.push(v);
        }
        times
    }

    pub fn tuple(&self) -> (usize, usize) {
        (self.timer, self.day_spawned)
    }

    pub fn count_tree(&self, end: usize, fish_oracle: &HashMap<(usize, usize), usize>) -> usize {
        if let Some(answer) = fish_oracle.get(&self.tuple()) {
            *answer
        } else {
            let mut total: usize = 1;
            let days = self.days_of_reproduction(end);
            for day in days {
                let new_fish = Fish {
                    timer: 8,
                    day_spawned: day,
                };
                if let Some(answer) = fish_oracle.get(&new_fish.tuple()) {
                    total += answer;
                } else {
                    panic!("nope");
                }
            }
            total
        }
    }
}

#[aoc_generator(day6)]
pub fn load_input(input: &str) -> Vec<Fish> {
    let input = input.lines().next().unwrap();
    let mut output = vec![];
    for fish in input.split(',') {
        output.push(Fish {
            timer: fish.parse::<usize>().unwrap(),
            day_spawned: 0,
        });
    }
    output
}

#[aoc(day6, part1)]
pub fn part1(input: &[Fish]) -> usize {
    let mut fish_oracle: HashMap<(usize, usize), usize> = HashMap::new();

    for d in (0..81).rev() {
        for i in (0..9).rev() {
            let f = Fish {
                timer: i,
                day_spawned: d,
            };
            let count = f.count_tree(80, &fish_oracle);
            fish_oracle.insert(f.tuple(), count);
        }
    }

    let mut total = 0;
    for fish in input {
        total += fish.count_tree(80, &fish_oracle);
    }
    total
}

#[aoc(day6, part2)]
pub fn part2(input: &[Fish]) -> usize {
    let mut fish_oracle: HashMap<(usize, usize), usize> = HashMap::new();

    for d in (0..257).rev() {
        for i in (0..9).rev() {
            let f = Fish {
                timer: i,
                day_spawned: d,
            };
            let count = f.count_tree(256, &fish_oracle);
            fish_oracle.insert(f.tuple(), count);
        }
    }

    let mut total = 0;
    for fish in input {
        total += fish.count_tree(256, &fish_oracle);
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/06.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 5934);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/06.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 26984457539);
    }
}

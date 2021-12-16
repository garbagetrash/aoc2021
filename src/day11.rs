struct Board {
    counter: Vec<Vec<u32>>,
    flash: [[bool; 10]; 10],
}

impl Board {
    pub fn new(counter: &Vec<Vec<u32>>) -> Board {
        Board {
            counter: counter.clone(),
            flash: [[false; 10]; 10],
        }
    }

    pub fn increment(&mut self) {
        for row in &mut self.counter {
            for col in row {
                *col += 1;
            }
        }
    }

    pub fn flash_point(&mut self, point: (usize, usize)) -> usize {
        let x = point.0;
        let y = point.1;
        let mut ncnt = 0;
        if !self.flash[x][y] && self.counter[x][y] > 9 {
            self.flash[x][y] = true;
            ncnt += 1;
            // Increment neighbors
            let neighbors = get_neighbors((x, y));
            for neighbor in neighbors {
                let nx = neighbor.0;
                let ny = neighbor.1;
                if !self.flash[nx][ny] {
                    self.counter[nx][ny] += 1;
                }
                ncnt += self.flash_point(neighbor);
            }
        }
        ncnt
    }

    pub fn flash(&mut self) -> usize {
        let mut ncnt = 0;
        let mut last_ncnt = ncnt;
        loop {
            // Check board
            for x in 0..10 {
                for y in 0..10 {
                    ncnt += self.flash_point((x, y));
                }
            }

            if ncnt == last_ncnt {
                break;
            }
            last_ncnt = ncnt;
        }

        // Reset state
        self.flash = [[false; 10]; 10];
        for x in 0..10 {
            for y in 0..10 {
                if self.counter[x][y] >= 10 {
                    self.counter[x][y] = 0;
                }
            }
        }
        ncnt
    }

    pub fn check_sync(&self) -> bool {
        for x in 0..10 {
            for y in 0..10 {
                if self.counter[x][y] != 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[aoc_generator(day11)]
pub fn load_input(input: &str) -> Vec<Vec<u32>> {
    let mut output = vec![];
    for line in input.lines() {
        let mut out_line = vec![];
        for c in line.chars() {
            out_line.push(c.to_digit(10).unwrap());
        }
        output.push(out_line);
    }
    output
}

fn get_neighbors(point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut points = vec![];
    if point.0 > 0 {
        points.push((point.0 - 1, point.1));
        if point.1 > 0 {
            points.push((point.0 - 1, point.1 - 1));
        }
        if point.1 < 9 {
            points.push((point.0 - 1, point.1 + 1));
        }
    }
    if point.0 < 9 {
        points.push((point.0 + 1, point.1));
        if point.1 > 0 {
            points.push((point.0 + 1, point.1 - 1));
        }
        if point.1 < 9 {
            points.push((point.0 + 1, point.1 + 1));
        }
    }
    if point.1 > 0 {
        points.push((point.0, point.1 - 1));
    }
    if point.1 < 9 {
        points.push((point.0, point.1 + 1));
    }

    points
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> usize {
    let mut score = 0;
    let mut board = Board::new(input);

    for _ in 0..100 {
        //board.print_board();
        board.increment();
        //board.print_board();
        let ncnt = board.flash();
        score += ncnt;
    }
    score
}

#[aoc(day11, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> usize {
    let mut board = Board::new(input);
    let mut idx = 0;
    loop {
        idx += 1;
        board.increment();
        board.flash();
        if board.check_sync() {
            break;
        }
    }
    idx
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/11.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 1656);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/11.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 195);
    }
}

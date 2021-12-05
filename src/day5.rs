#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub p1: (usize, usize),
    pub p2: (usize, usize),
}

impl Segment {
    #[allow(clippy::needless_range_loop)]
    pub fn fill(&self, board: &mut Vec<Vec<usize>>, part2: bool) {
        if self.p1.0 == self.p2.0 {
            // Horizontal line
            let x = self.p1.0;
            let starty = self.p1.1.min(self.p2.1);
            let stopy = self.p1.1.max(self.p2.1);
            for y in starty..(stopy + 1) {
                board[x][y] += 1;
            }
        } else if self.p1.1 == self.p2.1 {
            // Vertical line
            let y = self.p1.1;
            let startx = self.p1.0.min(self.p2.0);
            let stopx = self.p1.0.max(self.p2.0);
            for x in startx..(stopx + 1) {
                board[x][y] += 1;
            }
        } else if part2 {
            // Diagonal
            let mut startx = self.p1.0;
            let mut stopx = self.p2.0;
            let mut starty = self.p1.1;
            let mut stopy = self.p2.1;
            if stopx < startx {
                startx = stopx;
                stopx = self.p1.0;
                starty = stopy;
                stopy = self.p1.1;
            }
            for i in 0..(stopx - startx + 1) {
                let x = startx + i;
                let mut y = starty + i;
                if stopy < starty {
                    y = starty - i;
                }
                board[x][y] += 1;
            }
        }
    }
}

#[aoc_generator(day5)]
pub fn load_input(input: &str) -> Vec<Segment> {
    let mut output = vec![];
    for line in input.lines() {
        let mut _line = line.split(" -> ");
        let mut v1 = _line.next().unwrap().split(',');
        let x1 = v1.next().unwrap().parse::<usize>().unwrap();
        let y1 = v1.next().unwrap().parse::<usize>().unwrap();

        let mut v2 = _line.next().unwrap().split(',');
        let x2 = v2.next().unwrap().parse::<usize>().unwrap();
        let y2 = v2.next().unwrap().parse::<usize>().unwrap();

        output.push(Segment {
            p1: (x1, y1),
            p2: (x2, y2),
        });
    }
    output
}

#[allow(clippy::needless_range_loop)]
pub fn count_board(board: &[Vec<usize>]) -> usize {
    let mut total = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if board[x][y] >= 2 {
                total += 1;
            }
        }
    }
    total
}

#[aoc(day5, part1)]
pub fn part1(input: &[Segment]) -> usize {
    let mut board = vec![vec![0_usize; 1000]; 1000];
    for segment in input {
        segment.fill(&mut board, false);
    }
    count_board(&board)
}

#[aoc(day5, part2)]
pub fn part2(input: &[Segment]) -> usize {
    let mut board = vec![vec![0_usize; 1000]; 1000];
    for segment in input {
        segment.fill(&mut board, true);
    }
    count_board(&board)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/05.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/05.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 12);
    }
}

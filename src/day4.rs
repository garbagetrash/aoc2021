#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub rows: [[u8; 5]; 5],
    pub markers: [[u8; 5]; 5],
}

impl Board {
    pub fn mark(&mut self, value: u8) -> Option<i64> {

        // Put marker on
        for (i, row) in self.rows.iter().enumerate() {
            for (j, rv) in row.iter().enumerate() {
                if value == *rv {
                    self.markers[i][j] = 1;
                }
            }
        }

        if self.check_win() {
            let sumboard = self.get_unmarked_sum();
            return Some(sumboard * value as i64);
        }

        None
    }

    pub fn get_unmarked_sum(&self) -> i64 {
        let mut mysum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.markers[i][j] == 0 {
                    mysum += self.rows[i][j] as i64;
                }
            }
        }
        mysum
    }

    pub fn check_win(&self) -> bool {
        // Check rows
        for r in 0..5 {
            if (0..5).map(|x| self.markers[x][r]).product::<u8>() == 1 {
                return true;
            }
        }

        // Check columns
        for c in 0..5 {
            if (0..5).map(|x| self.markers[c][x]).product::<u8>() == 1 {
                return true;
            }
        }

        // If we got here everything failed, no win
        false
    }
}

#[aoc_generator(day4)]
pub fn load_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let inputs = input.lines().next().unwrap();
    let inputs: Vec<_> = inputs.split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    let markers = [[0u8; 5]; 5];
    let mut boards = vec![];
    let mut rows = [[0u8; 5]; 5];
    let mut rcntr = 0;
    for line in input.lines().skip(2) {

        if line != "" {
            let mut cntr = 0;
            for (i, value) in line.split(' ').enumerate() {
                if let Ok(num) = value.parse::<u8>() {
                    rows[rcntr][cntr] = num;
                    cntr += 1;
                } else {
                    // Skip empty spaces
                }
            }
            rcntr += 1;

            if rcntr == 5 {
                boards.push(Board { rows, markers });
                let mut rows = [[0u8; 5]; 5];
                rcntr = 0;
            }
        } else {
            // skip \n
        }
    }

    (inputs, boards)
}

#[aoc(day4, part1)]
pub fn part1(input: &(Vec<u8>, Vec<Board>)) -> i64 {
    let mut boards = input.1.clone();

    for value in &input.0 {
        for board in boards.iter_mut() {
            if let Some(win) = board.mark(*value) {
                return win;
            }
        }
    }

    panic!("Shouldn't get here, no win?");
}

#[aoc(day4, part2)]
pub fn part2(input: &(Vec<u8>, Vec<Board>)) -> i64 {
    let mut boards = input.1.clone();

    let mut flags = vec![1; boards.len()];

    let mut idx: i64 = -1;
    for value in &input.0 {
        for (i, board) in boards.iter_mut().enumerate() {
            if let Some(win) = board.mark(*value) {
                flags[i] = 0;
            }
        }
        let cntr: usize = flags.iter().sum();

        if cntr == 1 {
            for (i, c) in flags.iter().enumerate() {
                if *c == 1 {
                    idx = i as i64;
                    break;
                }
            }
            break;
        }
    }

    boards = input.1.clone();
    for value in &input.0 {
        if let Some(win) = boards[idx as usize].mark(*value) {
            return win;
        }
    }

    panic!("Shouldn't get here, no win?");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/04.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 4512);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/04.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 230);
    }
}

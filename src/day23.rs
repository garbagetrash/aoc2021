use std::collections::HashSet;

#[aoc_generator(day23)]
fn load_input(input: &str) -> Vec<char> {
    let mut output = vec![];
    for line in input.lines().skip(2) {
        for c in line.chars().filter(|c| c.is_alphabetic()) {
            output.push(c);
        }
    }
    output
}

struct Game {
    states_to_play: HashSet<State>,
    best_score: usize,
}

impl Game {
    fn from_letters(letters: &[char]) -> Game {
        let mut states_to_play = HashSet::new();
        states_to_play.insert(State::from_letters(letters));
        Game {
            states_to_play,
            best_score: usize::MAX,
        }
    }

    fn step(&mut self) {
        let next_states: Vec<Vec<State>> = self
            .states_to_play
            .iter()
            .map(|s| s.valid_moves())
            .collect();
        let mut next_game_states = HashSet::new();

        for svec in next_states {
            for s in svec {
                if s.finished_state() && s.cost < self.best_score {
                    self.best_score = s.cost;
                }
                if s.cost < self.best_score {
                    next_game_states.insert(s);
                }
            }
        }
        self.states_to_play = next_game_states;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    hallway: [char; 7],
    rooms: [[char; 2]; 4],
    cost: usize,
}

fn letter_value(letter: char) -> usize {
    match letter {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Invalid letter"),
    }
}

impl State {
    fn from_letters(letters: &[char]) -> State {
        let mut rooms = [['.', '.'], ['.', '.'], ['.', '.'], ['.', '.']];
        rooms[0][0] = letters[0];
        rooms[0][1] = letters[4];
        rooms[1][0] = letters[1];
        rooms[1][1] = letters[5];
        rooms[2][0] = letters[2];
        rooms[2][1] = letters[6];
        rooms[3][0] = letters[3];
        rooms[3][1] = letters[7];
        State {
            hallway: ['.'; 7],
            rooms,
            cost: 0,
        }
    }

    fn finished_state(&self) -> bool {
        for ridx in 0..4 {
            if !self.room_in_phase_2(ridx) {
                return false;
            }
        }

        if self.hallway.iter().filter(|&&h| h != '.').count() > 0 {
            return false;
        }
        true
    }

    // #############
    // #01.2.3.4.56#
    // ###0#1#2#3###
    //   #0#1#2#3#
    fn moves_left(&self, ridx: usize) -> Vec<State> {
        let mut output = vec![];
        // Bail immediately if nothing to move
        if self.rooms[ridx][0] == '.' && self.rooms[ridx][1] == '.' {
            return output;
        }
        // Bail if room is in phase 2
        if self.room_in_phase_2(ridx) {
            return output;
        }
        let start = 1 + ridx;
        let mut cntr = 0;

        let mut hidx = start - cntr;
        let mut hallway_moves = 1;
        while self.hallway[hidx] == '.' {
            let mut temp = *self;
            let mut didx = 0;
            while temp.rooms[ridx][didx] == '.' {
                didx += 1;
            }
            let letter = temp.rooms[ridx][didx];
            temp.rooms[ridx][didx] = '.';
            temp.hallway[hidx] = letter;

            // Cost to move to hallway
            let mut cost = 1 + didx;
            // Cost to move along hallway
            cost += hallway_moves;
            temp.cost += cost * letter_value(letter);

            output.push(temp);

            // We're done
            if hidx == 0 {
                break;
            } else if hidx == 1 {
                hallway_moves += 1;
            } else {
                hallway_moves += 2;
            }
            cntr += 1;
            hidx = start - cntr;
        }
        output
    }

    fn moves_right(&self, ridx: usize) -> Vec<State> {
        let mut output = vec![];
        // Bail immediately if nothing to move
        if self.rooms[ridx].iter().filter(|&&x| x != '.').count() == 0 {
            return output;
        }
        // Bail if room is in phase 2
        if self.room_in_phase_2(ridx) {
            return output;
        }
        let start = 2 + ridx;
        let mut cntr = 0;

        let mut hidx = start + cntr;
        let mut hallway_moves = 1;
        while self.hallway[hidx] == '.' {
            let mut temp = *self;
            let mut didx = 0;
            while temp.rooms[ridx][didx] == '.' {
                didx += 1;
            }
            let letter = temp.rooms[ridx][didx];
            temp.rooms[ridx][didx] = '.';
            temp.hallway[hidx] = letter;

            // Cost to move to hallway
            let mut cost = 1 + didx;
            // Cost to move along hallway
            cost += hallway_moves;
            temp.cost += cost * letter_value(letter);

            output.push(temp);

            // We're done
            if hidx == 6 {
                break;
            } else if hidx == 5 {
                hallway_moves += 1;
            } else {
                hallway_moves += 2;
            }
            cntr += 1;
            hidx = start + cntr;
        }
        output
    }

    fn room_in_phase_2(&self, ridx: usize) -> bool {
        let expected_letter = match ridx {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => panic!("Invalid room index"),
        };
        self.rooms[ridx]
            .iter()
            .filter(|&&r| r != expected_letter && r != '.')
            .count()
            == 0
    }

    fn hallway_to_room_moves(&self, ridx: usize) -> Vec<State> {
        let mut output = vec![];

        // First check to see if this room is even ready yet
        if !self.room_in_phase_2(ridx) {
            return output;
        }

        // Room only admits certain letters if in phase 2
        let expected_letter = match ridx {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => panic!("Invalid room index"),
        };

        // Room is ready if we get here
        for hidx in 0..7 {
            // Make sure there's an appropriate letter here
            if self.hallway[hidx] != expected_letter {
                continue;
            }

            let mut valid = true;
            if ridx + 2 > hidx {
                // Moving to the right
                for i in hidx + 1..ridx + 2 {
                    if self.hallway[i] != '.' {
                        valid = false;
                    }
                }
            } else {
                // Moving to the left
                for i in ridx + 2..hidx {
                    if self.hallway[i] != '.' {
                        valid = false;
                    }
                }
            }

            if valid {
                // If we have a valid letter, and the path to the room is
                // unblocked, we can move
                let mut temp = *self;
                let letter = temp.hallway[hidx];
                temp.hallway[hidx] = '.';
                let mut didx = temp.rooms[ridx].len() - 1;
                while temp.rooms[ridx][didx] != '.' && didx > 0 {
                    didx -= 1;
                }
                temp.rooms[ridx][didx] = letter;
                // Cost to move to room
                let mut cost = 2 + didx;
                // Cost to move along hallway
                let mut moves = 0;
                if ridx + 2 > hidx {
                    // Moving to the right
                    if hidx == 0 {
                        moves += 1;
                        moves += 2 * (ridx - hidx);
                    } else {
                        moves = 2 * (ridx - (hidx - 1));
                    }
                } else {
                    // Moving to the left
                    if hidx == 6 {
                        moves += 1;
                        moves += 2 * (hidx - 3 - ridx);
                    } else {
                        moves = 2 * (hidx - 2 - ridx);
                    }
                }
                cost += moves;
                temp.cost += cost * letter_value(letter);

                output.push(temp);
            }
        }
        output
    }

    fn valid_moves(&self) -> Vec<State> {
        let mut output = vec![];
        for ridx in 0..4 {
            // Now enumerate hallway -> room moves
            // NOTE: IF any of these are possible, they are always the best
            // move to make, so we won't even bother reporting the others
            let moves = self.hallway_to_room_moves(ridx);
            if !moves.is_empty() {
                for m in moves {
                    output.push(m);
                }
                return output;
            }
            // First enumerate room -> hallway moves to the left
            let moves = self.moves_left(ridx);
            for m in moves {
                output.push(m);
            }
            // Now enumerate room -> hallway moves to the right
            let moves = self.moves_right(ridx);
            for m in moves {
                output.push(m);
            }
        }
        output
    }
}

#[aoc(day23, part1)]
fn part1(input: &[char]) -> usize {
    let mut game = Game::from_letters(input);

    // Play all possible games
    while !game.states_to_play.is_empty() {
        game.step();
    }
    game.best_score
}

struct Game2 {
    states_to_play: HashSet<State2>,
    best_score: usize,
}

impl Game2 {
    fn from_letters(letters: &[char]) -> Game2 {
        let mut states_to_play = HashSet::new();
        states_to_play.insert(State2::from_letters(letters));
        Game2 {
            states_to_play,
            best_score: usize::MAX,
        }
    }

    fn step(&mut self) {
        let next_states: Vec<Vec<State2>> = self
            .states_to_play
            .iter()
            .map(|s| s.valid_moves())
            .collect();
        let mut next_game_states = HashSet::new();

        for svec in next_states {
            for s in svec {
                if s.finished_state() && s.cost < self.best_score {
                    self.best_score = s.cost;
                }
                if s.cost < self.best_score {
                    next_game_states.insert(s);
                }
            }
        }
        self.states_to_play = next_game_states;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State2 {
    hallway: [char; 7],
    rooms: [[char; 4]; 4],
    cost: usize,
}

impl State2 {
    fn from_letters(letters: &[char]) -> State2 {
        let mut rooms = [
            ['.', '.', '.', '.'],
            ['.', '.', '.', '.'],
            ['.', '.', '.', '.'],
            ['.', '.', '.', '.'],
        ];

        rooms[0][0] = letters[0];
        rooms[0][1] = letters[4];
        rooms[0][2] = letters[8];
        rooms[0][3] = letters[12];

        rooms[1][0] = letters[1];
        rooms[1][1] = letters[5];
        rooms[1][2] = letters[9];
        rooms[1][3] = letters[13];

        rooms[2][0] = letters[2];
        rooms[2][1] = letters[6];
        rooms[2][2] = letters[10];
        rooms[2][3] = letters[14];

        rooms[3][0] = letters[3];
        rooms[3][1] = letters[7];
        rooms[3][2] = letters[11];
        rooms[3][3] = letters[15];

        State2 {
            hallway: ['.'; 7],
            rooms,
            cost: 0,
        }
    }

    fn finished_state(&self) -> bool {
        for ridx in 0..4 {
            if !self.room_in_phase_2(ridx) {
                return false;
            }
        }

        if self.hallway.iter().filter(|&&h| h != '.').count() > 0 {
            return false;
        }
        true
    }

    // #############
    // #01.2.3.4.56#
    // ###0#1#2#3###
    //   #0#1#2#3#
    //   #0#1#2#3#
    //   #0#1#2#3#
    fn moves_left(&self, ridx: usize) -> Vec<State2> {
        let mut output = vec![];
        // Bail immediately if nothing to move
        if self.rooms[ridx].iter().filter(|&&x| x != '.').count() == 0 {
            return output;
        }
        // Bail if room is in phase 2
        if self.room_in_phase_2(ridx) {
            return output;
        }
        let start = 1 + ridx;
        let mut cntr = 0;

        let mut hidx = start - cntr;
        let mut hallway_moves = 1;
        while self.hallway[hidx] == '.' {
            let mut temp = *self;
            let mut didx = 0;
            while temp.rooms[ridx][didx] == '.' {
                didx += 1;
            }
            let letter = temp.rooms[ridx][didx];
            temp.rooms[ridx][didx] = '.';
            temp.hallway[hidx] = letter;

            // Cost to move to hallway
            let mut cost = 1 + didx;
            // Cost to move along hallway
            cost += hallway_moves;
            temp.cost += cost * letter_value(letter);

            output.push(temp);

            // We're done
            if hidx == 0 {
                break;
            } else if hidx == 1 {
                hallway_moves += 1;
            } else {
                hallway_moves += 2;
            }
            cntr += 1;
            hidx = start - cntr;
        }
        output
    }

    fn moves_right(&self, ridx: usize) -> Vec<State2> {
        let mut output = vec![];
        // Bail immediately if nothing to move
        if self.rooms[ridx].iter().filter(|&&x| x != '.').count() == 0 {
            return output;
        }
        // Bail if room is in phase 2
        if self.room_in_phase_2(ridx) {
            return output;
        }
        let start = 2 + ridx;
        let mut cntr = 0;

        let mut hidx = start + cntr;
        let mut hallway_moves = 1;
        while self.hallway[hidx] == '.' {
            let mut temp = *self;
            let mut didx = 0;
            while temp.rooms[ridx][didx] == '.' {
                didx += 1;
            }
            let letter = temp.rooms[ridx][didx];
            temp.rooms[ridx][didx] = '.';
            temp.hallway[hidx] = letter;

            // Cost to move to hallway
            let mut cost = 1 + didx;
            // Cost to move along hallway
            cost += hallway_moves;
            temp.cost += cost * letter_value(letter);

            output.push(temp);

            // We're done
            if hidx == 6 {
                break;
            } else if hidx == 5 {
                hallway_moves += 1;
            } else {
                hallway_moves += 2;
            }
            cntr += 1;
            hidx = start + cntr;
        }
        output
    }

    fn room_in_phase_2(&self, ridx: usize) -> bool {
        let expected_letter = match ridx {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => panic!("Invalid room index"),
        };
        self.rooms[ridx]
            .iter()
            .filter(|&&r| r != expected_letter && r != '.')
            .count()
            == 0
    }

    fn hallway_to_room_moves(&self, ridx: usize) -> Vec<State2> {
        let mut output = vec![];

        // First check to see if this room is even ready yet
        if !self.room_in_phase_2(ridx) {
            return output;
        }

        // Room only admits certain letters if in phase 2
        let expected_letter = match ridx {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => panic!("Invalid room index"),
        };

        // Room is ready if we get here
        for hidx in 0..7 {
            // Make sure there's an appropriate letter here
            if self.hallway[hidx] != expected_letter {
                continue;
            }

            let mut valid = true;
            if ridx + 2 > hidx {
                // Moving to the right
                for i in hidx + 1..ridx + 2 {
                    if self.hallway[i] != '.' {
                        valid = false;
                    }
                }
            } else {
                // Moving to the left
                for i in ridx + 2..hidx {
                    if self.hallway[i] != '.' {
                        valid = false;
                    }
                }
            }

            if valid {
                // If we have a valid letter, and the path to the room is
                // unblocked, we can move
                let mut temp = *self;
                let letter = temp.hallway[hidx];
                temp.hallway[hidx] = '.';
                let mut didx = temp.rooms[ridx].len() - 1;
                while temp.rooms[ridx][didx] != '.' && didx > 0 {
                    didx -= 1;
                }
                temp.rooms[ridx][didx] = letter;
                // Cost to move to room
                let mut cost = 2 + didx;
                // Cost to move along hallway
                let mut moves = 0;
                if ridx + 2 > hidx {
                    // Moving to the right
                    if hidx == 0 {
                        moves += 1;
                        moves += 2 * (ridx - hidx);
                    } else {
                        moves = 2 * (ridx - (hidx - 1));
                    }
                } else {
                    // Moving to the left
                    if hidx == 6 {
                        moves += 1;
                        moves += 2 * (hidx - 3 - ridx);
                    } else {
                        moves = 2 * (hidx - 2 - ridx);
                    }
                }
                cost += moves;
                temp.cost += cost * letter_value(letter);

                output.push(temp);
            }
        }
        output
    }

    fn valid_moves(&self) -> Vec<State2> {
        let mut output = vec![];
        for ridx in 0..4 {
            // Now enumerate hallway -> room moves
            // NOTE: IF any of these are possible, they are always the best
            // move to make, so we won't even bother reporting the others
            let moves = self.hallway_to_room_moves(ridx);
            if !moves.is_empty() {
                for m in moves {
                    output.push(m);
                }
                return output;
            }
            // First enumerate room -> hallway moves to the left
            let moves = self.moves_left(ridx);
            for m in moves {
                output.push(m);
            }
            // Now enumerate room -> hallway moves to the right
            let moves = self.moves_right(ridx);
            for m in moves {
                output.push(m);
            }
        }
        output
    }
}

#[aoc(day23, part2)]
fn part2(input: &[char]) -> usize {
    let mut modified_input = input.to_vec();
    let new_letters = vec!['D', 'C', 'B', 'A', 'D', 'B', 'A', 'C'];
    for x in new_letters.iter().rev() {
        modified_input.insert(4, *x);
    }
    let mut game = Game2::from_letters(&modified_input);

    // Play all possible games
    while !game.states_to_play.is_empty() {
        game.step();
    }
    game.best_score
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/23.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 12521);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/23.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 44169);
    }
}

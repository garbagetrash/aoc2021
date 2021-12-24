use std::collections::HashMap;

#[aoc_generator(day21)]
fn load_input(input: &str) -> Vec<Player> {
    let mut output = vec![];
    for line in input.lines() {
        let mut liter = line.split(": ");
        liter.next();
        let pos = liter.next().unwrap().parse::<usize>().unwrap();
        output.push(Player::new(pos));
    }
    output
}

struct DeterministicDice {
    value: usize,
    n_rolls: usize,
}

impl DeterministicDice {
    fn new() -> DeterministicDice {
        DeterministicDice {
            value: 1,
            n_rolls: 0,
        }
    }
}

impl Iterator for DeterministicDice {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let output = self.value;
        self.value += 1;
        if self.value > 100 {
            self.value = 1;
        }
        self.n_rolls += 1;
        Some(self.value)
    }
}

#[derive(Clone, Copy, Debug)]
struct Player {
    score: usize,
    position: usize,
}

impl Player {
    fn new(position: usize) -> Player {
        Player {
            score: 0,
            position: position,
        }
    }
}

#[aoc(day21, part1)]
fn part1(input: &[Player]) -> usize {
    let mut dice = DeterministicDice::new();
    println!("{:?}", input[0]);
    println!("{:?}", input[1]);
    0
}

#[derive(Clone, Copy, Debug)]
struct State {
    score: usize,
    position: usize,
    nways: usize,
}

impl State {
    fn new(pos0: usize, key: &[usize]) -> State {
        let roll_nways: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];
        let mut score = 0;
        let mut position = pos0;
        let mut nways = 1;
        for s in key {
            position += s;
            if position > 10 {
                position -= 10;
            }
            score += position;
            nways *= roll_nways[s - 3];
        }
        State {
            score,
            position,
            nways,
        }
    }
}

fn next_state_map<const C: usize>(pos0: usize, last_key: &[usize]) -> HashMap<[usize; C], State> {
    let mut output = HashMap::new();

    let rolls: [usize; 7] = [3, 4, 5, 6, 7, 8, 9];
    let mut key = [0_usize; C];
    for i in 0..C - 1 {
        key[i] = last_key[i];
    }
    for roll in rolls {
        key[C - 1] = roll;
        let level = State::new(pos0, &key);
        output.insert(key, level);
    }
    output
}

#[aoc(day21, part2)]
fn part2(input: &[Player]) -> usize {
    let rolls: [usize; 7] = [3, 4, 5, 6, 7, 8, 9];

    /*
    let mut player1_states = vec![];
    for i in 1..11 {
        let mut map: HashMap<[usize; i], State> = HashMap::new();
        player1_states.push(map);
    }
    */

    let mut state1: HashMap<[usize; 1], State> = HashMap::new();
    let mut state2: HashMap<[usize; 2], State> = HashMap::new();
    let mut state3: HashMap<[usize; 3], State> = HashMap::new();
    let mut state4: HashMap<[usize; 4], State> = HashMap::new();
    let mut state5: HashMap<[usize; 5], State> = HashMap::new();
    let mut state6: HashMap<[usize; 6], State> = HashMap::new();
    let mut state7: HashMap<[usize; 7], State> = HashMap::new();
    let mut state8: HashMap<[usize; 8], State> = HashMap::new();
    let mut state9: HashMap<[usize; 9], State> = HashMap::new();
    let mut state10: HashMap<[usize; 10], State> = HashMap::new();

    let pos0 = input[0].position;

    /*
    let mut maps = vec![];
    let new_map = next_state_map::<1>(pos0, &[]);
    maps.push(new_map.clone());

    for (key, state) in new_map {
        if state.score < 21 {
            let new_map = next_state_map::<2>(pos0, &key);
        }
    }
    */

    for roll1 in rolls {
        let key = [roll1];
        let level1 = State::new(pos0, &key);
        state1.insert(key, level1);

        if level1.score < 21 {
            for roll2 in rolls {
                let key = [roll1, roll2];
                let level2 = State::new(pos0, &key);
                state2.insert(key, level2);

                if level2.score < 21 {
                    for roll3 in rolls {
                        let key = [roll1, roll2, roll3];
                        let level3 = State::new(pos0, &key);
                        state3.insert(key, level3);

                        if level3.score < 21 {
                            for roll4 in rolls {
                                let key = [roll1, roll2, roll3, roll4];
                                let level4 = State::new(pos0, &key);
                                state4.insert(key, level4);

                                if level4.score < 21 {
                                    for roll5 in rolls {
                                        let key = [roll1, roll2, roll3, roll4, roll5];
                                        let level5 = State::new(pos0, &key);
                                        state5.insert(key, level5);

                                        if level5.score < 21 {
                                            for roll6 in rolls {
                                                let key = [roll1, roll2, roll3, roll4, roll5, roll6];
                                                let level6 = State::new(pos0, &key);
                                                state6.insert(key, level6);

                                                if level6.score < 21 {
                                                    for roll7 in rolls {
                                                        let key = [roll1, roll2, roll3, roll4, roll5, roll6, roll7];
                                                        let level7 = State::new(pos0, &key);
                                                        state7.insert(key, level7);

                                                        if level7.score < 21 {
                                                            for roll8 in rolls {
                                                                let key = [roll1, roll2, roll3, roll4, roll5, roll6, roll7, roll8];
                                                                let level8 = State::new(pos0, &key);
                                                                state8.insert(key, level8);

                                                                if level8.score < 21 {
                                                                    for roll9 in rolls {
                                                                        let key = [roll1, roll2, roll3, roll4, roll5, roll6, roll7, roll8, roll9];
                                                                        let level9 = State::new(pos0, &key);
                                                                        state9.insert(key, level9);

                                                                        if level9.score < 21 {
                                                                            for roll10 in rolls {
                                                                                let key = [roll1, roll2, roll3, roll4, roll5, roll6, roll7, roll8, roll9, roll10];
                                                                                let level10 = State::new(pos0, &key);
                                                                                state10.insert(key, level10);

                                                                                if level10.score < 21 {
                                                                                    panic!("WTF mate");
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", state1.len());
    println!("{}", state2.len());
    println!("{}", state3.len());
    println!("{}", state4.len());
    println!("{}", state5.len());
    println!("{}", state6.len());
    println!("{}", state7.len());
    println!("{}", state8.len());
    println!("{}", state9.len());
    println!("{}", state10.len());

    let ways_to_lose_2: usize = state2.values().filter(|s| s.score < 21).map(|s| s.nways).sum();

    // Wins start happening in 3 steps, stop happening after 10th step
    let ways_to_win_3: usize = state3.values().filter(|s| s.score > 20).map(|s| s.nways).sum();
    let ways_to_lose_3: usize = state3.values().filter(|s| s.score < 21).map(|s| s.nways).sum();

    let ways_to_win_4: usize = state4.values().filter(|s| s.score > 20).map(|s| s.nways).sum();
    let ways_to_lose_4: usize = state4.values().filter(|s| s.score < 21).map(|s| s.nways).sum();

    let ways_to_win_5: usize = state5.values().filter(|s| s.score > 20).map(|s| s.nways).sum();
    let ways_to_lose_5: usize = state5.values().filter(|s| s.score < 21).map(|s| s.nways).sum();

    let ways_to_win_6: usize = state6.values().filter(|s| s.score > 20).map(|s| s.nways).sum();
    let ways_to_lose_6: usize = state6.values().filter(|s| s.score < 21).map(|s| s.nways).sum();

    let ways_to_win_7: usize = state7.values().filter(|s| s.score > 20).map(|s| s.nways).sum();
    let ways_to_lose_7: usize = state7.values().filter(|s| s.score < 21).map(|s| s.nways).sum();

    let ways_to_win_8: usize = state8.values().filter(|s| s.score > 20).map(|s| s.nways).sum();
    let ways_to_lose_8: usize = state8.values().filter(|s| s.score < 21).map(|s| s.nways).sum();

    let ways_to_win_9: usize = state9.values().filter(|s| s.score > 20).map(|s| s.nways).sum();
    let ways_to_lose_9: usize = state9.values().filter(|s| s.score < 21).map(|s| s.nways).sum();

    let ways_to_win_10: usize = state10.values().filter(|s| s.score > 20).map(|s| s.nways).sum();
    let ways_to_lose_10: usize = state10.values().filter(|s| s.score < 21).map(|s| s.nways).sum();
    assert_eq!(ways_to_lose_10, 0);

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    p1_wins += ways_to_win_3 * ways_to_lose_2;

    println!("Player 1 ways to win in 3: {}", p1_wins);

    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/21.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/21.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}

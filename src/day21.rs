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
        Some(output)
    }
}

#[derive(Clone, Copy, Debug)]
struct Player {
    score: usize,
    position: usize,
}

impl Player {
    fn new(position: usize) -> Player {
        Player { score: 0, position }
    }
}

#[aoc(day21, part1)]
fn part1(input: &[Player]) -> usize {
    let mut dice = DeterministicDice::new();
    let mut p1 = input[0];
    let mut p2 = input[1];

    loop {
        let r1 = dice.next().unwrap();
        let r2 = dice.next().unwrap();
        let r3 = dice.next().unwrap();
        let r = r1 + r2 + r3;
        p1.position += r;
        while p1.position > 10 {
            p1.position -= 10;
        }
        p1.score += p1.position;
        if p1.score >= 1000 {
            return p2.score * dice.n_rolls;
        }

        let r1 = dice.next().unwrap();
        let r2 = dice.next().unwrap();
        let r3 = dice.next().unwrap();
        let r = r1 + r2 + r3;
        p2.position += r;
        while p2.position > 10 {
            p2.position -= 10;
        }
        p2.score += p2.position;
        if p2.score >= 1000 {
            return p1.score * dice.n_rolls;
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    score: usize,
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
        State { score, nways }
    }
}

#[allow(clippy::type_complexity)]
fn initialize_maps() -> (
    HashMap<[usize; 1], State>,
    HashMap<[usize; 2], State>,
    HashMap<[usize; 3], State>,
    HashMap<[usize; 4], State>,
    HashMap<[usize; 5], State>,
    HashMap<[usize; 6], State>,
    HashMap<[usize; 7], State>,
    HashMap<[usize; 8], State>,
    HashMap<[usize; 9], State>,
    HashMap<[usize; 10], State>,
) {
    let state1: HashMap<[usize; 1], State> = HashMap::new();
    let state2: HashMap<[usize; 2], State> = HashMap::new();
    let state3: HashMap<[usize; 3], State> = HashMap::new();
    let state4: HashMap<[usize; 4], State> = HashMap::new();
    let state5: HashMap<[usize; 5], State> = HashMap::new();
    let state6: HashMap<[usize; 6], State> = HashMap::new();
    let state7: HashMap<[usize; 7], State> = HashMap::new();
    let state8: HashMap<[usize; 8], State> = HashMap::new();
    let state9: HashMap<[usize; 9], State> = HashMap::new();
    let state10: HashMap<[usize; 10], State> = HashMap::new();

    (
        state1, state2, state3, state4, state5, state6, state7, state8, state9, state10,
    )
}

#[allow(clippy::type_complexity)]
fn the_dirty_work(
    maps: &mut (
        HashMap<[usize; 1], State>,
        HashMap<[usize; 2], State>,
        HashMap<[usize; 3], State>,
        HashMap<[usize; 4], State>,
        HashMap<[usize; 5], State>,
        HashMap<[usize; 6], State>,
        HashMap<[usize; 7], State>,
        HashMap<[usize; 8], State>,
        HashMap<[usize; 9], State>,
        HashMap<[usize; 10], State>,
    ),
    pos0: usize,
) {
    let rolls: [usize; 7] = [3, 4, 5, 6, 7, 8, 9];
    for roll1 in rolls {
        let key = [roll1];
        let level1 = State::new(pos0, &key);
        maps.0.insert(key, level1);

        if level1.score < 21 {
            for roll2 in rolls {
                let key = [roll1, roll2];
                let level2 = State::new(pos0, &key);
                maps.1.insert(key, level2);

                if level2.score < 21 {
                    for roll3 in rolls {
                        let key = [roll1, roll2, roll3];
                        let level3 = State::new(pos0, &key);
                        maps.2.insert(key, level3);

                        if level3.score < 21 {
                            for roll4 in rolls {
                                let key = [roll1, roll2, roll3, roll4];
                                let level4 = State::new(pos0, &key);
                                maps.3.insert(key, level4);

                                if level4.score < 21 {
                                    for roll5 in rolls {
                                        let key = [roll1, roll2, roll3, roll4, roll5];
                                        let level5 = State::new(pos0, &key);
                                        maps.4.insert(key, level5);

                                        if level5.score < 21 {
                                            for roll6 in rolls {
                                                let key =
                                                    [roll1, roll2, roll3, roll4, roll5, roll6];
                                                let level6 = State::new(pos0, &key);
                                                maps.5.insert(key, level6);

                                                if level6.score < 21 {
                                                    for roll7 in rolls {
                                                        let key = [
                                                            roll1, roll2, roll3, roll4, roll5,
                                                            roll6, roll7,
                                                        ];
                                                        let level7 = State::new(pos0, &key);
                                                        maps.6.insert(key, level7);

                                                        if level7.score < 21 {
                                                            for roll8 in rolls {
                                                                let key = [
                                                                    roll1, roll2, roll3, roll4,
                                                                    roll5, roll6, roll7, roll8,
                                                                ];
                                                                let level8 = State::new(pos0, &key);
                                                                maps.7.insert(key, level8);

                                                                if level8.score < 21 {
                                                                    for roll9 in rolls {
                                                                        let key = [
                                                                            roll1, roll2, roll3,
                                                                            roll4, roll5, roll6,
                                                                            roll7, roll8, roll9,
                                                                        ];
                                                                        let level9 =
                                                                            State::new(pos0, &key);
                                                                        maps.8.insert(key, level9);

                                                                        if level9.score < 21 {
                                                                            for roll10 in rolls {
                                                                                let key = [
                                                                                    roll1, roll2,
                                                                                    roll3, roll4,
                                                                                    roll5, roll6,
                                                                                    roll7, roll8,
                                                                                    roll9, roll10,
                                                                                ];
                                                                                let level10 =
                                                                                    State::new(
                                                                                        pos0, &key,
                                                                                    );
                                                                                maps.9.insert(
                                                                                    key, level10,
                                                                                );

                                                                                if level10.score
                                                                                    < 21
                                                                                {
                                                                                    panic!(
                                                                                        "WTF mate"
                                                                                    );
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
}

#[aoc(day21, part2)]
fn part2(input: &[Player]) -> usize {
    let pos0 = input[0].position;
    let pos1 = input[1].position;

    let mut maps_p1 = initialize_maps();
    let mut maps_p2 = initialize_maps();

    the_dirty_work(&mut maps_p1, pos0);
    the_dirty_work(&mut maps_p2, pos1);

    let ways_to_lose_2: usize = maps_p2
        .1
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    // Wins start happening in 3 steps, stop happening after 10th step
    let ways_to_win_3: usize = maps_p1
        .2
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_3: usize = maps_p2
        .2
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_4: usize = maps_p1
        .3
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_4: usize = maps_p2
        .3
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_5: usize = maps_p1
        .4
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_5: usize = maps_p2
        .4
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_6: usize = maps_p1
        .5
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_6: usize = maps_p2
        .5
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_7: usize = maps_p1
        .6
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_7: usize = maps_p2
        .6
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_8: usize = maps_p1
        .7
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_8: usize = maps_p2
        .7
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_9: usize = maps_p1
        .8
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_9: usize = maps_p2
        .8
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_10: usize = maps_p1
        .9
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();

    let mut p1_wins = 0;
    p1_wins += ways_to_win_3 * ways_to_lose_2;
    p1_wins += ways_to_win_4 * ways_to_lose_3;
    p1_wins += ways_to_win_5 * ways_to_lose_4;
    p1_wins += ways_to_win_6 * ways_to_lose_5;
    p1_wins += ways_to_win_7 * ways_to_lose_6;
    p1_wins += ways_to_win_8 * ways_to_lose_7;
    p1_wins += ways_to_win_9 * ways_to_lose_8;
    p1_wins += ways_to_win_10 * ways_to_lose_9;

    // Wins start happening in 3 steps, stop happening after 10th step
    let ways_to_win_3: usize = maps_p2
        .2
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_3: usize = maps_p1
        .2
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_4: usize = maps_p2
        .3
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_4: usize = maps_p1
        .3
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_5: usize = maps_p2
        .4
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_5: usize = maps_p1
        .4
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_6: usize = maps_p2
        .5
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_6: usize = maps_p1
        .5
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_7: usize = maps_p2
        .6
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_7: usize = maps_p1
        .6
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_8: usize = maps_p2
        .7
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_8: usize = maps_p1
        .7
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_9: usize = maps_p2
        .8
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_9: usize = maps_p1
        .8
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let ways_to_win_10: usize = maps_p2
        .9
        .values()
        .filter(|s| s.score > 20)
        .map(|s| s.nways)
        .sum();
    let ways_to_lose_10: usize = maps_p1
        .9
        .values()
        .filter(|s| s.score < 21)
        .map(|s| s.nways)
        .sum();

    let mut p2_wins = 0;
    p2_wins += ways_to_win_3 * ways_to_lose_3;
    p2_wins += ways_to_win_4 * ways_to_lose_4;
    p2_wins += ways_to_win_5 * ways_to_lose_5;
    p2_wins += ways_to_win_6 * ways_to_lose_6;
    p2_wins += ways_to_win_7 * ways_to_lose_7;
    p2_wins += ways_to_win_8 * ways_to_lose_8;
    p2_wins += ways_to_win_9 * ways_to_lose_9;
    p2_wins += ways_to_win_10 * ways_to_lose_10;

    if p1_wins > p2_wins {
        p1_wins
    } else {
        p2_wins
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/21.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 739785);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/21.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 444356092776315);
    }
}

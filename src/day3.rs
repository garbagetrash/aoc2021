#[aoc_generator(day3)]
pub fn load_input(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect()
}

pub fn gamma_rate(input: &[String]) -> Vec<i64> {
    let N = input[0].len();
    let mut gamma = vec![0_i64; N];
    for line in input {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                gamma[i] += 1;
            } else {
                // '0'
                gamma[i] -= 1;
            }
        }
    }

    for v in gamma.iter_mut() {
        if *v > 0 {
            *v = 1;
        } else if *v < 0 {
            *v = 0;
        } else {
            *v = -1;
        }
    }
    // gamma contains most common bit value in position at this point
    gamma
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> i64 {
    let gamma = gamma_rate(&input.to_vec());
    let epsilon = gamma.iter().map(|x| x ^ 1).collect();

    to_int(&gamma) * to_int(&epsilon)
}

pub fn to_int(input: &Vec<i64>) -> i64 {
    let mut s = String::new();
    for v in input {
        if *v == 0 {
            s.push('0');
        } else {
            s.push('1');
        }
    }
    println!("str: {}", s);
    let output = i64::from_str_radix(&s, 2).unwrap();
    println!("output: {}", output);
    output
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> i64 {
    let N = input[0].len();

    let mut cols = vec![];
    let mut total_pool: Vec<String> = input.to_vec();
    for bit_pos in 0..N {
        let mut pool: Vec<String> = vec![];
        cols = gamma_rate(&total_pool);
        for line in total_pool {
            let c = line.chars().nth(bit_pos).unwrap();
            if cols[bit_pos] == -1 {
                if c == '1' {
                    pool.push(line.clone());
                }
            } else {
                if c == char::from_digit(cols[bit_pos] as u32, 2).unwrap() {
                    pool.push(line.clone());
                }
            }
        }
        total_pool = pool;
        if total_pool.len() == 1 {
            break;
        }
    }
    let oxy = total_pool[0].clone();
    println!("oxy: {:?}", oxy);

    total_pool = input.to_vec();
    for bit_pos in 0..N {
        let mut pool: Vec<String> = vec![];
        cols = gamma_rate(&total_pool);
        for line in total_pool {
            let c = line.chars().nth(bit_pos).unwrap();
            if cols[bit_pos] == -1 {
                if c == '0' {
                    pool.push(line.clone());
                }
            } else {
                if c != char::from_digit(cols[bit_pos] as u32, 2).unwrap() {
                    pool.push(line.clone());
                }
            }
        }
        total_pool = pool;
        if total_pool.len() == 1 {
            break;
        }
    }
    let co2 = total_pool[0].clone();
    println!("co2: {:?}", co2);
    i64::from_str_radix(&co2, 2).unwrap() * i64::from_str_radix(&oxy, 2).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/03.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/03.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 230);
    }
}

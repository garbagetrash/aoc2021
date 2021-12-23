use std::collections::HashMap;

#[aoc_generator(day20)]
fn load_input(input: &str) -> (Vec<u8>, HashMap<(i32, i32), u8>) {
    let mut cntr = 0;
    let mut key = vec![];
    for line in input.lines().take_while(|line| !line.is_empty()) {
        for c in line.chars() {
            if c == '.' {
                key.push(0);
            } else if c == '#' {
                key.push(1);
            } else {
                panic!("Invalid key character");
            }
        }
        cntr += 1;
    }

    let mut image = HashMap::new();
    for (y, line) in input.lines().skip(cntr + 1).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((x as i32, y as i32), 1);
            } else if c == '.' {
                image.insert((x as i32, y as i32), 0);
            } else {
                panic!("Invalid input!");
            }
        }
    }

    (key, image)
}

// Get the surrounding point indexes
fn get_neighbors(point: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (point.0 - 1, point.1 - 1),
        (point.0, point.1 - 1),
        (point.0 + 1, point.1 - 1),
        (point.0 - 1, point.1),
        (point.0, point.1),
        (point.0 + 1, point.1),
        (point.0 - 1, point.1 + 1),
        (point.0, point.1 + 1),
        (point.0 + 1, point.1 + 1),
    ]
}

fn code_to_number(code: &[u8]) -> usize {
    let mut output = code[0] as usize;
    for c in code.to_vec().iter().skip(1) {
        output <<= 1;
        output += *c as usize;
    }
    output
}

fn enhance_pixel(pixel: (i32, i32), key: &[u8], map: &HashMap<(i32, i32), u8>, toggle: u8) -> u8 {
    let idxs = get_neighbors(pixel);
    let mut code = vec![];
    for i in idxs {
        if let Some(value) = map.get(&i) {
            code.push(*value);
        } else {
            code.push(toggle);
        }
    }
    let number = code_to_number(&code);
    key[number]
}

#[allow(dead_code)]
fn print_board(map: &HashMap<(i32, i32), u8>) {
    for y in -40..50 {
        for x in -40..50 {
            if let Some(value) = map.get(&(x, y)) {
                if *value == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    println!();
    println!();
}

fn add_border(map: &mut HashMap<(i32, i32), u8>, n_times: i32) {
    // First let's insert a border of darkness around the existing image
    let upper_left = (0, 0);
    let mut lower_right = (0, 0);
    let mut prior_sum = 0;
    for pixel in map.keys() {
        if pixel.0 + pixel.1 > prior_sum {
            prior_sum = pixel.0 + pixel.1;
            lower_right = *pixel;
        }
    }

    let n = n_times + 1;
    for x in upper_left.0 - n..lower_right.0 + n {
        for k in 1..n + 1 {
            map.insert((x, upper_left.1 - k), 0);
            map.insert((x, lower_right.1 + k), 0);
        }
    }

    for y in upper_left.1 - n..lower_right.1 + n {
        for k in 1..n + 1 {
            map.insert((upper_left.0 - k, y), 0);
            map.insert((lower_right.0 + k, y), 0);
        }
    }
}

#[aoc(day20, part1)]
fn part1(input: &(Vec<u8>, HashMap<(i32, i32), u8>)) -> usize {
    let mut old_image = input.1.clone();
    let mut new_image = HashMap::new();

    add_border(&mut old_image, 2);

    let mut toggling = true;
    if input.0[0] == 0 {
        toggling = false;
    }

    // Now iterate on stuff in the map
    let mut toggle = 0;
    for _ in 0..2 {
        //print_board(&old_image);
        for pixel in old_image.keys() {
            let new_pixel = enhance_pixel(*pixel, &input.0, &old_image, toggle);
            new_image.insert(*pixel, new_pixel);
        }
        old_image = new_image;
        new_image = HashMap::new();

        if toggling {
            toggle ^= 1;
        }
    }

    // Count up the '#'s in the new image
    //print_board(&old_image);
    let mut answer = 0;
    for value in old_image.values() {
        if *value == 1 {
            answer += 1;
        }
    }
    answer
}

#[aoc(day20, part2)]
fn part2(input: &(Vec<u8>, HashMap<(i32, i32), u8>)) -> usize {
    let mut old_image = input.1.clone();
    let mut new_image = HashMap::new();

    add_border(&mut old_image, 50);

    let mut toggling = true;
    if input.0[0] == 0 {
        toggling = false;
    }

    // Now iterate on stuff in the map
    let mut toggle = 0;
    for _ in 0..50 {
        //print_board(&old_image);
        for pixel in old_image.keys() {
            let new_pixel = enhance_pixel(*pixel, &input.0, &old_image, toggle);
            new_image.insert(*pixel, new_pixel);
        }
        old_image = new_image;
        new_image = HashMap::new();

        if toggling {
            toggle ^= 1;
        }
    }

    // Count up the '#'s in the new image
    //print_board(&old_image);
    let mut answer = 0;
    for value in old_image.values() {
        if *value == 1 {
            answer += 1;
        }
    }
    answer
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/20.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/20.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 3351);
    }
}

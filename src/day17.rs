#[derive(Debug)]
pub struct Rectangle((i32, i32), (i32, i32));

#[aoc_generator(day17)]
pub fn load_input(input: &str) -> Rectangle {
    let mut segments = input.split('=');
    segments.next();
    let xpart = segments.next().unwrap();
    let ypart = segments.next().unwrap();
    let mut xvals = xpart.split("..");
    let mut yvals = ypart.split("..");
    let xmin = xvals.next().unwrap();
    let xmax = xvals.next().unwrap();
    let mut xmax = xmax.split(',');
    let xmax = xmax.next().unwrap();
    let ymin = yvals.next().unwrap();
    let ymax = yvals.next().unwrap();
    let mut ymax = ymax.split(',');
    let ymax = ymax.next().unwrap().split_whitespace().next().unwrap();

    Rectangle(
        (xmin.parse::<i32>().unwrap(), xmax.parse::<i32>().unwrap()),
        (ymin.parse::<i32>().unwrap(), ymax.parse::<i32>().unwrap()),
    )
}

#[allow(clippy::comparison_chain)]
fn valid_check(v: (i32, i32), xrange: (i32, i32), yrange: (i32, i32)) -> bool {
    let mut p = (0, 0);
    let mut v_ = v;
    loop {
        p.0 += v_.0;
        p.1 += v_.1;

        if v_.0 > 0 {
            v_.0 -= 1;
        } else if v_.0 < 0 {
            v_.0 += 1;
        }
        v_.1 -= 1;

        if xrange.0 <= p.0 && p.0 <= xrange.1 && yrange.0 <= p.1 && p.1 <= yrange.1 {
            return true;
        }

        if p.1 < yrange.0 {
            return false;
        }
    }
}

fn max_height(vy: i32) -> i32 {
    let mut y = 0;
    let mut vy = vy;
    while vy > 0 {
        y += vy;
        vy -= 1;
    }
    y
}

#[aoc(day17, part1)]
pub fn part1(input: &Rectangle) -> i32 {
    // Max possible vy = abs(input.1.0) - 1
    let max_vy = input.1 .0.abs() - 1;
    max_height(max_vy)
}

#[aoc(day17, part2)]
pub fn part2(input: &Rectangle) -> usize {
    let max_vy = input.1 .0.abs() - 1;
    let mut count = 0;
    for vx in 0..input.0 .1 + 1 {
        for vy in input.1 .0..max_vy + 1 {
            if valid_check((vx, vy), input.0, input.1) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/17.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 45);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/17.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 112);
    }
}

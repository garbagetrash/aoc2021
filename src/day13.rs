use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Fold {
    pub orientation: char,
    pub number: usize,
}

#[aoc_generator(day13)]
pub fn load_input(input: &str) -> (HashMap<(usize, usize), u8>, Vec<Fold>) {
    let mut folds = vec![];
    let mut output = HashMap::<(usize, usize), u8>::new();
    for line in input.lines() {
        if line.is_empty() {
        } else if line.contains("fold along") {
            let mut ll = line.split(' ');
            ll.next();
            ll.next();
            let eq = ll.next().unwrap();
            let mut ee = eq.split('=');
            let orient = ee.next().unwrap().chars().next().unwrap();
            let number = ee.next().unwrap().parse::<usize>().unwrap();
            folds.push(Fold {
                orientation: orient,
                number: number,
            });
        } else {
            let mut ll = line.split(',');
            let x = ll.next().unwrap().parse::<usize>().unwrap();
            let y = ll.next().unwrap().parse::<usize>().unwrap();
            output.insert((x, y), 1);
        }
    }

    (output, folds)
}

pub fn get_max(board: &HashMap<(usize, usize), u8>) -> usize {
    let mut maxidx = 0;
    for idxs in board.keys() {
        if idxs.0 > maxidx {
            maxidx = idxs.0;
        }
        if idxs.1 > maxidx {
            maxidx = idxs.1;
        }
    }
    maxidx
}

pub fn fold_point(point: &(usize, usize), fold: &Fold) -> (usize, usize) {
    if fold.orientation == 'x' {
        // Vertical fold to the left along x=fold:number
        let xplus = point.0 - fold.number;
        let xminus = fold.number - xplus;
        return (xminus, point.1);
    } else {
        // Horizontal fold up along y=fold:number
        let yplus = point.1 - fold.number;
        let yminus = fold.number - yplus;
        return (point.0, yminus);
    }
}

pub fn do_fold(board: &mut HashMap<(usize, usize), u8>, fold: &Fold) {
    let keys: Vec<_> = board.keys().copied().collect();
    for point in keys {
        if fold.orientation == 'x' {
            if point.0 > fold.number {
                let new_point = fold_point(&point, &fold);
                board.remove(&point);
                board.insert(new_point, 1);
            }
        } else {
            if point.1 > fold.number {
                let new_point = fold_point(&point, &fold);
                board.remove(&point);
                board.insert(new_point, 1);
            }
        }
    }
}

pub fn count_dots(board: &HashMap<(usize, usize), u8>) -> usize {
    let mut output = 0;
    for v in board.values() {
        if *v > 0 {
            output += 1;
        }
    }
    output
}

pub fn print_paper(board: &HashMap<(usize, usize), u8>) {
    for y in 0..10 {
        for x in 0..64 {
            if board.get(&(x, y)).is_some() {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &(HashMap<(usize, usize), u8>, Vec<Fold>)) -> usize {
    let mut board = input.0.clone();
    let fold = input.1[0].clone();
    do_fold(&mut board, &fold);
    count_dots(&board)
}

#[aoc(day13, part2)]
pub fn part2(input: &(HashMap<(usize, usize), u8>, Vec<Fold>)) -> usize {
    let mut board = input.0.clone();
    let folds = input.1.clone();
    for fold in &folds {
        do_fold(&mut board, fold);
    }

    print_paper(&board);
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/13.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 17);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/13.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}

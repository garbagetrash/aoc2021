use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cube {
    xvalues: (i64, i64),
    yvalues: (i64, i64),
    zvalues: (i64, i64),
}

impl Cube {
    fn new(xvalues: (i64, i64), yvalues: (i64, i64), zvalues: (i64, i64)) -> Cube {
        Cube {
            xvalues,
            yvalues,
            zvalues,
        }
    }

    fn volume(&self) -> usize {
        ((self.xvalues.1 - self.xvalues.0)
            * (self.yvalues.1 - self.yvalues.0)
            * (self.zvalues.1 - self.zvalues.0)) as usize
    }

    fn intersect(&self, other: Cube) -> Option<Cube> {
        if let Some(xint) = intersect_1d(self.xvalues, other.xvalues) {
            if let Some(yint) = intersect_1d(self.yvalues, other.yvalues) {
                intersect_1d(self.zvalues, other.zvalues).map(|zint| Cube::new(xint, yint, zint))
            } else {
                None
            }
        } else {
            None
        }
    }
}

// Partitioning a 1d range with its intersection can result in up to 3 ranges.
fn partition_1d(range1: (i64, i64), intersection: (i64, i64)) -> Vec<(i64, i64)> {
    let mut output = vec![];
    if range1.0 < intersection.0 {
        output.push((range1.0, intersection.0));
    }
    output.push(intersection);
    if range1.1 > intersection.1 {
        output.push((intersection.1, range1.1));
    }
    output
}

// When breaking up cube, smaller cubes can occupy 3x3x3 possible spaces, so up
// to 27 cubes result
fn partition_3d(cube: Cube, intersection: Cube) -> Vec<Cube> {
    let xparts = partition_1d(cube.xvalues, intersection.xvalues);
    let yparts = partition_1d(cube.yvalues, intersection.yvalues);
    let zparts = partition_1d(cube.zvalues, intersection.zvalues);

    let mut output = vec![];
    for x in &xparts {
        for y in &yparts {
            for z in &zparts {
                output.push(Cube::new(*x, *y, *z));
            }
        }
    }
    output
}

fn cube_sum(cube1: Cube, cube2: Cube) -> Vec<Cube> {
    let mut cubes = vec![cube1];
    if let Some(intersection) = cube1.intersect(cube2) {
        // Partition space into non overlapping cubes. Keep cube1 the same,
        // only breaking up cube2.
        let parts = partition_3d(cube2, intersection);
        for c in parts {
            if c != intersection {
                cubes.push(c);
            }
        }
    } else {
        // No intersection, just add cube2 to the set.
        cubes.push(cube2);
    }
    cubes
}

fn add_cube_to_set(cube: Cube, cubeset: &mut HashSet<Cube>) {
    let mut cubes_to_update = vec![];
    for sub_cube in cubeset.iter() {
        if cube.intersect(*sub_cube).is_some() {
            cubes_to_update.push(sub_cube);
        }
    }

    // Now that we know which cubes intersect our new one, break him up and add
    // all the sub cubes to cubeset.
    let mut cubes_to_add = vec![cube];
    while let Some(sub_cube) = cubes_to_update.pop() {
        let mut cubes_to_add_next = vec![];
        for add_cube in cubes_to_add {
            let cubes = cube_sum(*sub_cube, add_cube);
            let cubes: Vec<_> = cubes.iter().skip(1).collect();
            for &c in cubes {
                cubes_to_add_next.push(c);
            }
        }
        cubes_to_add = cubes_to_add_next;
    }

    for add_cube in cubes_to_add {
        cubeset.insert(add_cube);
    }
}

// cube1 - cube2
fn cube_difference(cube1: Cube, cube2: Cube) -> Vec<Cube> {
    let mut cubes = vec![];
    if let Some(intersection) = cube1.intersect(cube2) {
        // Partition space into non overlapping cubes
        let parts = partition_3d(cube1, intersection);
        for c in parts {
            if c != intersection {
                cubes.push(c);
            }
        }
    } else {
        cubes.push(cube1);
    }
    cubes
}

fn remove_cube_from_set(cube: Cube, cubeset: &mut HashSet<Cube>) {
    let mut cubes_to_update = vec![];
    for sub_cube in cubeset.iter() {
        if cube.intersect(*sub_cube).is_some() {
            cubes_to_update.push(*sub_cube);
        }
    }

    for sub_cube in cubes_to_update {
        // Remove affected cube
        cubeset.remove(&sub_cube);

        // Grab the difference...
        let intersection = cube.intersect(sub_cube).unwrap();
        let cubes = cube_difference(sub_cube, intersection);

        // Add the difference back in
        for c in cubes {
            cubeset.insert(c);
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    turn_on: bool,
    cube: Cube,
}

impl Instruction {
    fn new(
        turn_on: bool,
        xvalues: (i64, i64),
        yvalues: (i64, i64),
        zvalues: (i64, i64),
    ) -> Instruction {
        Instruction {
            turn_on,
            cube: Cube::new(xvalues, yvalues, zvalues),
        }
    }
}

#[aoc_generator(day22)]
fn load_input(input: &str) -> Vec<Instruction> {
    let mut output = vec![];
    for line in input.lines() {
        let mut liter = line.split(' ');

        let turn_on;
        if liter.next().unwrap() == "on" {
            turn_on = true;
        } else {
            turn_on = false;
        }

        let mut liter = line.split('=');
        liter.next();
        let xstring = liter.next().unwrap();
        let ystring = liter.next().unwrap();
        let zstring = liter.next().unwrap();

        let mut xiter = xstring.split("..");
        let xmin = xiter.next().unwrap().parse::<i64>().unwrap();
        let xmax = xiter
            .next()
            .unwrap()
            .split(',')
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let mut yiter = ystring.split("..");
        let ymin = yiter.next().unwrap().parse::<i64>().unwrap();
        let ymax = yiter
            .next()
            .unwrap()
            .split(',')
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let mut ziter = zstring.split("..");
        let zmin = ziter.next().unwrap().parse::<i64>().unwrap();
        let zmax = ziter
            .next()
            .unwrap()
            .split(',')
            .next()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        output.push(Instruction::new(
            turn_on,
            (xmin, xmax + 1),
            (ymin, ymax + 1),
            (zmin, zmax + 1),
        ));
    }
    output
}

#[aoc(day22, part1)]
fn part1(input: &[Instruction]) -> usize {
    let mut on_set: HashSet<Cube> = HashSet::new();

    for inst in input.iter().take(20) {
        if inst.turn_on {
            add_cube_to_set(inst.cube, &mut on_set);
        } else {
            remove_cube_from_set(inst.cube, &mut on_set);
        }
    }

    // Because the above partitions the space into non-overlapping cubes for us
    // we just sum their volumes here
    let mut output = 0;
    for cube in on_set {
        output += cube.volume();
    }
    output
}

fn intersect_1d(range1: (i64, i64), range2: (i64, i64)) -> Option<(i64, i64)> {
    let mut min = range1.0;
    if min < range2.0 {
        min = range2.0;
    }
    let mut max = range1.1;
    if max > range2.1 {
        max = range2.1;
    }
    if min >= max {
        None
    } else {
        Some((min, max))
    }
}

#[aoc(day22, part2)]
fn part2(input: &[Instruction]) -> usize {
    let mut on_set: HashSet<Cube> = HashSet::new();

    for inst in input {
        if inst.turn_on {
            add_cube_to_set(inst.cube, &mut on_set);
        } else {
            remove_cube_from_set(inst.cube, &mut on_set);
        }
    }

    // Because the above partitions the space into non-overlapping cubes for us
    // we just sum their volumes here
    let mut output = 0;
    for cube in on_set {
        output += cube.volume();
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/22a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 590784);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/22b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 2758514936282235);
    }
}

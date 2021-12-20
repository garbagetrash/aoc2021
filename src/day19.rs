use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Scanner {
    beacons: HashSet<(i32, i32, i32)>,
}

#[aoc_generator(day19)]
fn load_input(input: &str) -> Vec<Scanner> {
    let mut output = vec![];
    let mut cntr = 0;
    for line in input.lines() {
        if line.contains("scanner") {
            // New Scanner
            output.push(Scanner {
                beacons: HashSet::new(),
            });
        } else if line.contains(',') {
            // If position
            let mut liter = line.split(',');
            let n1 = liter.next().unwrap().parse::<i32>().unwrap();
            let n2 = liter.next().unwrap().parse::<i32>().unwrap();
            let n3 = liter.next().unwrap().parse::<i32>().unwrap();
            let pos = (n1, n2, n3);
            output[cntr].beacons.insert(pos);
        } else if line.is_empty() {
            cntr += 1;
        }
    }
    output
}

// List the 24 orientations of this given one of them
fn orientations(coords: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![
        // In->out (looking at XY plane)
        (coords.0, coords.1, coords.2),
        (coords.1, -coords.0, coords.2),
        (-coords.0, -coords.1, coords.2),
        (-coords.1, coords.0, coords.2),
        // Out->in (looking at -XY plane)
        (coords.0, -coords.1, -coords.2),
        (coords.1, coords.0, -coords.2),
        (-coords.0, coords.1, -coords.2),
        (-coords.1, -coords.0, -coords.2),
        // Right->left (looking at YZ plane)
        (coords.1, coords.2, coords.0),
        (coords.2, -coords.1, coords.0),
        (-coords.1, -coords.2, coords.0),
        (-coords.2, coords.1, coords.0),
        // Left->right (looking at -YZ plane)
        (coords.1, -coords.2, -coords.0),
        (coords.2, coords.1, -coords.0),
        (-coords.1, coords.2, -coords.0),
        (-coords.2, -coords.1, -coords.0),
        // Top->down (looking at XZ plane)
        (coords.0, coords.2, -coords.1),
        (coords.2, -coords.0, -coords.1),
        (-coords.0, -coords.2, -coords.1),
        (-coords.2, coords.0, -coords.1),
        // Bottom->up (looking at -XZ plane)
        (coords.0, -coords.2, coords.1),
        (coords.2, coords.0, coords.1),
        (-coords.0, coords.2, coords.1),
        (-coords.2, -coords.0, coords.1),
    ]
}

// Create 24 beacon sets for the 24 orientations given an input set
fn beacon_sets(beacons: &HashSet<(i32, i32, i32)>) -> Vec<HashSet<(i32, i32, i32)>> {
    let mut output = vec![];
    for _ in 0..24 {
        output.push(HashSet::new());
    }

    for beacon in beacons {
        let beacon_views = orientations(*beacon);
        for i in 0..24 {
            output[i].insert(beacon_views[i]);
        }
    }

    output
}

// Set of points referenced to some other point... just a linear translation.
fn referenced_set(
    set: &HashSet<(i32, i32, i32)>,
    ref_point: (i32, i32, i32),
) -> HashSet<(i32, i32, i32)> {
    let mut output = HashSet::new();
    for point in set {
        output.insert((
            point.0 - ref_point.0,
            point.1 - ref_point.1,
            point.2 - ref_point.2,
        ));
    }
    output
}

// Set translation to each of the beacons in known set, then for each proposal
// orientation beacon HashSet set translation relative to each of _its_ beacons,
// see if at least 12 beacons between known set and proposal set works
fn check_set(
    known_set: &HashSet<(i32, i32, i32)>,
    proposal_set: &HashSet<(i32, i32, i32)>,
) -> Option<(i32, i32, i32)> {
    for ref_point in known_set {
        let known_referenced_set = referenced_set(known_set, *ref_point);

        for proposal_point in proposal_set {
            // Check proposal set referenced to each of its sensors
            let proposal_referenced_set = referenced_set(proposal_set, *proposal_point);

            let intersection_set: HashSet<_> = known_referenced_set
                .intersection(&proposal_referenced_set)
                .collect();

            if intersection_set.len() >= 12 {
                // Found one!

                // Vknown_goal = Vknown_common + Vcg = Vknown_common + Vprop_goal - Vprop_common
                // translation = Vknown_common - Vprop_common
                // Vknown_goal = Vprop_goal + translation
                // ref_point = Vknown_common
                // proposal_point = Vprop_common
                let translation = (
                    ref_point.0 - proposal_point.0,
                    ref_point.1 - proposal_point.1,
                    ref_point.2 - proposal_point.2,
                );
                return Some(translation);
            }
        }
    }

    None
}

#[aoc(day19, part1)]
fn part1(input: &[Scanner]) -> usize {
    let mut absmap = HashSet::<(i32, i32, i32)>::new();

    // First get our absolute position established
    for beacon in &input[0].beacons {
        absmap.insert(*beacon);
    }

    // Actually have to think now
    let mut next_round_scanners_to_solve = input[1..].to_vec();
    loop {
        let scanners_to_solve = next_round_scanners_to_solve.clone();
        if scanners_to_solve.is_empty() {
            // When we've solved them all we're done
            break;
        }
        let mut remove_idxs = vec![];
        for (i, scanner) in scanners_to_solve.iter().enumerate() {
            // TODO: Find which scanner(s?) see the same beacons
            // TODO: Iterate through orientations (24), see if at least 12 beacons
            //       match known beacons.
            let proposal_sets = beacon_sets(&scanner.beacons);
            for proposal_set in proposal_sets {
                // Look at each orientation of the proposal set
                if let Some(translation) = check_set(&absmap, &proposal_set) {
                    let absnew = proposal_set
                        .iter()
                        .map(|pt| {
                            (
                                pt.0 + translation.0,
                                pt.1 + translation.1,
                                pt.2 + translation.2,
                            )
                        })
                        .collect();
                    absmap = absmap
                        .union(&absnew)
                        .copied()
                        .collect::<HashSet<(i32, i32, i32)>>();
                    remove_idxs.push(i);
                    break;
                }
            }
        }

        // Remove scanners we've found for next iteration
        if remove_idxs.is_empty() {
            panic!("This should never happen.");
        }
        remove_idxs.sort_unstable();
        for i in remove_idxs.iter().rev() {
            next_round_scanners_to_solve.remove(*i);
        }
    }
    absmap.len()
}

#[aoc(day19, part2)]
fn part2(input: &[Scanner]) -> i32 {
    let mut absmap = HashSet::<(i32, i32, i32)>::new();

    // First get our absolute position established
    for beacon in &input[0].beacons {
        absmap.insert(*beacon);
    }

    let mut translations = vec![];

    // Actually have to think now
    let mut next_round_scanners_to_solve = input[1..].to_vec();
    loop {
        let scanners_to_solve = next_round_scanners_to_solve.clone();
        if scanners_to_solve.is_empty() {
            // When we've solved them all we're done
            break;
        }
        let mut remove_idxs = vec![];
        for (i, scanner) in scanners_to_solve.iter().enumerate() {
            // TODO: Find which scanner(s?) see the same beacons
            // TODO: Iterate through orientations (24), see if at least 12 beacons
            //       match known beacons.
            let proposal_sets = beacon_sets(&scanner.beacons);
            for proposal_set in proposal_sets {
                // Look at each orientation of the proposal set
                if let Some(translation) = check_set(&absmap, &proposal_set) {
                    translations.push(translation);
                    let absnew = proposal_set
                        .iter()
                        .map(|pt| {
                            (
                                pt.0 + translation.0,
                                pt.1 + translation.1,
                                pt.2 + translation.2,
                            )
                        })
                        .collect();
                    absmap = absmap
                        .union(&absnew)
                        .copied()
                        .collect::<HashSet<(i32, i32, i32)>>();
                    remove_idxs.push(i);
                    break;
                }
            }
        }

        // Remove scanners we've found for next iteration
        if remove_idxs.is_empty() {
            panic!("This should never happen.");
        }
        remove_idxs.sort_unstable();
        for i in remove_idxs.iter().rev() {
            next_round_scanners_to_solve.remove(*i);
        }
    }

    // Finally look at all translations and find largest distance
    let mut dists = vec![];
    for i in 0..translations.len() {
        for j in 0..translations.len() {
            if i != j {
                let t0 = translations[i];
                let t1 = translations[j];
                let dist = (t0.0 - t1.0).abs() + (t0.1 - t1.1).abs() + (t0.2 - t1.2).abs();
                dists.push(dist)
            }
        }
    }
    *dists.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/2021/19.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 79);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/2021/19.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 3621);
    }
}

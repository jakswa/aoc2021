use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let scanners = input
        .split("\n\n")
        .map(|i| {
            i.lines()
                .skip(1)
                .map(|l| {
                    l.split(',')
                        .map(|j| j.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>()
        })
        .collect::<Vec<Vec<Vec<i64>>>>();

    let mut found: HashMap<usize, (i64, i64, i64, usize)> = HashMap::new();
    found.insert(0, (0, 0, 0, 0));
    let mut beacons: HashSet<(i64, i64, i64)> = HashSet::new();

    loop {
        if found.len() == scanners.len() {
            println!("WOW DONE MATCHING.");
            break;
        }
        let mut newly_found: Vec<(usize, i64, i64, i64, usize)> = vec![];
        scanners
            .iter()
            .enumerate()
            .filter(|(sind, _s)| !found.contains_key(sind))
            .for_each(|(sind, scanner)| {
                'outr: for (rind, rotscan) in rotations(&scanner).iter().enumerate() {
                    for find in found.keys() {
                        let fval = found[find];
                        let foundscan = &rotations(&scanners[*find])[fval.3];
                        if let Some(ohmy) = meet(fval, foundscan, rotscan) {
                            println!("MATCH! {}", sind);
                            newly_found.push((sind, ohmy.0, ohmy.1, ohmy.2, rind));
                            break 'outr;
                        }
                    }
                }
            });
        if newly_found.is_empty() {
            break;
        }
        newly_found
            .drain(..)
            .for_each(|n: (usize, i64, i64, i64, usize)| {
                found.insert(n.0, (n.1, n.2, n.3, n.4));
            });
    }

    // part 1. counting unique points, fuck
    for (sind, found_attrs) in found.iter() {
        let oriented_scanner = &rotations(&scanners[*sind])[found_attrs.3];
        for b in oriented_scanner {
            beacons.insert((
                b[0] + found_attrs.0,
                b[1] + found_attrs.1,
                b[2] + found_attrs.2,
            ));
        }
    }

    println!("FUCKING HELL: {}", beacons.len());

    // part 2. finding max scanner dist
    let mut max: i64 = 0;
    for (ind, (sx, sy, sz, _)) in found.iter() {
        for (iind, (ssx, ssy, ssz, _)) in found.iter() {
            if ind == iind {
                continue;
            }
            let dist = (ssx - sx).abs() + (ssy - sy).abs() + (ssz - sz).abs();
            if dist > max {
                max = dist;
            }
        }
    }
    println!("max distance: {}", max);
}

fn meet(
    loffset: (i64, i64, i64, usize),
    l: &Vec<Vec<i64>>,
    r: &Vec<Vec<i64>>,
) -> Option<(i64, i64, i64)> {
    for rb in r.iter() {
        for lb in l.iter().skip(l.len() - 12) {
            let roffset = (
                lb[0] + loffset.0 - rb[0],
                lb[1] + loffset.1 - rb[1],
                lb[2] + loffset.2 - rb[2],
            );
            if r.iter()
                .filter(|rbb| {
                    let rbbn = (rbb[0] + roffset.0, rbb[1] + roffset.1, rbb[2] + roffset.2);
                    l.iter().any(|lbb| {
                        lbb[0] + loffset.0 == rbbn.0
                            && lbb[1] + loffset.1 == rbbn.1
                            && lbb[2] + loffset.2 == rbbn.2
                    })
                })
                .count()
                >= 12
            {
                return Some(roffset);
            }
        }
    }
    None
}

fn rotations(scanner: &Vec<Vec<i64>>) -> Vec<Vec<Vec<i64>>> {
    let mut orientations = vec![];

    // X positive
    orientations.push(scanner.clone());
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[0], -beacon[1], -beacon[2]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[0], -beacon[2], beacon[1]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[0], beacon[2], -beacon[1]])
            .collect::<Vec<Vec<i64>>>(),
    );

    // X negative
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[0], -beacon[1], beacon[2]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[0], beacon[1], -beacon[2]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[0], beacon[2], beacon[1]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[0], -beacon[2], -beacon[1]])
            .collect::<Vec<Vec<i64>>>(),
    );

    //y pos
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[1], beacon[0], beacon[2]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[1], beacon[0], -beacon[2]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[2], beacon[0], beacon[1]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[2], beacon[0], -beacon[1]])
            .collect::<Vec<Vec<i64>>>(),
    );

    // y neg
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[1], -beacon[0], beacon[2]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[1], -beacon[0], -beacon[2]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[2], -beacon[0], beacon[1]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[2], -beacon[0], -beacon[1]])
            .collect::<Vec<Vec<i64>>>(),
    );

    // z pos
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[2], beacon[1], beacon[0]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[2], -beacon[1], beacon[0]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[1], beacon[2], beacon[0]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[1], -beacon[2], beacon[0]])
            .collect::<Vec<Vec<i64>>>(),
    );

    // z neg
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[2], beacon[1], -beacon[0]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[2], -beacon[1], -beacon[0]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![beacon[1], -beacon[2], -beacon[0]])
            .collect::<Vec<Vec<i64>>>(),
    );
    orientations.push(
        scanner
            .iter()
            .map(|beacon| vec![-beacon[1], beacon[2], -beacon[0]])
            .collect::<Vec<Vec<i64>>>(),
    );

    orientations
}


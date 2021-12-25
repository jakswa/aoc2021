fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
    let mut cmds = input
        .lines()
        .enumerate()
        .map(|(lind, line)| {
            let on = &line[0..2] == "on";
            let ranges = line.split('=').skip(1);
            let ranges = ranges
                .map(|c| {
                    let mut nums = c
                        .split(',')
                        .next()
                        .unwrap()
                        .split("..")
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    nums.sort();
                    nums
                })
                .collect::<Vec<Vec<i32>>>();
            (
                lind,
                on,
                [
                    ranges[0][0],
                    ranges[0][1],
                    ranges[1][0],
                    ranges[1][1],
                    ranges[2][0],
                    ranges[2][1],
                ],
            )
        })
        .collect::<Vec<(usize, bool, [i32; 6])>>();

    let mut prisms_on = vec![cmds[0].2];
    'cmdloop: for cmd in cmds[1..].iter().filter(|cmd| cmd.1) {
        let mut collide = vec![cmd.2];
        for p in prisms_on.iter() {
            for col in collide.clone().iter() {
                match try_collide(*p, *col) {
                    None => break 'cmdloop,
                    Some(mut new_collide) => collide.append(&mut new_collide),
                }
            }
        }
        prisms_on.append(&mut collide);
    }
    println!("shit {}: {:?}", prisms_on.len(), prisms_on);
}

fn try_collide(station: [i32; 6], orbit: [i32; 6]) -> Option<Vec<[i32; 6]>> {
    if is_disjointed(&station, &orbit) {
        return Some(vec![]);
    }
    if swallows(&station, &orbit) {
        return None;
    }
    let mut ret = vec![];
    for (x1, x2) in cranges(station[0], station[1], orbit[0], orbit[1]).iter() {
        for (y1, y2) in cranges(station[2], station[3], orbit[2], orbit[3]).iter() {
            for (z1, z2) in cranges(station[4], station[5], orbit[4], orbit[5]).iter() {
                ret.push([*x1, *x2, *y1, *y2, *z1, *z2]);
            }
        }
    }
    Some(ret)
}

fn cranges(s1: i32, s2: i32, o1: i32, o2: i32) -> Vec<(i32, i32)> {
    if s1 <= o1 && o2 <= s2 {
        vec![(o1, o2)] // maybe?
    } else if o1 < s1 && s2 < o2 {
        vec![(o1, s1 - 1), (s1, s2), (s2, o2)]
    } else if s1 <= o1 {
        vec![(s2 + 1, o2)]
    } else {
        vec![(o1, s1 - 1)]
    }
}

fn is_disjointed(s: &[i32; 6], o: &[i32; 6]) -> bool {
    s[0] > o[1] || s[1] < o[0] || s[2] > o[3] || s[3] < o[2] || s[4] > o[5] || s[5] < o[4]
}

fn swallows(s: &[i32; 6], o: &[i32; 6]) -> bool {
    s[0] < o[0] && s[1] > o[1] && s[2] < o[2] && s[3] > o[3] && s[4] < o[4] && s[5] > o[5]
}


fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
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
    let mut prisms_on: Vec<[i32; 6]> = vec![cmds[0].2];
    for (_lind, onoff, prism) in &cmds[1..] {
        if *onoff {
            let mut distinct = vec![*prism];
            for pon in prisms_on.iter() {
                distinct = collide_array(pon, distinct);
            }
            prisms_on.append(&mut distinct);
        } else {
            prisms_on = collide_array(prism, prisms_on);
        }
    }
    println!("res: {:?}", prisms_on);
    println!("hmmm: {:?}", sum_those_on(&prisms_on));
}

fn collide_array(s: &[i32; 6], other: Vec<[i32; 6]>) -> Vec<[i32; 6]> {
    let mut res = vec![];
    for o in other.iter() {
        res.append(&mut collide(s, o));
    }
    res
}

fn overlaps(s: &[i32; 6], o: &[i32; 6]) -> bool {
    !(s[0] > o[1] || s[1] < o[0] || s[2] > o[3] || s[3] < o[2] || s[4] > o[5] || s[5] < o[4])
}

fn swallows(s: &[i32; 6], o: &[i32; 6]) -> bool {
    s[0] < o[0] && o[1] < s[1] && s[2] < o[2] && o[3] < s[3] && s[4] < o[4] && o[5] < s[5]
}

fn collide(s: &[i32; 6], o: &[i32; 6]) -> Vec<[i32; 6]> {
    if !overlaps(s, o) {
        return vec![o.clone()];
    }
    if swallows(s, o) {
        return vec![];
    }
    let mut res = vec![];
    for (xmin, xmax) in cranges(s[0], s[1], o[0], o[1]) {
        for (ymin, ymax) in cranges(s[2], s[3], o[2], o[3]) {
            for (zmin, zmax) in cranges(s[4], s[5], o[4], o[5]) {
                let candidate = [xmin, xmax, ymin, ymax, zmin, zmax];
                if !overlaps(s, &candidate) {
                    res.push(candidate);
                }
            }
        }
    }
    res
}

fn cranges(s1: i32, s2: i32, o1: i32, o2: i32) -> Vec<(i32, i32)> {
    if s1 <= o1 && o2 <= s2 {
        vec![(o1, o2)] // swallow just this dimension
    } else if o1 <= s1 && s2 <= o2 {
        vec![(o1, s1 - 1), (s1, s2), (s2 + 1, o2)]
    } else if s1 < o1 {
        vec![(o1, s2), (s2 + 1, o2)]
    } else {
        vec![(o1, s1 - 1), (s1, o2)]
    }
}

fn sum_those_on(prisms: &Vec<[i32; 6]>) -> i64 {
    prisms
        .iter()
        .map(|p| (p[1] - p[0] + 1) as i64 * (p[3] - p[2] + 1) as i64 * (p[5] - p[4] + 1) as i64)
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // on x=10..12,y=10..12,z=10..12
    // on x=11..13,y=11..13,z=11..13
    // off x=9..11,y=9..11,z=9..11
    // on x=10..10,y=10..10,z=10..10
    #[test]
    fn test_example_collide() {
        let station = &[10, 12, 10, 12, 10, 12];
        let mut collides = collide(station, &[11, 13, 11, 13, 11, 13]);
        let mut all_on = vec![station.clone()];
        all_on.append(&mut collides);
        assert_eq!(sum_those_on(&all_on), 46);

        all_on = collide_array(&[9, 11, 9, 11, 9, 11], all_on);
        assert_eq!(sum_those_on(&all_on), 38);

        let mut adds: Vec<[i32; 6]> = vec![[10, 10, 10, 10, 10, 10]];
        for i in all_on.iter() {
            adds = collide_array(i, adds);
        }
        all_on.append(&mut adds);
        assert_eq!(sum_those_on(&all_on), 39);
    }

    #[test]
    fn test_explode_collide() {
        let station = &[0, 5, 0, 5, 0, 5];
        let collides = collide(station, &[-2, 7, -2, 7, -2, 7]);
        assert_eq!(collides.len(), 26);
    }

    #[test]
    fn test_swallow_collide() {
        let station = &[0, 5, 0, 5, 0, 5];
        let collides = collide(station, &[1, 4, 1, 4, 1, 4]);
        assert_eq!(collides.len(), 0);
    }

    #[test]
    fn test_stab_cube_collide() {
        let station = &[0, 5, 0, 5, 0, 5];
        let collides = collide(station, &[1, 4, 1, 4, 3, 7]);
        assert_eq!(collides.len(), 1);
    }

    #[test]
    fn test_side_cube_collide() {
        let station = &[0, 5, 0, 5, 0, 5];
        let collides = collide(station, &[1, 4, 3, 7, 3, 7]);
        assert_eq!(collides.len(), 3);
    }

    #[test]
    fn test_corner_cube_collide() {
        let station = &[0, 5, 0, 5, 0, 5];
        let collides = collide(station, &[3, 7, 3, 7, 3, 7]);
        assert_eq!(collides.len(), 7);
    }

    #[test]
    fn test_no_collide() {
        assert_eq!(
            collide(&[0, 5, 0, 5, 0, 5], &[6, 8, 6, 8, 6, 8]),
            vec![[6, 8, 6, 8, 6, 8]]
        );
    }

    #[test]
    fn test_single_point_overlap() {
        assert_eq!(overlaps(&[0, 5, 0, 5, 0, 5], &[-5, 0, -5, 0, -5, 0]), true);
    }

    #[test]
    fn test_line_overlap() {
        assert_eq!(overlaps(&[0, 5, 0, 5, 0, 5], &[-5, 0, -5, 0, 0, 5]), true);
    }

    #[test]
    fn test_side_overlap() {
        assert_eq!(overlaps(&[0, 5, 0, 5, 0, 5], &[-5, 0, 0, 5, 0, 5]), true);
    }

    #[test]
    fn test_basic_overlap() {
        assert_eq!(overlaps(&[0, 5, 0, 5, 0, 5], &[2, 7, 2, 7, 2, 7]), true);
    }

    #[test]
    fn test_straddling_overlap() {
        assert_eq!(overlaps(&[-1, 7, 2, 4, 2, 4], &[0, 5, 0, 5, 0, 5]), true);
    }

    #[test]
    fn test_stabbing_overlap() {
        assert_eq!(overlaps(&[2, 8, 2, 4, 2, 4], &[0, 5, 0, 5, 0, 5]), true);
    }

    #[test]
    fn test_swallowing_overlap() {
        assert_eq!(overlaps(&[3, 5, 3, 5, 3, 5], &[2, 7, 2, 7, 2, 7]), true);
    }

    #[test]
    fn test_basic_non_overlap() {
        assert_eq!(
            overlaps(&[0, 5, 0, 5, 0, 5], &[12, 17, 12, 17, 12, 17]),
            false
        );
    }
}


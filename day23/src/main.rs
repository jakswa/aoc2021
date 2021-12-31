fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut pos: Vec<(char, usize, usize)> = vec![];
    for (lind, line) in input.lines().enumerate() {
        for (cind, c) in line.chars().enumerate() {
            match c {
                'A' => {
                    pos.push((c, cind, lind));
                }
                'B' => {
                    pos.push((c, cind, lind));
                }
                'C' => {
                    pos.push((c, cind, lind));
                }
                'D' => {
                    pos.push((c, cind, lind));
                }
                _ => {}
            }
        }
    }

    pos.sort_by_key(|i| i.0);

    println!("wee: {:?}", dig(&mut pos, 0, &mut i64::MAX, 0));
}

fn pp(pos: &Vec<(char, usize, usize)>, cost: i64) {
    for y in 0..6 {
        for x in 0..12 {
            if let Some(wee) = pos.iter().find(|i| i.1 == x && i.2 == y) {
                print!("{}", wee.0);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("=-=-=-=-{}=-=-=-=-", cost);
}

fn home(c: char) -> usize {
    ((c as u8 - 'A' as u8) * 2 + 3) as usize
}

fn move_cost(c: char) -> usize {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        _ => 1000, // 'D'
    }
}

fn dig(
    pos: &mut Vec<(char, usize, usize)>,
    cost: i64,
    least_cost: &mut i64,
    cnt: u64,
) -> Option<i64> {
    //pp(&pos, cost);
    let mut least_cost = *least_cost;
    if cost > least_cost {
        return None;
    }
    if is_win(&pos) {
        return Some(cost);
    }

    for ind in 0..pos.len() {
        let p = pos[ind];
        let orig = p.clone();
        for m in moves(ind, &pos).iter().rev() {
            pos[ind] = (p.0, m.0, m.1);
            if let Some(found_cost) = dig(
                pos,
                cost + (((p.1.max(m.0) - p.1.min(m.0)) + (p.2.max(m.1) - p.2.min(m.1)))
                    * move_cost(p.0)) as i64,
                &mut least_cost,
                cnt + 1,
            ) {
                if found_cost < least_cost {
                    println!("least cost: {}", found_cost);
                    least_cost = found_cost;
                }
            }
        }
        if cnt == 0 {
            println!("ding.");
        }
        pos[ind] = orig;
    }
    if least_cost != i64::MAX {
        return Some(least_cost);
    }
    None
}

const TOPS: [(usize, usize); 7] = [(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];

fn moves(ind: usize, pos: &Vec<(char, usize, usize)>) -> Vec<(usize, usize)> {
    let p = pos[ind];
    // means in home slot and no need to unblock one beneath
    let hoome = home(p.0);
    let perm_home = p.1 == hoome
        && !pos
            .iter()
            .any(|pp| p.0 != pp.0 && pp.1 == p.1 && pp.2 > p.2);
    if perm_home {
        return vec![];
    }
    let can_move_home = p.2 == 1
        && !pos.iter().any(|pp| p.0 != pp.0 && pp.1 == hoome)
        && !pos
            .iter()
            .any(|pp| pp.2 == 1 && ((p.1 < pp.1 && pp.1 < hoome) || (p.1 > pp.1 && pp.1 > hoome)));
    if can_move_home {
        let mut res = vec![];
        for yy in 2..=5 {
            if pos.iter().any(|pp| pp.1 == hoome && pp.2 == yy) {
                break;
            }
            res.push((hoome, yy));
        }
        return vec![res.last().unwrap().clone()];
    }
    let can_move_up = p.2 > 1 && !pos.iter().any(|pp| pp.1 == p.1 && pp.2 < p.2);
    if can_move_up {
        // return top spaces that aren't blocked by any other
        return TOPS
            .iter()
            .filter(|(xx, yy)| {
                !pos.iter().any(|pp| {
                    pp.2 == 1
                        && ((*xx > p.1 && pp.1 > p.1 && pp.1 < *xx)
                            || (*xx < p.1 && pp.1 < p.1 && pp.1 > *xx)
                            || (pp.1 == *xx && pp.2 == *yy))
                })
            })
            .map(|i| (i.0, i.1))
            .collect::<Vec<(usize, usize)>>();
    }
    vec![]
}

fn is_win(pos: &Vec<(char, usize, usize)>) -> bool {
    pos.iter().all(|p| p.1 == home(p.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_move_to_empty_home() {
        let m = moves(0, &vec![('A', 1, 1)]);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0], (3, 5));
    }

    #[test]
    fn can_move_to_ready_home() {
        let m = moves(0, &vec![('A', 1, 1), ('A', 3, 5)]);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0], (3, 4));
    }

    #[test]
    fn can_be_blocked_from_home() {
        let m = moves(0, &vec![('A', 1, 1), ('B', 2, 1)]);
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn can_avoid_home_if_not_ready() {
        let m = moves(0, &vec![('A', 1, 1), ('B', 3, 5)]);
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn can_be_perm_home() {
        let m = moves(0, &vec![('A', 3, 5)]);
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn can_leave_start() {
        let m = moves(0, &vec![('A', 5, 5)]);
        assert_eq!(m.len(), TOPS.len());
    }

    #[test]
    fn can_ignore_blocked_top_spaces() {
        let m = moves(0, &vec![('A', 5, 5), ('B', 4, 1)]);
        assert_eq!(m.len(), TOPS.len() - 3);
    }

    #[test]
    fn can_ignore_some_blocked_top_spaces() {
        let m = moves(0, &vec![('A', 5, 5), ('B', 4, 1)]);
        assert_eq!(m.len(), TOPS.len() - 3);
    }

    #[test]
    fn can_have_all_top_spaces_blocked() {
        let m = moves(0, &vec![('A', 5, 5), ('B', 4, 1), ('C', 6, 1)]);
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn welp() {
        let m = moves(0, &vec![('A', 1, 1), ('B', 2, 1), ('C', 6, 1)]);
        assert_eq!(m.len(), 0);
    }
}


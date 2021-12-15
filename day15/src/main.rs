use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|i| {
            i.chars()
                .map(|j| j.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    // ========
    // part 2 addition
    let origlen = grid.len();
    for i in 1..5 {
        for j in 0..origlen {
            let n = grid[j].iter().map(|k| k + i).collect();
            grid.push(n);
        }
    }
    let origlen = grid.len();
    let origwid = grid[0].len();
    for i in 1..5 {
        for j in 0..origlen {
            let n = grid[j][0..origwid]
                .iter()
                .map(|k| k + i)
                .collect::<Vec<u8>>();
            n.iter().for_each(|o| grid[j].push(*o));
        }
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] > 9 {
                grid[i][j] -= 9;
            }
        }
    }
    // ========

    let mut costs: HashMap<(usize, usize), u64> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let adj = vec![(0, 1), (1, 0), (2, 1), (1, 2)];
    costs.insert((0, 0), 0);

    loop {
        let val = costs.iter().min_by_key(|(_i, j)| *j).unwrap();
        let (x, y, cost) = (val.0 .0, val.0 .1, *val.1);
        costs.remove(&(x, y));
        visited.insert((x, y));
        if x == grid.len() - 1 && y == grid.len() - 1 {
            println!("done: {:?}", (x, y, cost));
            break;
        }

        for (nx, ny) in adj.iter() {
            let xx = *nx;
            let yy = *ny;
            // don't care about out-of-bounds visits
            if (yy == 0 && y == 0)
                || (y >= (grid[0].len() - 1) && yy == 2)
                || (xx == 0 && x == 0)
                || (x >= (grid.len() - 1) && xx == 2)
            {
                continue;
            }
            let newx = x + xx - 1;
            let newy = y + yy - 1;
            let new_cost = cost + grid[newx][newy] as u64;
            match costs.get(&(newx, newy)) {
                Some(cost) => {
                    if cost > &new_cost {
                        costs.insert((newx, newy), new_cost);
                    }
                }
                None => {
                    if !visited.contains(&(newx, newy)) {
                        costs.insert((newx, newy), new_cost);
                    }
                }
            }
        }
    }
}


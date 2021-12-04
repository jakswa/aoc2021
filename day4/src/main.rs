fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let mut callouts = lines.next().unwrap().split(',')
        .filter_map(|i| i.parse::<u16>().ok());

    let mut grids: Vec<Vec<Vec<u16>>> = vec![];
    lines.next();
    let mut grid: Vec<Vec<u16>> = vec![];

    for line in lines {
        if line.len() == 0 {
            grids.push(grid);
            grid = vec![];
            continue;
        }

        let line = line.split_whitespace()
            .map(|i| i.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
        grid.push(line);
    }

    grids.push(grid);

    let mut shouts: Vec<u16> = vec![];
    let mut winners: Vec<u16> = vec![];
    loop {
        shouts.push(callouts.next().unwrap());
        grids.iter().enumerate().filter(|(_grind, grid)| is_bingo(&grid, &shouts)).for_each(|(grind, winner)| {
            if !winners.contains(&(grind as u16)) {
                winners.push(grind as u16);
            }
        });
        if winners.len() == grids.len() { break; }
    }

    let loserwinner = &grids[*winners.last().unwrap() as usize];
    println!("done, {:?}, {:?}", shouts, board_score(loserwinner, &shouts));
}

fn board_score(grid: &Vec<Vec<u16>>, nums: &Vec<u16>) -> u16 {
    let sums: u16 = grid.iter().map(|row| {
        let rowsum: u16 = row.iter().filter(|i| !nums.contains(&i)).sum();
        rowsum
    }).sum();
    println!("HMM: {}, {}", sums, nums.last().unwrap());
    sums * nums.last().unwrap()
}

fn is_bingo(grid: &Vec<Vec<u16>>, nums: &Vec<u16>) -> bool {
    let horizontal = grid.iter().any( |hline|  {
        hline.iter().all(|num| nums.contains(num))
    });
    if horizontal { println!("HORIZONTAL!, {}", grid[0][0]); return true; }

    let vertical = (0..grid[0].len()).any(|i| {
        (0..grid.len()).all(|j| nums.contains(&grid[j][i]))
    });
    if vertical { println!("VERTICAL!"); return true; }

    // if (0..5).all(|ind| nums.contains(&grid[ind][ind])) ||
    //         (0..5).all(|ind| nums.contains(&grid[4-ind][ind])) {
    //     println!("DIAGONAL!");
    //     return true;
    // }
    false
}

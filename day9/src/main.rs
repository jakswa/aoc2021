fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|i| i.chars().map(|j| (j as u8) - ('0' as u8)).collect())
        .collect();

    let ylen = grid[0].len();
    let xlen = grid.len();

    let mut basins: Vec<Vec<(usize, usize)>> = vec![];
    for x in 0..xlen {
        for y in 0..ylen {
            let curr = grid[x][y];
            if (x == 0 || grid[x - 1][y] > curr)
                && (y == 0 || grid[x][y - 1] > curr)
                && (x == (xlen - 1) || grid[x + 1][y] > curr)
                && (y == (ylen - 1) || grid[x][y + 1] > curr)
            {
                basins.push(dig(&grid, (x, y), vec![]));
            }
        }
    }

    basins.sort_by(|b, c| b.len().cmp(&c.len()));
    println!(
        "done: {}",
        basins.pop().unwrap().len() * basins.pop().unwrap().len() * basins.pop().unwrap().len()
    );
}

fn dig(
    grid: &Vec<Vec<u8>>,
    pos: (usize, usize),
    mut visited: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    if pos.0 >= grid.len()
        || pos.1 >= grid[0].len()
        || grid[pos.0][pos.1] == 9
        || visited.contains(&pos)
    {
        return visited;
    }
    visited.push(pos);
    println!("visiting: {:?}", pos);
    if pos.0 > 0 {
        let next = (pos.0 - 1, pos.1);
        if !visited.contains(&next) {
            visited = dig(grid, next, visited.clone());
        }
    }
    if pos.1 > 0 {
        let next = (pos.0, pos.1 - 1);
        if !visited.contains(&next) {
            visited = dig(grid, next, visited.clone());
        }
    }
    let next = (pos.0 + 1, pos.1);
    if !visited.contains(&next) {
        visited = dig(grid, next, visited.clone());
    }
    let next = (pos.0, pos.1 + 1);
    if !visited.contains(&next) {
        visited = dig(grid, next, visited.clone());
    }
    visited
}


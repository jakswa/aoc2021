fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|i| i.chars().map(|j| (j as u8) - ('0' as u8)).collect())
        .collect();

    let ylen = grid[0].len();
    let xlen = grid.len();

    let mut cost: u64 = 0;
    for x in 0..xlen {
        for y in 0..ylen {
            let curr = grid[x][y];
            if (x == 0 || grid[x - 1][y] > curr)
                && (y == 0 || grid[x][y - 1] > curr)
                && (x == (xlen - 1) || grid[x + 1][y] > curr)
                && (y == (ylen - 1) || grid[x][y + 1] > curr)
            {
                cost += (curr as u64) + 1;
            }
        }
    }

    println!("done: {}", cost);
}


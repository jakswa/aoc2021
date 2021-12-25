fn main() {
    let mut grid = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|i| i.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut steps = 0;
    let xwid = grid.len();
    let ywid = grid[0].len();
    let mut nextgrid = grid.clone();
    //pp(&grid);
    let mut unmoved_cnt = 0;

    //for _ in 0..10 {
    loop {
        let mut moved = false;
        for x in 0..xwid {
            for y in 0..ywid {
                let c = grid[x][y];
                if c == '.' {
                    continue;
                }
                match steps % 2 == 0 {
                    true => {
                        if c == 'v' {
                            continue;
                        }
                    }
                    false => {
                        if c == '>' {
                            continue;
                        }
                    }
                }
                let mut nx = x;
                let mut ny = y;
                match c {
                    '>' => ny += 1,
                    'v' => nx += 1,
                    _ => {}
                }
                if nx >= xwid {
                    nx = 0;
                }
                if ny >= ywid {
                    ny = 0;
                }
                if grid[nx][ny] == '.' {
                    moved = true;
                    nextgrid[nx][ny] = c;
                    nextgrid[x][y] = '.';
                }
            }
        }
        steps += 1;
        if !moved {
            unmoved_cnt += 1;
            if steps % 2 == 0 && unmoved_cnt > 1 {
                break;
            }
        } else {
            unmoved_cnt = 0;
        }
        grid = nextgrid;
        nextgrid = grid.clone();
        if steps % 2 == 0 {
            unmoved_cnt = 0;
            //pp(&grid);
        }
    }
    println!("Hello, world! {}", steps / 2);
}

fn pp(g: &Vec<Vec<char>>) {
    g.iter().for_each(|vr| {
        vr.iter().for_each(|c| {
            print!("{}", c);
        });
        println!("");
    });
    println!("\n-----------\n");
}


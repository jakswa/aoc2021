fn main() {
    let mut input: Vec<Vec<u8>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|i| {
            i.chars()
                .map(|i| i.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let mut flashes: Vec<(usize, usize)> = vec![];
    let mut prev_flash: u64 = 0;
    let mut flash_count: u64 = 0;
    let mut step = 0;

    loop {
        for x in 0..10 {
            for y in 0..10 {
                input[x][y] += 1;
                if input[x][y] == 10 {
                    flashes.push((x, y));
                }
            }
        }
        loop {
            if flashes.len() == 0 {
                break;
            }
            let flash = flashes.pop().unwrap();
            for dx in -1..=1_i8 {
                for dy in -1..=1_i8 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let xx = (flash.0 as i8) + dx;
                    let yy = (flash.1 as i8) + dy;
                    if xx < 0 || yy < 0 || xx >= 10 || yy >= 10 {
                        continue;
                    }
                    input[xx as usize][yy as usize] += 1;
                    if input[xx as usize][yy as usize] == 10 {
                        flashes.push((xx as usize, yy as usize));
                    }
                }
            }
        }
        prev_flash = flash_count;
        for x in 0..10 {
            for y in 0..10 {
                if input[x][y] > 9 {
                    input[x][y] = 0;
                    flash_count += 1;
                }
            }
        }
        step += 1;
        if flash_count - prev_flash >= 100 {
            break;
        }
    }

    println!("step: {}", step);
}


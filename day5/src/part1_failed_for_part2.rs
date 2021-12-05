fn main() {
    let xyxy: Vec<u16> = std::fs::read_to_string("example.txt")
        .unwrap()
        .split(&[' ', '-', '>', ',', '\n'][..])
        .filter(|i| !i.is_empty())
        .filter_map(|s| s.parse::<u16>().ok())
        .map(|i| i as u16)
        .collect::<Vec<u16>>();

    let max: u16 = *xyxy.iter().max().unwrap();
    let mut hits: u64 = 0;
    for x in 0..=max {
        for y in 0..=max {
            let mut lines_hit = 0;
            for line_ind in 0..(xyxy.len() / 4) {
                let line = &xyxy[line_ind * 4..line_ind * 4 + 4];
                if overlap(line, x, y) {
                    lines_hit += 1
                }
            }
            if lines_hit > 1 {
                hits += 1
            }
        }
    }
    println!("wow: {:?}", hits);
}

fn overlap(line: &[u16], x: u16, y: u16) -> bool {
    if x == line[0]
        && x == line[2]
        && ((y >= line[1] && y <= line[3]) || (y >= line[3] && y <= line[1]))
    {
        return true;
    }
    if y == line[1]
        && y == line[3]
        && ((x >= line[0] && x <= line[2]) || (x >= line[2] && x <= line[0]))
    {
        return true;
    }

    //part2
    if line[0] == line[2] || line[1] == line[3] {
        return false;
    }
    let diag_xrange = match line[0] < line[2] {
        true => line[0]..=line[2],
        false => line[2]..=line[0],
    };
    let diag_yrange = match line[1] < line[3] {
        true => line[1]..=line[3],
        false => line[3]..=line[1],
    };

    for diagx in diag_xrange {
        for diagy in diag_yrange.clone() {
            if x == diagx && y == diagy {
                return true;
            }
        }
    }
    false
}


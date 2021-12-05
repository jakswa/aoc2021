use std::collections::HashMap;

fn main() {
    let mut cnts = HashMap::new();

    let xyxy = std::fs::read_to_string("input.txt").unwrap();
    let mut nums = xyxy
        .split(&[' ', '-', '>', ',', '\n'][..])
        .filter(|i| !i.is_empty())
        .filter_map(|s| s.parse::<u16>().ok())
        .map(|i| i as u16);

    loop {
        let first = match nums.next() {
            Some(f) => f,
            None => break,
        };

        let line = [
            first,
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        ];

        let mut x = line[0];
        let mut y = line[1];

        loop {
            let cnt = cnts.entry((x, y)).or_insert(0);
            *cnt += 1;
            if line[2] == x && line[3] == y {
                break;
            }
            if line[2] > x {
                x += 1
            }
            if line[2] < x {
                x -= 1
            }
            if line[3] > y {
                y += 1
            }
            if line[3] < y {
                y -= 1
            }
        }
    }

    println!("wow: {:?}", cnts.values().filter(|i| **i >= 2).count());
}


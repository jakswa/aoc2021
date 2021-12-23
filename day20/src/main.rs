fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut chunks = input.split("\n\n");

    let algo = chunks.next().unwrap().chars().collect::<Vec<char>>();
    let mut image = chunks
        .next()
        .unwrap()
        .lines()
        .map(|i| i.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for _ in 0..25 {
        image = enhance(&image, &algo, 200);
        image = enhance(&image, &algo, -100);
    }

    println!("yeet: {:?}", lit(image));
}

fn lit(v: Vec<Vec<char>>) -> i64 {
    let mut sum = 0;
    for r in v.iter() {
        for c in r.iter() {
            match c {
                '#' => sum += 1,
                _ => {}
            }
        }
    }
    sum
}

fn pp(v: Vec<Vec<char>>) {
    v.iter().for_each(|i| println!("{:?}", i));
}

fn enhance(image: &Vec<Vec<char>>, algo: &Vec<char>, pad: i64) -> Vec<Vec<char>> {
    let mut ret = vec![];

    for yy in 0..=(image.len() + pad as usize) as i64 {
        let mut row: Vec<char> = vec![];
        for xx in 0..=(image[0].len() + pad as usize) as i64 {
            let pixel = algo[pixel_for(image, xx - (pad / 2), yy - (pad / 2))];
            row.push(pixel);
        }
        ret.push(row);
    }

    ret
}

fn pixel_for(image: &Vec<Vec<char>>, x: i64, y: i64) -> usize {
    let mut number = 0;
    for yy in 0..=2 as i64 {
        for xx in 0..=2 as i64 {
            let ny: i64 = yy + y - 1;
            let nx: i64 = xx + x - 1;
            number <<= 1;
            if ny < 0 || ny >= image.len() as i64 || nx < 0 || nx >= image[0].len() as i64 {
                continue;
            } else if image[ny as usize][nx as usize] == '#' {
                number += 1;
            }
        }
    }
    number
}


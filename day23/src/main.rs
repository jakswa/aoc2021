fn main() {
    let input = std::fs::read_to_string("example.txt").unwrap();
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

    println!("Hello, world! {:?}", can_move(pos));
}

fn can_move(pos: Vec<(char, usize, usize)>) -> Vec<(char, usize, usize)> {
    pos.iter()
        .filter(|p| {
            pos.iter()
                .filter(|pp| pp != p)
                .filter(|pp| pp.1 == p.1 && pp.2 < p.2)
                .count()
                == 0
        })
        .map(|i| (i.0, i.1, i.2))
        .collect::<Vec<(char, usize, usize)>>()
}


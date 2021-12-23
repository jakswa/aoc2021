fn main() {
    let mut pos = vec![2, 9];
    let mut scores = vec![0, 0];
    let mut rolls = 0;
    let mut r = 0;

    loop {
        rolls += 3;
        for _ in 0..3 {
            r = roll(r);
            pos[0] += r;
        }
        scores[0] += pos[0] % 10 + 1;
        if scores[0] >= 1000 {
            println!("WIN1: {}", scores[1] * rolls);
            return;
        }
        rolls += 3;
        for _ in 0..3 {
            r = roll(r);
            pos[1] += r;
        }
        scores[1] += pos[1] % 10 + 1;
        if scores[1] >= 1000 {
            println!("WIN2: {}", scores[0] * rolls);
            return;
        }
        if scores[0] < 20 {
            println!("pos/scores: {:?} ==> {:?}", pos, scores);
        }
    }

    println!("Hello, world!");
}

fn roll(last: i64) -> i64 {
    if last < 100 {
        last + 1
    } else {
        1
    }
}


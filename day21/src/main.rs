use std::collections::HashMap;
fn main() {
    let mut cache: HashMap<(bool, u8, u8, u8, u8), (u64, u64)> = HashMap::new();
    println!("hm: {:?}", play(true, 3, 10, 0, 0, &mut cache));
}

fn play(
    p1turn: bool,
    p1: u8,
    p2: u8,
    p1s: u8,
    p2s: u8,
    cache: &mut HashMap<(bool, u8, u8, u8, u8), (u64, u64)>,
) -> (u64, u64) {
    if p1s >= 21 {
        return (1, 0);
    } else if p2s >= 21 {
        return (0, 1);
    }
    let mut p1wins: u64 = 0;
    let mut p2wins: u64 = 0;
    if p1turn {
        for roll1 in 1..=3 {
            for roll2 in 1..=3 {
                for roll3 in 1..=3 {
                    let newpos = (p1 + roll1 + roll2 + roll3 - 1) % 10 + 1;
                    match cache.get(&(false, newpos, p2, p1s + newpos, p2s)) {
                        Some((p1w, p2w)) => {
                            p1wins += p1w;
                            p2wins += p2w;
                        }
                        None => {
                            let (p1w, p2w) = play(false, newpos, p2, p1s + newpos, p2s, cache);
                            p1wins += p1w;
                            p2wins += p2w;
                        }
                    }
                }
            }
        }
    } else {
        for roll1 in 1..=3 {
            for roll2 in 1..=3 {
                for roll3 in 1..=3 {
                    let newpos = (p2 + roll1 + roll2 + roll3 - 1) % 10 + 1;
                    match cache.get(&(true, p1, newpos, p1s, p2s + newpos)) {
                        Some((p1w, p2w)) => {
                            p1wins += p1w;
                            p2wins += p2w;
                        }
                        None => {
                            let (p1w, p2w) = play(true, p1, newpos, p1s, p2s + newpos, cache);
                            p1wins += p1w;
                            p2wins += p2w;
                        }
                    }
                }
            }
        }
    }
    cache.insert((p1turn, p1, p2, p1s, p2s), (p1wins, p2wins));
    (p1wins, p2wins)
}


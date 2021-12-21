use std::collections::HashSet;

// example = target area: x=20..30, y=-10..-5
// input = target area: x=282..314, y=-80..-45
fn main() {
    let xmin = 20;
    let xmax = 30;
    let ymin = -10;
    let ymax = -5;
    let xmin = 282;
    let xmax = 314;
    let ymin = -80;
    let ymax = -45;
    //let mut winner = 0;
    let mut vels: HashSet<(i64, i64)> = HashSet::new();
    for vy in -1000..1000 {
        for vx in -1000..1000 {
            let mut xvel = vx;
            let mut yvel = vy;
            let mut pos: (i64, i64) = (0, 0);
            for _t in 0..10000 {
                pos.0 += xvel;
                pos.1 += yvel;
                if xvel > 0 {
                    xvel -= 1
                } else if xvel < 0 {
                    xvel += 1
                }
                yvel -= 1;
                if pos.0 > xmax || (xvel == 0 && pos.0 < xmin) {
                    break;
                }
                if pos.1 < ymin {
                    break;
                }
                if pos.0 >= xmin && pos.0 <= xmax && pos.1 >= ymin && pos.1 <= ymax {
                    println!("winner at {}, {}, {:?}", vx, vy, pos);
                    //winner = maxy;
                    vels.insert((vx, vy));
                    break;
                }
            }
        }
    }
    println!("Hello, world!, {}", vels.len() * 2);
}


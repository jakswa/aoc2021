fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input
        .lines()
        .rev()
        .map(|i| {
            i.chars()
                .map(|i| i as u8 as i64)
                .map(|i| match i {
                    91 => -91,
                    93 => -93,
                    44 => -44,
                    o => o - ('0' as u8 as i64),
                })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let mut res: Vec<i64> = vec![];
    for l in lines.iter() {
        for l2 in lines.iter() {
            res.push(product(&add(l, l2)));
        }
    }
    println!("done: {:?}", res.iter().max());
}

fn add(l: &Vec<i64>, r: &Vec<i64>) -> Vec<i64> {
    //let mut comb = "[".to_string() + l + "," + r + "]";
    let mut comb = vec![-91];
    comb.extend_from_slice(l);
    comb.push(-44);
    comb.extend_from_slice(r);
    comb.push(-93);
    loop {
        match reduce(&comb) {
            None => break,
            Some(r) => comb = r,
        }
        //println!("  _B comb: {:?}", pp(&comb));
    }
    //println!("  _A comb: {:?}", pp(&comb));
    comb
}

fn reduce(s: &Vec<i64>) -> Option<Vec<i64>> {
    let mut depth = 0;
    let mut stack: Vec<i64> = vec![];
    let mut last: i64 = -5983;
    let mut to_split: Option<usize> = None;
    for (ind, c) in s.iter().enumerate() {
        if *c == -44 {
            // ','
            continue;
        }
        match c {
            -91 => {
                // '['
                depth += 1
            }
            -93 => {
                // ']'
                depth -= 1;
                if depth >= 4 && is_num(*stack.last().unwrap()) && is_num(last) {
                    return Some(explode(&s, ind));
                }
            }
            i => {
                if *i > 9 && to_split == None {
                    to_split = Some(ind);
                }
            }
        }
        if last != -5983 {
            stack.push(last);
        }
        last = *c;
    }
    if let Some(split_ind) = to_split {
        return Some(split(&s, split_ind));
    }
    None
}

fn split(s: &Vec<i64>, ind: usize) -> Vec<i64> {
    //print!("<SPLIT>");
    let val = s.get(ind).unwrap();
    let (sleft, sright) = s.split_at(ind);
    let mut ret = vec![];
    ret.extend_from_slice(sleft);
    ret.push(-91);
    ret.push(val / 2);
    ret.push(-44);

    let mut rval = val / 2;
    if val % 2 == 1 {
        rval += 1;
    }
    ret.push(rval);
    ret.push(-93);
    ret.extend_from_slice(&sright[1..]);
    //println!(" ---|-|--- SPLIT: {:?}", pp(&ret[..]));
    ret
}
fn product(s: &Vec<i64>) -> i64 {
    let mut s = s.clone();
    loop {
        if s.len() == 1 {
            return *s.last().unwrap();
        }
        let mut stack: Vec<i64> = vec![];
        for i in s.iter() {
            match i {
                -91 => stack.push(-91),
                -93 => {
                    let r = stack.pop().unwrap() * 2;
                    let l = stack.pop().unwrap() * 3;
                    stack.pop();
                    stack.push(l + r);
                }
                -44 => {}
                o => stack.push(*o),
            }
        }
        s = stack;
    }
}

// ind = index of the closing ]
fn explode(s: &Vec<i64>, ind: usize) -> Vec<i64> {
    //print!("|OploO|");
    //println!(" ---<>--- EXPLOD: {:?}", pp(&s[ind - 4..ind + 1]));
    let r = s[ind - 1..ind].iter().next().unwrap();
    let l = s[ind - 3..ind - 2].iter().next().unwrap();
    let (lchunk, _) = s.split_at(ind - 4);
    let (_, rchunk) = s.split_at(ind + 1);

    let mut lchunkv = vec![];
    lchunkv.extend_from_slice(lchunk);
    lchunkv.reverse();
    match lchunkv.iter().position(|i| is_num(*i)) {
        None => {}
        Some(o) => lchunkv[o] = lchunkv[o] + l,
    }
    lchunkv.reverse();

    let mut rchunkv = vec![];
    rchunkv.extend_from_slice(rchunk);
    match rchunkv.iter().position(|i| is_num(*i)) {
        None => {}
        Some(o) => rchunkv[o] = rchunkv[o] + r,
    }

    lchunkv.push(0);
    lchunkv.append(&mut rchunkv);
    //println!(" ---<><>--- EXPLOD: {:?}", pp(&lchunkv[..]));
    lchunkv
}

// going to [ to hopefully get all added-past-9 cases
fn is_num(s: i64) -> bool {
    s >= 0
    //s != ('[' as u8) && s != (']' as u8) && s != (',' as u8)
}

fn pp(s: &[i64]) -> String {
    let strs = s
        .iter()
        .map(|i| match i {
            -91 => "[".to_string(),
            -93 => "]".to_string(),
            -44 => ",".to_string(),
            o => o.to_string(),
        })
        .collect::<Vec<String>>();
    strs.join("")
}


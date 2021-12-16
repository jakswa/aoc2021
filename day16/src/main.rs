fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let binary: String = input
        .trim()
        .chars()
        .map(|i| format!("{:0>4b}", i.to_digit(16).unwrap()))
        .collect::<Vec<String>>()
        .join("");

    println!("oof: {:?}", scan_packet(&binary));
}

fn scan_packet(s: &str) -> (usize, usize) {
    let packet_type = usize::from_str_radix(&s[3..6], 2).unwrap();
    match packet_type {
        4 => scan_literal(&s),
        _ => scan_operator(&s),
    }
}

// returns length of the literal
fn scan_literal(s: &str) -> (usize, usize) {
    let mut curr = 6;
    let mut nums: Vec<&str> = vec![];
    loop {
        if &s[curr..curr + 1] == "0" {
            break;
        }
        nums.push(&s[curr + 1..curr + 5]);
        curr += 5;
    }
    nums.push(&s[curr + 1..curr + 5]);
    let num = usize::from_str_radix(&nums.join(""), 2).unwrap();
    (num, curr + 5)
}
fn scan_operator(s: &str) -> (usize, usize) {
    match &s[6..7] {
        "0" => scan_operator_chunk(&s, usize::from_str_radix(&s[7..22], 2).unwrap()),
        _ => scan_operator_packets(&s, usize::from_str_radix(&s[7..18], 2).unwrap()),
    }
}

fn scan_operator_chunk(s: &str, chunk_len: usize) -> (usize, usize) {
    let packet_type = usize::from_str_radix(&s[3..6], 2).unwrap();
    let mut curr = 0;
    let mut vals: Vec<usize> = vec![];
    loop {
        if curr >= chunk_len {
            break;
        }
        let (val, chunk_len) = scan_packet(&s[22 + curr..]);
        curr += chunk_len;
        vals.push(val);
    }
    (process_val(packet_type, vals), curr + 22)
}

fn scan_operator_packets(s: &str, packet_count: usize) -> (usize, usize) {
    let packet_type = usize::from_str_radix(&s[3..6], 2).unwrap();
    let mut packets = 0;
    let mut curr = 0;
    let mut vals: Vec<usize> = vec![];
    loop {
        if packets >= packet_count {
            break;
        }
        let (val, chunk_len) = scan_packet(&s[18 + curr..]);
        packets += 1;
        curr += chunk_len;
        vals.push(val);
    }

    (process_val(packet_type, vals), curr + 18)
}

fn process_val(packet_type: usize, vals: Vec<usize>) -> usize {
    match packet_type {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => match vals[0] > vals[1] {
            true => 1,
            false => 0,
        },
        6 => match vals[0] < vals[1] {
            true => 1,
            false => 0,
        },
        7 => match vals[0] == vals[1] {
            true => 1,
            false => 0,
        },
        _ => 0,
    }
}


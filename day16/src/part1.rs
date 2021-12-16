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
    let version_sum = usize::from_str_radix(&s[0..3], 2).unwrap();
    let mut curr = 6;
    loop {
        if &s[curr..curr + 1] == "0" {
            curr += 5;
            break;
        }
        curr += 5;
    }
    (version_sum, curr)
}
fn scan_operator(s: &str) -> (usize, usize) {
    match &s[6..7] {
        "0" => scan_operator_chunk(&s, usize::from_str_radix(&s[7..22], 2).unwrap()),
        _ => scan_operator_packets(&s, usize::from_str_radix(&s[7..18], 2).unwrap()),
    }
}

fn scan_operator_chunk(s: &str, chunk_len: usize) -> (usize, usize) {
    let mut version_sum = usize::from_str_radix(&s[0..3], 2).unwrap();
    let mut curr = 0;
    loop {
        if curr >= chunk_len {
            break;
        }
        let (chunk_sum, chunk_len) = scan_packet(&s[22 + curr..]);
        curr += chunk_len;
        version_sum += chunk_sum;
    }
    (version_sum, curr + 22)
}

fn scan_operator_packets(s: &str, packet_count: usize) -> (usize, usize) {
    let mut version_sum = usize::from_str_radix(&s[0..3], 2).unwrap();
    let mut packets = 0;
    let mut curr = 0;
    loop {
        if packets >= packet_count {
            break;
        }
        let (chunk_sum, chunk_len) = scan_packet(&s[18 + curr..]);
        packets += 1;
        curr += chunk_len;
        version_sum += chunk_sum;
    }

    (version_sum, curr + 18)
}


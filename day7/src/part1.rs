fn main() {
    let mut input = std::fs::read_to_string("input.txt")
        .unwrap().split(&[',','\n'][..]).filter_map(|i| i.parse::<i64>().ok())
        .collect::<Vec<i64>>();
        
    input.sort();
    println!("hmm {}, {:?}", input.len(), input);

    let best = (input[0]..*input.last().unwrap()).fold(i64::MAX, |accum, center| {
        let sum: i64 = input.iter().map(|i| (center - *i).abs()).sum();
        if accum < sum {
            return accum;
        }
        sum
    });

    println!("used: {}", best);
}

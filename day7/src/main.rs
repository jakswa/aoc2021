fn main() {
    //let mut input: Vec<i64> = vec![16,1,2,0,4,2,7,1,2,14];
    let mut input = std::fs::read_to_string("input.txt")
        .unwrap().split(&[',','\n'][..]).filter_map(|i| i.parse::<i64>().ok())
        .collect::<Vec<i64>>();
        
    input.sort();
    println!("hmm {}, {:?}", input.len(), input);

    let best = (input[0]..=*input.last().unwrap()).fold(i64::MAX, |accum, center| {
        let sum: i64 = input.iter().map(|i| (0..=(center - *i).abs()).sum::<i64>()).sum();
        if accum < sum {
            return accum;
        }
        sum
    });

    println!("used: {}", best);
}

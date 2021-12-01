fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let nums: Vec<i64> = input.lines().filter_map(|i| i.parse().ok()).collect();
    // part1:
    //println!("res: {}", nums.windows(2).filter(|i| i[0] < i[1]).count());
    println!(
        "res: {}",
        nums.windows(4)
            .filter(|i| i[0..3].iter().sum::<i64>() < i[1..4].iter().sum())
            .count()
    );
    Ok(())
}


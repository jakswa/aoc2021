fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    //let input = std::fs::read_to_string("example.txt")?;
    let line_count = input.lines().count() as i64;
    let line_len = input.lines().next().unwrap().len();
    let mut zeros: Vec<i64> = vec![0; line_len];
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(ind, c)| {
            if c == '0' {
                zeros[ind] += 1;
            }
        })
    });

    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    zeros.iter().for_each(|zero_cnt| {
        gamma <<= 1;
        epsilon <<= 1;
        if line_count - zero_cnt < *zero_cnt {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    });
    println!("done: {}, {}, {}", gamma, epsilon, gamma * epsilon);
    Ok(())
}


fn main() {
    let count: usize = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|i| {
            i.split("|")
                .last()
                .unwrap()
                .split_whitespace()
                .filter(|i| i.len() == 2 || i.len() == 3 || i.len() == 4 || i.len() == 7)
                .count()
        })
        .sum();

    println!("WOO {}", count);
}


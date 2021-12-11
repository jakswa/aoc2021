fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let openers: Vec<char> = vec!['{', '(', '<', '['];
    let mut stacks: Vec<Vec<char>> = vec![];

    input.lines().for_each(|line| {
        let mut stack: Vec<char> = vec![];
        for linechar in line.chars() {
            if openers.contains(&linechar) {
                stack.push(linechar);
            } else if let Some(stackchar) = stack.pop() {
                if linechar != closer(&stackchar) {
                    return; // violator
                }
            }
        }
        stacks.push(stack);
    });

    let mut scores = stacks
        .iter()
        .map(|stack| points(stack))
        .collect::<Vec<u64>>();
    scores.sort();

    println!("scores: {:?}", scores[scores.len() / 2]);
}

fn closer(opener: &char) -> char {
    match opener {
        '{' => '}',
        '[' => ']',
        '<' => '>',
        '(' => ')',
        _ => '_',
    }
}

fn points(viol: &Vec<char>) -> u64 {
    let mut score = 0;
    viol.iter()
        .rev()
        .map(|i| match closer(i) {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        })
        .for_each(|pnt| {
            score = (score * 5) + pnt;
        });
    score
}


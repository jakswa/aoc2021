fn main() {
    let count: usize = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|i| solve_line(i))
        .sum();

    println!("WOO {}", count);
}

fn solve_line(line: &str) -> usize {
    let mut split = line.split("|");
    let left = split
        .next()
        .unwrap()
        .split_whitespace()
        .map(|i| {
            let mut s = i.chars().collect::<Vec<char>>();
            s.sort();
            s
        })
        .collect::<Vec<Vec<char>>>();
    let right = split.next().unwrap().split_whitespace().map(|i| {
        let mut s = i.chars().collect::<Vec<char>>();
        s.sort();
        s
    });

    let one = left.iter().find(|i| i.len() == 2).unwrap();
    let four = left.iter().find(|i| i.len() == 4).unwrap();
    let seven = left.iter().find(|i| i.len() == 3).unwrap();
    let eight = left.iter().find(|i| i.len() == 7).unwrap();

    // only the two digit has a unique char in it
    let mut charcnts: [usize; 7] = [0; 7];
    left.iter().for_each(|i| {
        i.iter()
            .for_each(|c| charcnts[(*c as usize) - ('a' as usize)] += 1);
    });
    let find = charcnts
        .iter()
        .enumerate()
        .find(|indi| *indi.1 == 9)
        .unwrap();
    let f = ((find.0 + ('a' as usize)) as u8) as char;
    let two = left.iter().find(|i| !i.contains(&f)).unwrap();

    // clobbering random conditions to find the others
    let three = left
        .iter()
        .filter(|c| c.len() == 5)
        .find(|i| i.iter().filter(|c| two.contains(c)).count() == 4)
        .unwrap();
    let five = left
        .iter()
        .find(|i| i.len() == 5 && *i != three && *i != two)
        .unwrap();
    let nine = left
        .iter()
        .filter(|i| i.len() == 6)
        .find(|i| four.iter().all(|b| i.contains(b)))
        .unwrap();
    let six = left
        .iter()
        .filter(|i| i.len() == 6 && *i != nine)
        .find(|i| five.iter().all(|j| i.contains(j)))
        .unwrap();
    let zero = left
        .iter()
        .find(|i| i.len() == 6 && *i != nine && *i != six)
        .unwrap();

    right
        .map(|i| {
            if i == *zero {
                '0'
            } else if i == *one {
                '1'
            } else if i == *two {
                '2'
            } else if i == *three {
                '3'
            } else if i == *four {
                '4'
            } else if i == *five {
                '5'
            } else if i == *six {
                '6'
            } else if i == *seven {
                '7'
            } else if i == *eight {
                '8'
            } else {
                '9'
            }
        })
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}


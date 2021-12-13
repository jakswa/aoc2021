use std::collections::HashMap;

fn main() {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let input = std::fs::read_to_string("input.txt").unwrap();
    input
        .lines()
        .map(|i| i.split('-').collect::<Vec<&str>>())
        .for_each(|vec| {
            let l = graph.entry(vec[0]).or_insert(vec![]);
            l.push(vec[1]);
            let r = graph.entry(vec[1]).or_insert(vec![]);
            r.push(vec[0]);
        });

    let mut visits: Vec<Vec<&str>> = vec![vec!["start"]];
    let mut paths: u64 = 0;
    //println!("graph: {:?}", graph);

    loop {
        if visits.len() == 0 {
            break;
        }
        let visit = visits.pop().unwrap();
        let curr = visit.last().unwrap();
        graph.entry(curr).or_default().iter().for_each(|sibling| {
            if sibling == &"end" {
                paths += 1;
                //println!("PATH: {:?}", visit);
                return;
            }
            let first = visit.first().unwrap();
            let small_cave = small_cave(sibling);
            if sibling == &"start"
                || (first == &"DING"
                    && small_cave
                    && visit.iter().filter(|i| i == &sibling).count() >= 1)
            {
                return;
            }
            let mut next_visit = visit.clone();
            if first != &"DING" && small_cave && next_visit.contains(sibling) {
                next_visit.insert(0, "DING");
            }
            next_visit.push(sibling);
            visits.push(next_visit);
        });
    }

    println!("omg done: {}", paths);
}

fn small_cave(s: &str) -> bool {
    let char = s.chars().next().unwrap() as u8;
    char >= ('a' as u8) && char <= ('z' as u8)
}


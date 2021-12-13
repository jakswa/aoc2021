use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sections = input.split("\n\n");
    let mut map: HashSet<(u16, u16)> = HashSet::new();
    sections.next().unwrap().lines().for_each(|line| {
        let mut spl = line.split(",");
        map.insert((
            spl.next().unwrap().parse::<u16>().unwrap(),
            spl.next().unwrap().parse::<u16>().unwrap(),
        ));
    });

    let mut last_splits = (u16::MAX, u16::MAX);

    let splits = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let split = line[11..].split('=').collect::<Vec<&str>>();
            (split[0], split[1].parse::<u16>().unwrap())
        })
        .collect::<Vec<(&str, u16)>>();

    let mut new_dots: Vec<(u16, u16)> = vec![];
    let mut removes: Vec<(u16, u16)> = vec![];
    splits.iter().for_each(|(on, xy)| {
        map.iter().for_each(|(x, y)| {
            if on == &"y" && y > xy {
                last_splits.1 = *xy;
                new_dots.push((*x, xy - (y - xy)));
                removes.push((*x, *y));
            } else if on == &"x" && x > xy {
                last_splits.0 = *xy;
                new_dots.push((xy - (x - xy), *y));
                removes.push((*x, *y));
            } else if (on == &"y" && y == xy) || (on == &"x" && x == xy) {
                removes.push((*x, *y));
            }
        });

        new_dots.drain(..).for_each(|dot| {
            map.insert(dot);
        });
        removes.drain(..).for_each(|dot| {
            map.remove(&dot);
        });
    });

    for y in 0..last_splits.1 {
        for x in 0..last_splits.0 {
            match map.contains(&(x, y)) {
                true => print!("#"),
                false => print!("."),
            }
        }
        print!("\n");
    }
}


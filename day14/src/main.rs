use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let template: Vec<u8> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>();

    let mut formulas = HashMap::new();
    input.lines().skip(2).for_each(|line| {
        let strs: Vec<&str> = line.split(" -> ").collect::<Vec<&str>>();
        let mut key = strs[0].chars();
        formulas.insert(
            (key.next().unwrap() as u8, key.next().unwrap() as u8),
            strs[1].chars().next().unwrap() as u8,
        );
    });

    let mut counts: HashMap<u8, u64> = HashMap::new();
    let mut cache: HashMap<(u8, u8, u8), HashMap<u8, u64>> = HashMap::new();
    for chars in template.windows(2) {
        for (k, v) in dig(&formulas, chars[0], chars[1], 40, &mut cache).iter() {
            *counts.entry(*k).or_insert(0) += v;
        }
    }
    for chr in template.iter() {
        *counts.entry(*chr).or_insert(0) += 1;
    }

    let mut to_sort = counts
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect::<Vec<(u8, u64)>>();
    to_sort.sort_by_key(|i| i.1);
    println!("OOF: {}", to_sort.last().unwrap().1 - to_sort[0].1)
}

fn dig(
    formulas: &HashMap<(u8, u8), u8>,
    l: u8,
    r: u8,
    cnt: u8,
    cache: &mut HashMap<(u8, u8, u8), HashMap<u8, u64>>,
) -> HashMap<u8, u64> {
    if cnt == 0 {
        return HashMap::new();
    }
    if cnt > 4 {
        if let Some(cache_hit) = cache.get(&(l, r, cnt)) {
            return cache_hit.clone();
        }
    }

    let sub = *formulas.get(&(l, r)).unwrap();
    let mut cnts = dig(formulas, l, sub, cnt - 1, cache);
    *cnts.entry(sub).or_insert(0) += 1;
    for (k, v) in dig(formulas, sub, r, cnt - 1, cache).iter() {
        *cnts.entry(*k).or_insert(0) += v;
    }
    if cnt > 4 {
        cache.entry((l, r, cnt)).or_insert(cnts.clone());
    }
    cnts
}


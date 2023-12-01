use std::collections::HashMap;

fn main() {
    let (polymer, rules) = include_str!("../input").split_once("\n\n").unwrap();
    let rules: HashMap<_, _> = rules.lines().map(|rule| {
        let (left, right) = rule.split_once(" -> ").unwrap();
        let left = (left.as_bytes()[0], left.as_bytes()[1]);
        let right: u8 = right.as_bytes()[0];
        (left, right)

    }).collect();

    let mut pairs: HashMap<(u8, u8), u64> = HashMap::new();
    polymer.as_bytes().windows(2).for_each(|w| {
        let e = pairs.entry((w[0], w[1])).or_insert(0);
        *e += 1;
    });
    let mut counts: HashMap<u8, u64> = HashMap::new();
    polymer.as_bytes().iter().for_each(|b| {
        let e = counts.entry(*b).or_insert(0);
        *e += 1;
    });

    let steps_to_take = 40;
    for _ in 1..=steps_to_take {
        let mut new_pairs: HashMap<(u8, u8), u64> = pairs.clone();
        for (k, v) in &pairs {
            let new_ch = rules[k];
            *counts.entry(new_ch).or_insert(0) += v;
            *new_pairs.entry((k.0, new_ch)).or_insert(0) += v;
            *new_pairs.entry((new_ch, k.1)).or_insert(0) += v;
            *new_pairs.entry(*k).or_insert(0) -= v;
        }
        pairs = new_pairs;
    }

    let (min, max) = counts.iter().fold((u64::MAX, 0), |(min, max), (_, v)| {
        (*v.min(&min), *v.max(&max))
    });
    println!("{max} - {min} = {}", max - min);

}

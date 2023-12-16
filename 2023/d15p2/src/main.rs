use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<usize, Vec<(&str, &str)>> = HashMap::new();
    include_str!("../input").trim().split(",").for_each(|s| {
        if let Some(parts) = s.split_once('=') {
            let key = hash(parts.0);
            let bucket = hm.entry(key).or_insert(Vec::new());
            if let Some(p) = bucket.iter().position(|e| e.0 == parts.0) {
                bucket[p] = parts;
            } else {
                bucket.push(parts);
            }
        } else if let Some(parts) = s.split_once('-') {
            let key = hash(parts.0);
            let bucket = hm.entry(key).or_insert(Vec::new());
            if let Some(p) = bucket.iter().position(|e| e.0 == parts.0) {
                bucket.remove(p);
            }
        }
    });

    let sum: usize = hm
        .iter()
        .map(|(b, lenses)| {
            (b + 1)
                * lenses
                    .iter()
                    .enumerate()
                    .map(|(i, (_, n))| (i + 1) * n.parse::<usize>().unwrap())
                    .sum::<usize>()
        })
        .sum();

    println!("res: {sum}");
}

fn hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0, |a, b| {
        let s = a + *b as usize;
        (s * 17) % 256
    })
}

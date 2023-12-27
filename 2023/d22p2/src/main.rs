use std::collections::{HashMap, HashSet};

fn main() {
    let mut pieces: Vec<((usize, usize), (usize, usize), (usize, usize))> =
        include_str!("../input")
            .trim()
            .lines()
            .map(|line| {
                let (from, to) = line.split_once("~").unwrap();
                let v = from
                    .split(",")
                    .zip(to.split(","))
                    .map(|(a, b)| {
                        let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
                        (a.min(b), a.max(b))
                    })
                    .collect::<Vec<_>>();
                (v[0], v[1], v[2])
            })
            .collect();

    pieces.sort_by(|a, b| a.2 .0.cmp(&b.2 .0));

    // x-y coord -> (piece_id, height)
    let mut highest: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut supported_bys: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (i, p) in pieces.iter().enumerate() {
        let i = i + 1;
        let mut height = 1;
        for x in p.0 .0..=p.0 .1 {
            for y in p.1 .0..=p.1 .1 {
                if let Some(x) = highest.get(&(x, y)) {
                    height = height.max(x.1 + 1);
                }
            }
        }

        // OpenSCAD:
        // println!("translate([{}.05, {}.05, {}.05]) scale([0.9, 0.9, 0.9]) cube([{}, {}, {}]);",
        //     p.0.0, p.1.0, height,
        //     p.0.1-p.0.0+1, p.1.1-p.1.0+1, p.2.1-p.2.0+1,
        //     );

        let mut supported_by = HashSet::new();
        for x in p.0 .0..=p.0 .1 {
            for y in p.1 .0..=p.1 .1 {
                if let Some(x) = highest.get(&(x, y)) {
                    if x.1 == height - 1 {
                        supported_by.insert(x.0);
                    }
                }
                highest.insert((x, y), (i, height + p.2 .1 - p.2 .0));
            }
        }
        supported_bys.insert(i, supported_by);
    }

    let mut sum = 0;
    for i in 1..=pieces.len() {
        let mut removed = HashSet::from([i]);
        let mut changed = true;
        while changed {
            changed = false;

            for j in 1..=pieces.len() {
                let supports_of_j = supported_bys.get(&j).unwrap();
                if !removed.contains(&j) &&
                    supports_of_j.len() > 0 &&
                    supports_of_j.iter().all(|s| removed.contains(s)) {
                    removed.insert(j);
                    changed = true;
                }
            }
        }
        // println!("removing {} removes {:?}", i, removed);
        sum += removed.len()-1;
    }

    println!("res: {sum}");
}

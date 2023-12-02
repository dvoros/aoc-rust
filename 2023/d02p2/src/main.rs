use std::collections::HashMap;

fn main() {
    let res: usize = include_str!("../input").trim().lines().map(
        |line| {
            let r = line.trim().split_once(":").unwrap().1;
            let mut mins = HashMap::new();
            for game in r.split(";") {
                for part in game.split(",") {
                    let (n, c) = part.trim().split_once(" ").unwrap();
                    let n = n.trim().parse::<usize>().unwrap();
                    let e = mins.entry(c).or_insert(usize::MIN);
                    *e = n.max(*e);
                }
            }
            let mut prod = 1;
            for (_, v) in mins {
                prod *= v;
            }
            prod
        }
    ).sum();

    println!("{res}")
}

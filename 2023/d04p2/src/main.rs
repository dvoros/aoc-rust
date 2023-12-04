fn main() {
    let mut copies = Vec::new();
    let mut scores = Vec::new();
    include_str!("../input").trim().lines().for_each(
        |line| {
            let line = line.split_once(":").unwrap().1;
            let (winning, all) = line.split_once("|").unwrap();
            let winning: Vec<_> = winning.split_whitespace().collect();
            
            copies.push(1);
            scores.push(all.split_whitespace().filter(|n| winning.contains(n)).count());
        }
    );

    for i in 0..scores.len() {
        for j in 1..=scores[i] {
            copies[i+j] += copies[i]
        }
    }

    let res: usize = copies.iter().sum();
    println!("res: {res}")
}

pub fn main() {
    let res: usize = include_str!("../input").trim().lines().map(
        |line| {
            let line = line.split_once(":").unwrap().1;
            let (winning, all) = line.split_once("|").unwrap();
            let winning: Vec<_> = winning.split_whitespace().collect();
            let mut count = all.split_whitespace().filter(|n| winning.contains(n)).count();
            if count > 0 {
                count -= 1;
                return (2 as usize).pow(count as u32);
            } else {
                return 0;
            }
        }
    ).sum();
    println!("res: {res}");
}

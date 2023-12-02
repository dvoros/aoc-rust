fn main() {
    let res: usize = include_str!("../input").trim().lines().map(
        |line| {
            let (l, r) = line.trim().split_once(":").unwrap();
            let game_num: usize = l.split_once(" ").unwrap().1.parse().unwrap();
            for game in r.split(";") {
                for part in game.split(",") {
                    let (n, c) = part.trim().split_once(" ").unwrap();
                    let n = n.trim().parse::<usize>().unwrap();
                    if c == "red" && n > 12 ||
                        c == "green" && n > 13 ||
                        c == "blue" && n > 14 {
                        return 0
                    }
                }
            }
            game_num
        }
    ).sum();

    println!("{res}")
}

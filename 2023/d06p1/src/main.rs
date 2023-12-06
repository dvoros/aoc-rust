fn main() {
    let mut lines = include_str!("../input").trim().lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });
    let (times, distances) = (lines.next().unwrap(), lines.next().unwrap());

    let res = (0..times.len())
        .map(|race| {
            let (t, d) = (times[race], distances[race]);
            (0..=t).map(|n| (t - n) * n).filter(|&n| n > d).count()
        })
        .product::<usize>();

    println!("res: {res}");
}

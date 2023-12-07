pub fn main() {
    let mut parts = include_str!("../input").split("\n\n");
    let seeds: Vec<_> = parts
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let conversions: Vec<_> = parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let result = conversions
        .iter()
        .fold(seeds, |input, conversion| {
            input
                .iter()
                .map(|&i| {
                    let rule = conversion
                        .iter()
                        .filter(|&rule| i >= rule[1] && i <= rule[1] + rule[2])
                        .next();
                    match rule {
                        Some(rule) => i + rule[0] - rule[1],
                        None => i,
                    }
                })
                .collect()
        })
        .into_iter()
        .min()
        .unwrap();
    println!("res: {result}");
}

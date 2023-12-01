use std::collections::HashMap;

fn main() {
    let sum: u32 = include_str!("../input").trim().lines().map(
        |line| {
            let nums = parse_nums_from_line(line);
            // println!("{:?}", nums);
            nums[0] * 10 + nums[nums.len()-1]
        }
    ).sum();
    println!("{sum}")
}

fn parse_nums_from_line(line: &str) -> Vec<u32> {
    let nums = HashMap::from([
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5),
        ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ]);
    let mut res = Vec::new();
    for i in 0..line.len() {
        for n in &nums {
            if line[i..].starts_with(n.0) {
                res.push(*n.1);
            }
        }
    }
    res
}
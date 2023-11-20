use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

struct Point {
    found: bool,
    value: u8,
}

impl Point {
    fn new(value: u8) -> Point {
        Point {
            found: false,
            value,
        }
    }
}

pub fn main() {
    let mut closing = HashMap::new();
    closing.insert('[', ']');
    closing.insert('(', ')');
    closing.insert('{', '}');
    closing.insert('<', '>');
    let mut scores = HashMap::new();
    scores.insert(')', 1);
    scores.insert(']', 2);
    scores.insert('}', 3);
    scores.insert('>', 4);
    let mut all_scores = Vec::new();
    include_str!("../input").lines().for_each(|line| {
        // println!("{:?}", line);
        let mut stack = Vec::new();
        let mut valid = true;
        for ch in line.chars() {
            match closing.get(&ch) {
                Some(pair) => {
                    // Opening
                    stack.push(pair);
                }
                None => match stack.pop() {
                    // Closing
                    Some(&last) => {
                        if last != ch {
                            // println!("illegal, doesn't match: {ch} != {last}");
                            valid = false;
                            break;
                        }
                    }
                    None => {
                        // println!("illegal, nothing's open: {ch}");
                        valid = false;
                        break;
                    }
                },
            }
        }
        if valid {
            let mut score: u64 = 0;
            stack.reverse();
            for s in stack {
                score *= 5;
                score += scores[s];
            }
            // println!("score: {score}");
            all_scores.push(score);
        }
    });
    let count = all_scores.len();
    let (_, final_score, _) = all_scores.select_nth_unstable(count / 2);
    println!("final score is {final_score}")
}

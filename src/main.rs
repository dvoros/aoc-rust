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
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('}', 1197);
    scores.insert('>', 25137);
    let mut syntax_error_score = 0;
    include_str!("../input").lines().for_each(|line| {
        // println!("{:?}", line);
        let mut stack = Vec::new();
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
                            syntax_error_score += scores[&ch];
                            break;
                        }
                    }
                    None => {
                        // println!("illegal, nothing's open: {ch}");
                        syntax_error_score += scores[&ch];
                        break;
                    }
                },
            }
        }
    });
    println!("final score is {syntax_error_score}")
}

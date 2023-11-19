use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn main() {
    let sum: usize = parse_lines(include_str!("../input"))
        .unwrap()
        .1
        .iter()
        .map(|line| solve_line(&line.0, &line.1))
        .sum();
    println!("number of matching: {sum}");
}

fn solve_line(left: &Vec<&str>, right: &Vec<&str>) -> usize {
    let mut solutions: HashMap<u8, &str> = HashMap::new();
    let mut lookup = HashMap::new();
    // 1, 4, 7, 8
    for dig in left {
        for p in [(1, 2), (4, 4), (7, 3), (8, 7)] {
            if dig.len() == p.1 {
                solutions.insert(p.0, dig);
                lookup.insert(*dig, p.0);
            }
        }
    }
    // 9
    for dig in left {
        if dig.len() == 6 && solutions[&4].chars().all(|c| dig.contains(c)) {
            solutions.insert(9, dig);
            lookup.insert(*dig, 9);
        }
    }
    // 0, 6
    for dig in left {
        if dig.len() == 6 && *dig != solutions[&9] {
            if solutions[&1].chars().all(|c| dig.contains(c)) {
                solutions.insert(0, dig);
                lookup.insert(*dig, 0);
            } else {
                solutions.insert(6, dig);
                lookup.insert(*dig, 6);
            }
        }
    }
    // 3
    for dig in left {
        if dig.len() == 5 && solutions[&1].chars().all(|c| dig.contains(c)) {
            solutions.insert(3, dig);
            lookup.insert(*dig, 3);
        }
    }
    // 2, 5
    for dig in left {
        if dig.len() != 5 || *dig == solutions[&3] {
            continue;
        }

        if dig.chars().all(|c| solutions[&9].contains(c)) {
            solutions.insert(5, dig);
            lookup.insert(*dig, 5);
        } else {
            solutions.insert(2, dig);
            lookup.insert(*dig, 2);
        }
    }

    let mut res = String::new();
    for dig in right {
        for (&k, v) in &lookup {
            if k.len() == dig.len() && k.chars().all(|c| dig.contains(c)) {
                res.push_str(&v.to_string());
            }
        }
    }

    res.parse().unwrap()
}

fn parse_side(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, alpha1)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(parse_side, tag(" | "), parse_side)(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(Vec<&str>, Vec<&str>)>> {
    separated_list1(char('\n'), parse_line)(input)
}

use std::{collections::HashMap, fmt::Display, ops::Add};

use crate::util::read_lines;
use regex::Regex;

pub mod util;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    fn from_i32s(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn points_between(&self, other: &Point) -> Vec<Point> {
        let mut res = Vec::new();
        let delta = Point::from_i32s((other.x - self.x).signum(), (other.y - self.y).signum());
        let mut p = self.clone();
        res.push(p);
        while p != *other {
            p = p + delta;
            res.push(p);
        }
        return res;
    }

    fn is_diagonal(&self, other: &Point) -> bool {
        return self.x != other.x && self.y != other.y;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

fn main() -> Result<(), std::io::Error> {
    let lines = read_lines("./input")?;
    let re = Regex::new("([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").unwrap();

    let mut board: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        let line = line?;
        let captures = re.captures(line.as_str()).unwrap();

        let nums: Vec<i32> = (1..5)
            .map(|x| captures.get(x).unwrap().as_str().parse::<i32>().unwrap())
            .collect();

        let from = Point::from_i32s(nums[0], nums[1]);
        let to = Point::from_i32s(nums[2], nums[3]);
        // println!("{from} -> {to}");

        let between = from.points_between(&to);
        for p in between {
            board.insert(p, board.get(&p).unwrap_or(&0) + 1);
            // println!(" - {}", p)
        }
    }

    let mut sum = 0;
    for (k, v) in board {
        if v > 1 {
            sum += 1;
        }
    }
    println!("answer is: {sum}");

    Ok(())
}

use atoi::atoi;
use nom::{combinator::map_opt, IResult};

pub fn main() {
    let mut nums = vec![];
    let (mut min, mut max) = (u32::MAX, 0);
    include_bytes!("../input")
        .split(|b| *b == b',')
        .map(|entry| num(entry).unwrap().1)
        .for_each(|x| {
            nums.push(x);
            if x > max {
                max = x;
            }
            if x < min {
                min = x
            }
        });
    println!("there are {} values between {min} and {max}", nums.len());
    let mut min_cost = u32::MAX;
    for n in min..=max {
        println!("checking {n}");
        let c = cost(&nums, n);
        if c < min_cost {
            min_cost = c;
        }
    }
    println!("min cost is {min_cost}");
}

fn cost(positions: &[u32], target: u32) -> u32 {
    let mut c = 0;
    for p in positions {
        if *p != target {
            c += (1..=p.abs_diff(target)).sum::<u32>();
        }
    }
    c
}

fn num(input: &[u8]) -> IResult<&[u8], u32> {
    map_opt(nom::character::complete::digit1, atoi)(input)
}

use atoi::atoi;
use nom::{combinator::map_opt, IResult};

pub fn main() {
    let mut fish = vec![];
    include_bytes!("../input")
        .split(|b| *b == b',')
        .map(|entry| num(entry).unwrap().1)
        .for_each(|x| fish.push(x));

    let rounds_target = 80;
    // println!("{:?}", fish);
    for round in 1..=rounds_target {
        let mut new_fish = vec![];
        for f in fish.iter_mut() {
            *f -= 1;
            if *f == -1 {
                *f = 6;
                new_fish.push(8);
            }
        }
        fish.append(&mut new_fish);
        println!("round {round} done")
        // println!("{:?}", fish);
    }
    println!("{}", fish.len());
}

fn num(input: &[u8]) -> IResult<&[u8], i32> {
    map_opt(nom::character::complete::digit1, atoi)(input)
}

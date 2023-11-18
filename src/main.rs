use atoi::atoi;
use nom::{combinator::map_opt, IResult};

pub fn main() {
    let mut fish: [u64; 9] = [0; 9];
    include_bytes!("../input")
        .split(|b| *b == b',')
        .map(|entry| num(entry).unwrap().1)
        .for_each(|x| fish[x] += 1);

    let rounds_target = 256;
    // println!("{:?}", fish);
    for round in 1..=rounds_target {
        let new_fish = fish[0];
        for i in 0..fish.len() - 1 {
            fish[i] = fish[i + 1];
        }
        fish[8] = new_fish;
        fish[6] += new_fish;
        println!("round {round} done")
        // println!("{:?}", fish);
    }
    let res: u64 = fish.iter().sum();
    println!("{}", res);
}

fn num(input: &[u8]) -> IResult<&[u8], usize> {
    map_opt(nom::character::complete::digit1, atoi)(input)
}

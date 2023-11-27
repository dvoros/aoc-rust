#![feature(extract_if)]

//
// This is from https://github.com/timvisee/advent-of-code-2021/blob/master/day03b/src/main.rs
//

const WIDTH: usize = 12;

pub fn main() {
    let nums = include_str!("../input")
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = (0..WIDTH)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            oxy.extract_if(|n| (*n & 1 << i > 0) != one).last();
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..WIDTH)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.extract_if(|n| (*n & 1 << i > 0) == one).last();
            co2.first().copied()
        })
        .last()
        .unwrap();

    println!("{}", oxy * co2);
}
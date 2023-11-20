use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn main() {
    let sum: usize = parse_lines(include_str!("../test_input"))
        .unwrap()
        .1
        .iter()
        .map(|line| solve_line(&line.0, &line.1))
        .sum();
    println!("number of matching: {sum}");
}

fn solve_line(left: &Vec<&str>, right: &Vec<&str>) -> usize {
    right
        .iter()
        .filter(|word| [2, 3, 4, 7].contains(&word.len()))
        .count()
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
